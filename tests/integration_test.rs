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

#[test]
fn test_cat_file_with_existing_blob() {
    let cwd = std::env::current_dir().unwrap();
    let path = wyag::find_repository(cwd).unwrap();

    let repository = wyag::GitRepository::from_existing(&path).unwrap();

    let hash = "3ffe1398195ef384e2edbfd29d05516a33299e43";

    let git_object = repository.read_object(hash).unwrap();

    if let wyag::GitObject::GitBlob { data } = git_object {
        let text = String::from_utf8(data).unwrap();
        assert_eq!(text, "Lorem ipsum dolor sit amet.");
    } else {
        panic!("Expected GitBlob");
    }
}
