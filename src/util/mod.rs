use adler::adler32;

use super::{Options, mosh, ops};

#[test]
fn rgb() {
    let (mut buf, info) = ops::read_file("src/util/test-rgb.png".to_string()).unwrap();

    mosh(&info, &mut buf, &Options::default()).unwrap();
    ops::write_file("moshed.png", &buf, &info).unwrap();

    let output = std::fs::File::open("moshed.png").unwrap();
    let mut file = std::io::BufReader::new(output);
    let checksum = adler32(&mut file).unwrap();

    assert_eq!(checksum, 2_675_922_770);
}

#[test]
fn rgba() {
    let (mut buf, info) = ops::read_file("src/util/test-rgb-alpha.png".to_string()).unwrap();

    mosh(&info, &mut buf, &Options::default()).unwrap();
    ops::write_file("moshed.png", &buf, &info).unwrap();

    let output = std::fs::File::open("moshed.png").unwrap();
    let mut file = std::io::BufReader::new(output);
    let checksum = adler32(&mut file).unwrap();

    assert_eq!(checksum, 3_414_819_387);
}

#[test]
fn grayscale() {
    let (mut buf, info) = ops::read_file("src/util/test-grayscale.png".to_string()).unwrap();

    mosh(&info, &mut buf, &Options::default()).unwrap();
    ops::write_file("moshed.png", &buf, &info).unwrap();

    let output = std::fs::File::open("moshed.png").unwrap();
    let mut file = std::io::BufReader::new(output);
    let checksum = adler32(&mut file).unwrap();

    assert_eq!(checksum, 376_952_495);
}

#[test]
fn grayscale_alpha() {
    let (mut buf, info) = ops::read_file("src/util/test-grayscale-alpha.png".to_string()).unwrap();
    let result = mosh(&info, &mut buf, &Options::default());

    assert!(result.is_err());
}
