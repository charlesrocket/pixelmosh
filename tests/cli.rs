use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn file_not_found() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("pixelmosh")?;

    cmd.arg("test/file/not/found");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn invalid_sig() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("pixelmosh")?;

    cmd.arg("README.md");
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("Invalid PNG signature"));

    Ok(())
}

#[test]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("pixelmosh")?;

    cmd.arg("src/tests/test.png")
        .arg("--min-rate")
        .arg("5")
        .arg("--max-rate")
        .arg("10")
        .arg("--line-shift")
        .arg("0.2")
        .arg("--seed")
        .arg("1309999")
        .assert()
        .success();

    Ok(())
}
