use std::fs;
use std::path::{Path, PathBuf};

use ini::Ini;

use snafu::{ensure, Backtrace, OptionExt, ResultExt, Snafu};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Directory at [{}] does not exist!", path.display() ))]
    DirectoryNotFound { path: PathBuf, backtrace: Backtrace },

    #[snafu(display("File at [{}] does not exist!", path.display() ))]
    FileNotFound { path: PathBuf, backtrace: Backtrace },

    #[snafu(display("Folder at [{}] already exists!", worktree.display() ))]
    RepositoryAlreadyExists {
        worktree: PathBuf,
        backtrace: Backtrace,
    },

    #[snafu(display("[{}] is not a directory!", path.display() ))]
    NotADirectory { path: PathBuf, backtrace: Backtrace },

    #[snafu(display("[{}] is not a afile!", path.display() ))]
    NotAFile { path: PathBuf, backtrace: Backtrace },

    #[snafu(display("IoError [{}]: {}", msg, source))]
    IoError {
        msg: String,
        source: std::io::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("Error when reading config file at [{}]: {}", config_file.display(), source))]
    OpenConfigError {
        config_file: PathBuf,
        source: ini::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("Error when reading config file: {}", msg))]
    ConfigError { msg: String, backtrace: Backtrace },
}

type Result<T, E = Error> = std::result::Result<T, E>;

pub struct GitRepository {
    worktree: PathBuf,
    gitdir: PathBuf,
}

trait GitObject {
    fn serialize();
    fn deserialize();
}

impl GitRepository {
    pub fn from_existing(path: &Path) -> Result<Self> {
        let worktree = path.to_path_buf();
        let gitdir = path.join(".git");

        let repository = GitRepository { worktree, gitdir };

        repository.is_valid()?;

        Ok(repository)
    }

    pub fn init(path: &Path) -> Result<Self> {
        let worktree = path.to_path_buf();
        let gitdir = worktree.join(".git");
        let config_file = gitdir.join("config");

        ensure!(!worktree.exists(), RepositoryAlreadyExists { worktree });

        fs::create_dir(&worktree).context(IoError {
            msg: "Error while initializing repository".to_string(),
        })?;

        create_dir(&worktree, &[".git"])?;
        create_dir(&gitdir, &["branches"])?;
        create_dir(&gitdir, &["objects"])?;
        create_dir(&gitdir, &["refs"])?;
        create_dir(&gitdir, &["refs", "tags"])?;
        create_dir(&gitdir, &["refs", "heads"])?;

        // Write description
        fs::write(
            repo_path(&gitdir, &["description"]),
            "Unnamed repository; edit this file 'description' to name the repository.\n",
        )
        .context(IoError {
            msg: "Error while writing description".to_string(),
        })?;

        // Write head
        fs::write(repo_path(&gitdir, &["HEAD"]), "ref: refs/heads/master\n").context(IoError {
            msg: "Error while writing HEAD".to_string(),
        })?;

        // Write config
        let mut config = Ini::new();

        config
            .with_section(Some("core"))
            .set("repositoryformatversion", "0")
            .set("filemode", "false")
            .set("bare", "false");

        config.write_to_file(config_file).context(IoError {
            msg: "Error while writing config file".to_string(),
        })?;

        let repository = GitRepository { worktree, gitdir };

        repository.is_valid()?;

        Ok(repository)
    }

    pub fn is_valid(&self) -> Result<()> {
        ensure_repo_dir_exists(&self.worktree, &[])?;
        ensure_repo_dir_exists(&self.gitdir, &["branches"])?;
        ensure_repo_dir_exists(&self.gitdir, &["objects"])?;
        ensure_repo_dir_exists(&self.gitdir, &["refs", "tags"])?;
        ensure_repo_dir_exists(&self.gitdir, &["refs", "heads"])?;

        ensure_repo_file_exists(&self.gitdir, &["description"])?;
        ensure_repo_file_exists(&self.gitdir, &["HEAD"])?;

        // Open config
        let config_file = repo_path(&self.gitdir, &["config"]);
        let config = Ini::load_from_file(&config_file).context(OpenConfigError {
            config_file: &config_file,
        })?;

        check_config_key(&config, "repositoryformatversion", "0")?;
        check_config_key(&config, "filemode", "false")?;
        check_config_key(&config, "bare", "false")?;

        Ok(())
    }
}

fn repo_path<T: Into<PathBuf>>(root: T, paths: &[&str]) -> PathBuf {
    let mut pathbuf = root.into();
    for path in paths {
        pathbuf.push(path);
    }
    pathbuf
}

fn create_dir<T: Into<PathBuf>>(root: T, paths: &[&str]) -> Result<()> {
    let path = repo_path(root, paths);

    fs::create_dir(&path).context(IoError {
        msg: "Error while creating directory".to_string(),
    })
}

fn ensure_repo_dir_exists<T: Into<PathBuf>>(root: T, paths: &[&str]) -> Result<()> {
    let path = repo_path(root, paths);
    ensure!(path.exists(), DirectoryNotFound { path });
    ensure!(path.is_dir(), NotADirectory { path });

    Ok(())
}

fn ensure_repo_file_exists<T: Into<PathBuf>>(root: T, paths: &[&str]) -> Result<()> {
    let path = repo_path(root, paths);
    ensure!(path.exists() && path.is_file(), FileNotFound { path });
    ensure!(path.is_file(), NotAFile { path });
    Ok(())
}

fn check_config_key(config: &ini::Ini, key: &str, expected_value: &str) -> Result<()> {
    // Parse version information
    let section = config.section(Some("core")).context(ConfigError {
        msg: "core".to_string(),
    })?;

    let value = section.get(key).context(ConfigError {
        msg: format!("core::{}", key),
    })?;

    if value != expected_value {
        let msg = format!(
            "Unexpected value for [{}], expected [{}], got [{}]",
            key, expected_value, value
        );
        return ConfigError { msg }.fail();
    }

    Ok(())
}

impl std::fmt::Debug for GitRepository {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("GitRepository")
            .field("worktree", &self.worktree)
            .field("gitdir", &self.gitdir)
            .finish()
    }
}
