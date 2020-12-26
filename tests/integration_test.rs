use std::env;
use std::ops::Deref;
use std::path::{Path, PathBuf};

use tempfile::TempDir;

use fs_extra;

use libwyag;

// https://andrewra.dev/2019/03/01/testing-in-rust-temporary-files/

struct Fixture {
    path: PathBuf,
    source: PathBuf,
    _tempdir: TempDir,
}

impl Fixture {
    fn blank(fixture_filename: &str) -> Self {
        // First, figure out the right file in `tests/fixtures/`:
        let root_dir = &env::var("CARGO_MANIFEST_DIR").expect("$CARGO_MANIFEST_DIR");
        let mut source = PathBuf::from(root_dir);
        source.push("tests");
        source.push("fixtures");
        source.push(&fixture_filename);

        // The "real" path of the file is going to be under a temporary directory:
        let tempdir = tempfile::tempdir().unwrap();
        let mut path = PathBuf::from(&tempdir.path());
        path.push(&fixture_filename);

        Fixture {
            _tempdir: tempdir,
            source,
            path,
        }
    }

    fn copy(fixture_filename: &str) -> Self {
        let fixture = Fixture::blank(fixture_filename);

        println!("{} -> {}", fixture.source.display(), fixture.path.display());

        let mut options = fs_extra::dir::CopyOptions::new();
        options.copy_inside = true;

        fs_extra::dir::copy(&fixture.source, &fixture.path, &options).expect("Copy should work!");

        fixture
    }
}

impl Deref for Fixture {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        self.path.deref()
    }
}

#[test]
fn test_from_existing() {
    let path = Fixture::copy("empty_repo");

    let _repository = libwyag::GitRepository::from_existing(&path).expect("Should have worked!");
}

#[test]
fn test_opening_nonexistent_repo_should_fail() {
    let path = Path::new("nonexistent_repo");

    match libwyag::GitRepository::from_existing(path) {
        Err(libwyag::Error::OpenConfigError { .. }) => {}
        e => panic!("Expected 'RepositoryNotFound', but got '{:#?}'", e),
    }
}
