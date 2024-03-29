use assert_cmd::Command;
use std::fs::read_dir;
use tempfile::TempDir;

#[test]
fn update() {
    let mut command = Command::cargo_bin("alfred-gitignore").unwrap();

    let repository = TempDir::new().unwrap();
    command.arg("--repository").arg(repository.path());
    command.arg("--update");

    command.assert().success();

    let gitignore_count = read_dir(repository.path())
        .unwrap()
        .map(|entry| entry.unwrap())
        .filter(|entry| entry.file_name().to_str().unwrap().ends_with(".gitignore"))
        .count();

    assert!(gitignore_count > 0);
}
