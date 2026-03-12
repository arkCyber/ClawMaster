//! CLI integration tests for claw command.

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.arg("--help");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("ClawHub - Wasm Tool Package Manager"));
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.arg("--version");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("claw"));
}

// ── Skills Commands ─────────────────────────────────────────────────────────

#[test]
fn test_skills_help() {
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.arg("skills").arg("--help");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Skills management"));
}

#[test]
fn test_skills_search_help() {
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.arg("skills").arg("search").arg("--help");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Search for skills"));
}

#[test]
fn test_skills_info_help() {
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.arg("skills").arg("info").arg("--help");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Show skill information"));
}

#[test]
fn test_skills_publish_help() {
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.arg("skills").arg("publish").arg("--help");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Publish a skill"));
}

#[test]
fn test_skills_list_help() {
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.arg("skills").arg("list").arg("--help");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("List published skills"));
}

#[test]
fn test_skills_publish_missing_required_args() {
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.arg("skills").arg("publish").arg("test-skill");
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_skills_publish_missing_version() {
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.arg("skills")
       .arg("publish")
       .arg("test-skill")
       .arg("--description")
       .arg("Test")
       .arg("--author")
       .arg("Test");
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("version"));
}

// ── Tools Commands ──────────────────────────────────────────────────────────

#[test]
fn test_search_help() {
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.arg("search").arg("--help");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Search for tools"));
}

#[test]
fn test_install_help() {
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.arg("install").arg("--help");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Install a tool"));
}

#[test]
fn test_list_help() {
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.arg("list").arg("--help");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("List installed tools"));
}

#[test]
fn test_publish_help() {
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.arg("publish").arg("--help");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Publish a tool"));
}

#[test]
fn test_info_help() {
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.arg("info").arg("--help");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Show tool information"));
}

#[test]
fn test_install_missing_name() {
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.arg("install");
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_publish_missing_args() {
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.arg("publish");
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_info_missing_name() {
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.arg("info");
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

// ── Error Handling ──────────────────────────────────────────────────────────

#[test]
fn test_invalid_command() {
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.arg("invalid-command");
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized subcommand"));
}

#[test]
fn test_skills_invalid_subcommand() {
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.arg("skills").arg("invalid");
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized subcommand"));
}
