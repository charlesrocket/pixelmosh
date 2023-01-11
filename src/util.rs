use adler::adler32;
use png::{BitDepth, ColorType};

use std::{fs::File, io::BufReader};

use super::{
    ops::{read_file, write_file},
    MoshData, MoshOptions,
};

#[test]
fn rgb() {
    let input = read_file("src/util/test-rgb.png").unwrap();
    let mut image = MoshData::new(&input).unwrap();
    image.mosh(&MoshOptions::default()).unwrap();
    write_file(
        "moshed.png",
        &image.data,
        image.width,
        image.height,
        image.color_type,
        image.bit_depth,
    )
    .unwrap();

    let output = File::open("moshed.png").unwrap();
    let mut file = BufReader::new(output);
    let checksum = adler32(&mut file).unwrap();

    assert_eq!(checksum, 2_285_399_975);
}

#[test]
fn rgba() {
    let input = read_file("src/util/test-rgb-alpha.png").unwrap();
    let mut image = MoshData::new(&input).unwrap();
    image.mosh(&MoshOptions::default()).unwrap();
    write_file(
        "moshed.png",
        &image.data,
        image.width,
        image.height,
        image.color_type,
        image.bit_depth,
    )
    .unwrap();

    let output = File::open("moshed.png").unwrap();
    let mut file = BufReader::new(output);
    let checksum = adler32(&mut file).unwrap();

    assert_eq!(checksum, 105_467_096);
}

#[test]
fn grayscale() {
    let input = read_file("src/util/test-grayscale.png").unwrap();
    let mut image = MoshData::new(&input).unwrap();
    image.mosh(&MoshOptions::default()).unwrap();
    write_file(
        "moshed.png",
        &image.data,
        image.width,
        image.height,
        image.color_type,
        image.bit_depth,
    )
    .unwrap();

    let output = File::open("moshed.png").unwrap();
    let mut file = BufReader::new(output);
    let checksum = adler32(&mut file).unwrap();

    assert_eq!(checksum, 1_718_963_212);
}

#[test]
#[should_panic(expected = "UnsupportedColorType")]
fn grayscale_alpha() {
    let input = read_file("src/util/test-grayscale-alpha.png").unwrap();
    let mut image = MoshData::new(&input).unwrap();
    image.mosh(&MoshOptions::default()).unwrap();
}

#[test]
#[should_panic(expected = "EncodingError")]
fn encoding() {
    write_file(
        "moshed.png",
        &[0_u8],
        400,
        400,
        ColorType::Rgba,
        BitDepth::Eight,
    )
    .unwrap();
}

#[test]
fn invalid_parameters() {
    let input = read_file("src/util/test-grayscale.png").unwrap();
    let mut image = MoshData::new(&input).unwrap();
    let options = MoshOptions {
        min_rate: 13,
        max_rate: 6,
        pixelation: 0,
        line_shift: 0.5,
        reverse: 0.4,
        flip: 0.3,
        channel_swap: 0.2,
        channel_shift: 0.1,
        seed: 42,
    };

    image.mosh(&options).unwrap();
}
