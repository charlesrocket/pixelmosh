use std::{error::Error, fs, io::BufReader, path::Path, process::Command};

use adler::adler32;
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
fn batch() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("pixelmosh")?;

    cmd.arg("src/util/test-grayscale.png")
        .arg("--batch")
        .arg("3")
        .arg("--output")
        .arg("moshed-test")
        .assert()
        .success();

    assert!(Path::new("moshed-test-001.png").exists());
    assert!(Path::new("moshed-test-002.png").exists());
    assert!(Path::new("moshed-test-003.png").exists());

    fs::remove_file("moshed-test-001.png").unwrap();
    fs::remove_file("moshed-test-002.png").unwrap();
    fs::remove_file("moshed-test-003.png").unwrap();

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
        .arg("test")
        .assert()
        .success();

    let output = File::open("test.png")?;
    let mut file = BufReader::new(output);
    let checksum = adler32(&mut file)?;

    assert_eq!(checksum, 3_958_067_430);

    Ok(())
}
