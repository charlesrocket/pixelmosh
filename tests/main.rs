#[cfg(feature = "cli")]
mod cli {
    use std::{error::Error, fs::File, io::BufReader, process::Command};

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

        cmd.arg("src/util/test-indexed.png");
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
            .arg("2")
            .arg("--output")
            .arg("moshed-test")
            .arg("--seed")
            .arg("1309999")
            .assert()
            .success();

        let output_1 = File::open("moshed-test-001.png")?;
        let output_2 = File::open("moshed-test-002.png")?;

        let mut file_1 = BufReader::new(output_1);
        let checksum_1 = adler32(&mut file_1)?;

        let mut file_2 = BufReader::new(output_2);
        let checksum_2 = adler32(&mut file_2)?;

        assert_eq!(checksum_1, 3_406_465_027);
        assert_eq!(checksum_2, 1_498_643_664);

        Ok(())
    }

    #[test]
    fn single() -> Result<(), Box<dyn Error>> {
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

        assert_eq!(checksum, 3_487_120_849);

        Ok(())
    }
}
