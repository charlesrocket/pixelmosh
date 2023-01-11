use std::{error::Error, process::Command};

use assert_cmd::prelude::*;
use predicates::str::contains;

#[test]
fn file_not_found() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("pixelmosh")?;

    cmd.arg("test/file/not/found");

    if cfg!(windows) {
        cmd.assert()
            .failure()
            .stderr(contains("The system cannot find the path specified"));
    }

    if cfg!(unix) {
        cmd.assert()
            .failure()
            .stderr(contains("No such file or directory"));
    }

    Ok(())
}

#[test]
fn invalid_parameters() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("pixelmosh")?;

    cmd.arg("src/util/test-pixelation.png")
        .arg("--pixelation")
        .arg("255");

    cmd.assert()
        .failure()
        .stderr(contains("Invalid parameters"));

    Ok(())
}

#[test]
fn invalid_sig() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("pixelmosh")?;

    cmd.arg("README.md");
    cmd.assert()
        .failure()
        .stderr(contains("Invalid PNG signature"));

    Ok(())
}

#[test]
fn unsupported_color_type() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("pixelmosh")?;

    cmd.arg("src/util/test-grayscale-alpha.png");
    cmd.assert()
        .failure()
        .stderr(contains("Unsupported color type"));

    Ok(())
}

#[test]
fn main() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("pixelmosh")?;

    cmd.arg("src/util/test-rgb.png")
        .arg("--min-rate")
        .arg("5")
        .arg("--max-rate")
        .arg("10")
        .arg("--line-shift")
        .arg("0.2")
        .arg("--seed")
        .arg("1309999")
        .arg("--output")
        .arg("test.png")
        .assert()
        .success();

    Ok(())
}
