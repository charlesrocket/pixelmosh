use adler::adler32;

use super::ops::{read_file, write_file};
use super::{mosh, MoshOptions};

#[test]
fn rgb() {
    let (mut buf, info) = read_file("src/util/test-rgb.png").unwrap();

    mosh(&info, &mut buf, &MoshOptions::default()).unwrap();
    write_file("moshed.png", &buf, &info).unwrap();

    let output = std::fs::File::open("moshed.png").unwrap();
    let mut file = std::io::BufReader::new(output);
    let checksum = adler32(&mut file).unwrap();

    assert_eq!(checksum, 2_285_399_975);
}

#[test]
fn rgba() {
    let (mut buf, info) = read_file("src/util/test-rgb-alpha.png").unwrap();

    mosh(&info, &mut buf, &MoshOptions::default()).unwrap();
    write_file("moshed.png", &buf, &info).unwrap();

    let output = std::fs::File::open("moshed.png").unwrap();
    let mut file = std::io::BufReader::new(output);
    let checksum = adler32(&mut file).unwrap();

    assert_eq!(checksum, 105_467_096);
}

#[test]
fn grayscale() {
    let (mut buf, info) = read_file("src/util/test-grayscale.png").unwrap();

    mosh(&info, &mut buf, &MoshOptions::default()).unwrap();
    write_file("moshed.png", &buf, &info).unwrap();

    let output = std::fs::File::open("moshed.png").unwrap();
    let mut file = std::io::BufReader::new(output);
    let checksum = adler32(&mut file).unwrap();

    assert_eq!(checksum, 1_718_963_212);
}

#[test]
#[should_panic(expected = "UnsupportedColorType")]
fn grayscale_alpha() {
    let (mut buf, info) = read_file("src/util/test-grayscale-alpha.png").unwrap();
    mosh(&info, &mut buf, &MoshOptions::default()).unwrap();
}

#[test]
#[should_panic(expected = "EncodingError")]
fn encoding() {
    let info = png::OutputInfo {
        width: 400,
        height: 400,
        color_type: png::ColorType::Rgba,
        bit_depth: png::BitDepth::Eight,
        line_size: 1600,
    };

    write_file("moshed.png", &mut [0_u8], &info).unwrap();
}

#[test]
#[should_panic(expected = "InvalidParameters")]
fn pixelation() {
    let info = png::OutputInfo {
        width: 1,
        height: 1,
        color_type: png::ColorType::Rgba,
        bit_depth: png::BitDepth::Eight,
        line_size: 1,
    };

    let options = MoshOptions {
        min_rate: 3,
        max_rate: 5,
        pixelation: 0,
        line_shift: 0.5,
        reverse: 0.4,
        flip: 0.3,
        channel_swap: 0.2,
        channel_shift: 0.1,
        seed: 42,
    };

    mosh(&info, &mut [0_u8], &options).unwrap();
}
