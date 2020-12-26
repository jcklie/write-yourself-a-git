use std::fs;
use std::path::{Path, PathBuf};

use ini::Ini;

use snafu::{ensure, Backtrace, OptionExt, ResultExt, Snafu};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Repository at [{}] does not exist!", worktree.display() ))]
    RepositoryNotFound { worktree: PathBuf },

    #[snafu(display("Folder at [{}] already exists!", worktree.display() ))]
    RepositoryAlreadyExists { worktree: PathBuf },

    #[snafu(display("[{}] is not a directory!", path.display() ))]
    NotADirectory { path: PathBuf, backtrace: Backtrace },

    #[snafu(display("IoError [{}]: {}", msg, source))]
    IoError { msg: String, source: std::io::Error },

    #[snafu(display("Error when reading config file at [{}]: {}", config_file.display(), source))]
    OpenConfigError {
        config_file: PathBuf,
        source: ini::Error,
    },

    #[snafu(display("Error when reading config file at [{}]: {}", config_file.display(), msg))]
    ConfigError { config_file: PathBuf, msg: String },
}

type Result<T, E = Error> = std::result::Result<T, E>;

pub struct GitRepository {
    worktree: PathBuf,
    gitdir: PathBuf,
    config: Ini,
}

impl GitRepository {
    pub fn from_existing(path: &Path) -> Result<GitRepository> {
        let worktree = path.to_path_buf();
        let gitdir = path.clone().join(".git");

        ensure!(worktree.exists(), RepositoryNotFound { worktree });
        ensure!(worktree.is_dir(), NotADirectory { path: worktree });

        // Open config
        let config_file = gitdir.clone().join("config");
        let config = Ini::load_from_file(&config_file).context(OpenConfigError {
            config_file: config_file.clone(),
        })?;

        // Parse version information
        let section = config.section(Some("core")).context(ConfigError {
            config_file: config_file.clone(),
            msg: "core".to_string(),
        })?;

        let version_string = section
            .get("repositoryformatversion")
            .context(ConfigError {
                config_file: &config_file.clone(),
                msg: "core::repositoryformatversion".to_string(),
            })?;

        if version_string != "0" {
            let msg = format!("Unsupported 'repositoryformatversion' [{}]", version_string);
            return ConfigError { config_file, msg }.fail();
        }

        let repository = GitRepository {
            worktree,
            gitdir,
            config,
        };

        Ok(repository)
    }

    pub fn create_new(path: &Path) -> Result<GitRepository> {
        let worktree = path.to_path_buf();
        let gitdir = worktree.clone().join(".git");
        let config_file = gitdir.clone().join("config");

        ensure!(!worktree.exists(), RepositoryAlreadyExists { worktree });

        fs::create_dir(&worktree).context(IoError {
            msg: "Error while initializing repository".to_string(),
        })?;

        fs::create_dir(&gitdir).context(IoError {
            msg: "Error while creating .git folder".to_string(),
        })?;

        let mut config = Ini::new();

        config
            .with_section(Some("core"))
            .set("repositoryformatversion", "0");

        config.write_to_file(config_file).context(IoError {
            msg: "Error while writing config file".to_string(),
        })?;

        let repository = GitRepository {
            worktree,
            gitdir,
            config,
        };

        Ok(repository)
    }
}

impl std::fmt::Debug for GitRepository {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("GitRepository")
            .field("worktree", &self.worktree)
            .field("gitdir", &self.gitdir)
            .finish()
    }
}
