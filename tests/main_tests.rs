use shasum::{check_file_path, compute_file_hash, create_progress_bar};
use sha1::Sha1;
use sha2::Sha256;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_check_file_path_existing_file() {
    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path();
    assert!(check_file_path(path.to_str().unwrap()).is_ok());
}

#[test]
fn test_check_file_path_non_existent_file() {
    let result = check_file_path("non_existent_file.txt");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::NotFound);
}

#[test]
fn test_check_file_path_directory() {
    let temp_dir = tempfile::tempdir().unwrap();
    let result = check_file_path(temp_dir.path().to_str().unwrap());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::InvalidInput);
}

#[test]
fn test_compute_file_hash_sha256() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Hello, world!").unwrap();
    let path = temp_file.path();

    let result = compute_file_hash::<Sha256>(path, false, true).unwrap();
    assert_eq!(result, "d9014c4624844aa5bac314773d6b689ad467fa4e1d1a50a1b8a99d5a95f72ff5");
}

#[test]
fn test_compute_file_hash_sha1() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Hello, world!").unwrap();
    let path = temp_file.path();

    let result = compute_file_hash::<Sha1>(path, false, true).unwrap();
    assert_eq!(result, "09fac8dbfd27bd9b4d23a00eb648aa751789536d");
}

#[test]
fn test_compute_file_hash_binary_mode() {
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(&[0, 1, 2, 3, 4]).unwrap();
    let path = temp_file.path();

    let result = compute_file_hash::<Sha256>(path, true, true).unwrap();
    assert_eq!(result, "08bb5e5d6eaac1049ede0893d30ed022b1a4d9b5b48db414871f51c9cb35283d");
}

#[test]
fn test_create_progress_bar() {
    let pb = create_progress_bar(100);
    assert_eq!(pb.length(), Some(100));
    assert_eq!(pb.position(), 0);
}