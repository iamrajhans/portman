use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("portman").unwrap();
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A CLI tool for managing ports"));
}

#[test]
fn test_version_command() {
    let mut cmd = Command::cargo_bin("portman").unwrap();
    cmd.arg("--version");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("portman"));
}

#[test]
fn test_list_command_basic() {
    let mut cmd = Command::cargo_bin("portman").unwrap();
    cmd.arg("list");

    // The command should succeed (even if no ports are found)
    cmd.assert().success();
}

#[test]
fn test_check_command_valid_port() {
    let mut cmd = Command::cargo_bin("portman").unwrap();
    cmd.args(["check", "60000"]);

    // Should succeed and report port as available
    cmd.assert().success();
}

#[test]
fn test_invalid_command() {
    let mut cmd = Command::cargo_bin("portman").unwrap();
    cmd.arg("invalid-command");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("error"));
}

#[test]
fn test_list_json_format() {
    let mut cmd = Command::cargo_bin("portman").unwrap();
    cmd.args(["list", "--format", "json"]);

    cmd.assert().success();
}

#[test]
fn test_list_csv_format() {
    let mut cmd = Command::cargo_bin("portman").unwrap();
    cmd.args(["list", "--format", "csv"]);

    cmd.assert().success();
}

#[test]
fn test_check_port_range() {
    let mut cmd = Command::cargo_bin("portman").unwrap();
    cmd.args(["check", "60000-60010"]);

    cmd.assert().success();
}

#[test]
fn test_init_command() {
    use tempfile::TempDir;

    let temp_dir = TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("portman").unwrap();

    cmd.current_dir(temp_dir.path()).args(["init", "--force"]);

    cmd.assert().success();

    // Check that config file was created
    assert!(temp_dir.path().join(".portman.yaml").exists());
}
