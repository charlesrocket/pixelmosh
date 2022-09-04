use adler::adler32;

use super::{mosh, ops, MoshOptions};

#[test]
fn rgb() {
    let (mut buf, info) = ops::read_file("src/util/test-rgb.png".to_string()).unwrap();

    mosh(&info, &mut buf, &MoshOptions::default()).unwrap();
    ops::write_file("moshed.png", &buf, &info).unwrap();

    let output = std::fs::File::open("moshed.png").unwrap();
    let mut file = std::io::BufReader::new(output);
    let checksum = adler32(&mut file).unwrap();

    assert_eq!(checksum, 2_675_922_770);
}

#[test]
fn rgba() {
    let (mut buf, info) = ops::read_file("src/util/test-rgb-alpha.png".to_string()).unwrap();

    mosh(&info, &mut buf, &MoshOptions::default()).unwrap();
    ops::write_file("moshed.png", &buf, &info).unwrap();

    let output = std::fs::File::open("moshed.png").unwrap();
    let mut file = std::io::BufReader::new(output);
    let checksum = adler32(&mut file).unwrap();

    assert_eq!(checksum, 3_414_819_387);
}

#[test]
fn grayscale() {
    let (mut buf, info) = ops::read_file("src/util/test-grayscale.png".to_string()).unwrap();

    mosh(&info, &mut buf, &MoshOptions::default()).unwrap();
    ops::write_file("moshed.png", &buf, &info).unwrap();

    let output = std::fs::File::open("moshed.png").unwrap();
    let mut file = std::io::BufReader::new(output);
    let checksum = adler32(&mut file).unwrap();

    assert_eq!(checksum, 376_952_495);
}

#[test]
fn grayscale_alpha() {
    let (mut buf, info) = ops::read_file("src/util/test-grayscale-alpha.png".to_string()).unwrap();
    let result = mosh(&info, &mut buf, &MoshOptions::default());

    assert!(result.is_err());
}

#[test]
fn pixelation() {
    let options = MoshOptions {
        min_rate: 1,
        max_rate: 1,
        pixelation: 255,
        line_shift: 1.0,
        reverse: 1.0,
        flip: 1.0,
        channel_swap: 1.0,
        channel_shift: 1.0,
        seed: 42,
    };

    let (mut buf, info) = ops::read_file("src/util/test-pixelation.png".to_string()).unwrap();
    let result = mosh(&info, &mut buf, &options);

    assert!(result.is_err());
}
