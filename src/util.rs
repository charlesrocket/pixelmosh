use adler::adler32;
use png::{BitDepth, ColorType};

use std::{fs::File, io::BufReader};

use super::{
    ops::{read_file, write_file},
    MoshCore,
};

#[test]
fn rgb() {
    let input = read_file("src/util/test-rgb.png").unwrap();
    let mut image = MoshCore::new();
    image.read_image(&input).unwrap();
    image.mosh().unwrap();
    write_file(
        "moshed-rgb.png",
        &image.data.buf,
        image.data.width,
        image.data.height,
        image.data.color_type,
        image.data.bit_depth,
    )
    .unwrap();

    let output = File::open("moshed-rgb.png").unwrap();
    let mut file = BufReader::new(output);
    let checksum = adler32(&mut file).unwrap();

    assert_eq!(checksum, 177_632_196);
}

#[test]
fn rgba() {
    let input = read_file("src/util/test-rgb-alpha.png").unwrap();
    let mut image = MoshCore::new();
    image.read_image(&input).unwrap();
    image.mosh().unwrap();
    write_file(
        "moshed-rgb-alpha.png",
        &image.data.buf,
        image.data.width,
        image.data.height,
        image.data.color_type,
        image.data.bit_depth,
    )
    .unwrap();

    let output = File::open("moshed-rgb-alpha.png").unwrap();
    let mut file = BufReader::new(output);
    let checksum = adler32(&mut file).unwrap();

    assert_eq!(checksum, 4_232_986_456);
}

#[test]
fn grayscale() {
    let input = read_file("src/util/test-grayscale.png").unwrap();
    let mut image = MoshCore::new();
    image.read_image(&input).unwrap();
    image.mosh().unwrap();
    write_file(
        "moshed-grayscale.png",
        &image.data.buf,
        image.data.width,
        image.data.height,
        image.data.color_type,
        image.data.bit_depth,
    )
    .unwrap();

    let output = File::open("moshed-grayscale.png").unwrap();
    let mut file = BufReader::new(output);
    let checksum = adler32(&mut file).unwrap();

    assert_eq!(checksum, 2_950_358_433);
}

#[test]
fn seed() {
    let mut image = MoshCore::default();
    assert_eq!(image.options.seed, 1);

    image.options.new_seed();
    assert_eq!(image.options.seed, 901_042_006);
}

#[test]
#[should_panic(expected = "UnsupportedColorType")]
fn grayscale_alpha() {
    let input = read_file("src/util/test-grayscale-alpha.png").unwrap();
    let mut image = MoshCore::new();
    image.read_image(&input).unwrap();
    image.mosh().unwrap();
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
    let mut image = MoshCore::new();
    image.read_image(&input).unwrap();

    image.options.min_rate = 13;
    image.options.max_rate = 6;
    image.options.pixelation = 0;
    image.options.line_shift = 0.5;
    image.options.reverse = 0.4;
    image.options.flip = 0.3;
    image.options.channel_swap = 0.2;
    image.options.channel_shift = 0.1;
    image.options.seed = 42;

    image.mosh().unwrap();
}
