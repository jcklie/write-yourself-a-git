use std::path::Path;

use tempfile::TempDir;

use wyag;

#[test]
fn test_from_existing() {
    let tempdir = TempDir::new().expect("Could not create tempdir");
    let path = tempdir.path().join("test_repo");

    wyag::GitRepository::init(&path).expect("Failed to init repo");

    let repository = wyag::GitRepository::from_existing(&path).expect("Should have worked!");

    assert!(repository.is_valid().is_ok());
}

#[test]
fn test_opening_nonexistent_repo_should_fail() {
    let path = Path::new("nonexistent_repo");

    match wyag::GitRepository::from_existing(path) {
        Err(wyag::Error::DirectoryNotFound { .. }) => {}
        e => panic!("Expected 'RepositoryNotFound', but got '{:#?}'", e),
    }
}
