use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_without_args() {
    let mut cmd = Command::cargo_bin("vsix-install").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn test_search_command_without_query() {
    let mut cmd = Command::cargo_bin("vsix-install").unwrap();
    cmd.arg("search")
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_install_command_without_id() {
    let mut cmd = Command::cargo_bin("vsix-install").unwrap();
    cmd.arg("install")
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("vsix-install").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Downloads and installs .vsix extensions"));
}

#[test]
fn test_version_command() {
    let mut cmd = Command::cargo_bin("vsix-install").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("vsix-install"));
}

#[test]
fn test_search_help() {
    let mut cmd = Command::cargo_bin("vsix-install").unwrap();
    cmd.args(&["search", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Search for extensions"));
}

#[test]
fn test_install_help() {
    let mut cmd = Command::cargo_bin("vsix-install").unwrap();
    cmd.args(&["install", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Install an extension"));
}