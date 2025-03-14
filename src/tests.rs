use adler::adler32;

use std::{fs::File, io::BufReader};

use super::{
    MoshCore, MoshData, MoshOptions,
    ops::{read_file, write_file},
};

#[test]
fn ansi_rgb() {
    let input = read_file("tests/assets/test-rgb.png").unwrap();
    let mut image = MoshCore::new();
    image.options.ansi = true;
    image.read_image(&input).unwrap();
    image.mosh().unwrap();
    write_file("moshed-ansi-rgb.png", &image.data, &image.options).unwrap();

    let output = File::open("moshed-ansi-rgb.png").unwrap();
    let mut file = BufReader::new(output);
    let checksum = adler32(&mut file).unwrap();

    assert_eq!(checksum, 2_188_040_842);
}

#[test]
fn ansi_rgb_alpha() {
    let input = read_file("tests/assets/test-rgb-alpha.png").unwrap();
    let mut image = MoshCore::new();
    image.options.ansi = true;
    image.read_image(&input).unwrap();
    image.mosh().unwrap();
    write_file("moshed-ansi-rgb-alpha.png", &image.data, &image.options).unwrap();

    let output = File::open("moshed-ansi-rgb-alpha.png").unwrap();
    let mut file = BufReader::new(output);
    let checksum = adler32(&mut file).unwrap();

    assert_eq!(checksum, 309_438_591);
}

#[test]
fn ansi_grayscale() {
    let input = read_file("tests/assets/test-grayscale.png").unwrap();
    let mut image = MoshCore::new();
    image.options.ansi = true;
    image.read_image(&input).unwrap();
    image.mosh().unwrap();
    write_file("moshed-grayscale.png", &image.data, &image.options).unwrap();

    let output = File::open("moshed-grayscale.png").unwrap();
    let mut file = BufReader::new(output);
    let checksum = adler32(&mut file).unwrap();

    assert_eq!(checksum, 2_433_927_020);
}

#[test]
fn ansi_grayscale_alpha() {
    let input = read_file("tests/assets/test-grayscale-alpha.png").unwrap();
    let mut image = MoshCore::new();
    image.options.ansi = true;
    image.read_image(&input).unwrap();
    image.mosh().unwrap();
    write_file("moshed-grayscale-alpha.png", &image.data, &image.options).unwrap();

    let output = File::open("moshed-grayscale-alpha.png").unwrap();
    let mut file = BufReader::new(output);
    let checksum = adler32(&mut file).unwrap();

    assert_eq!(checksum, 2_083_001_249);
}

#[test]
fn rgb() {
    let input = read_file("tests/assets/test-rgb.png").unwrap();
    let mut image = MoshCore::new();
    image.read_image(&input).unwrap();
    image.mosh().unwrap();
    write_file("moshed-rgb.png", &image.data, &image.options).unwrap();

    let output = File::open("moshed-rgb.png").unwrap();
    let mut file = BufReader::new(output);
    let checksum = adler32(&mut file).unwrap();

    assert_eq!(checksum, 1_527_506_200);
}

#[test]
fn rgb_alpha() {
    let input = read_file("tests/assets/test-rgb-alpha.png").unwrap();
    let mut image = MoshCore::new();
    image.read_image(&input).unwrap();
    image.mosh().unwrap();
    write_file("moshed-rgb-alpha.png", &image.data, &image.options).unwrap();

    let output = File::open("moshed-rgb-alpha.png").unwrap();
    let mut file = BufReader::new(output);
    let checksum = adler32(&mut file).unwrap();

    assert_eq!(checksum, 2_248_848_268);
}

#[test]
fn grayscale() {
    let input = read_file("tests/assets/test-grayscale.png").unwrap();
    let mut image = MoshCore::new();
    image.read_image(&input).unwrap();
    image.mosh().unwrap();
    write_file("moshed-grayscale.png", &image.data, &image.options).unwrap();

    let output = File::open("moshed-grayscale.png").unwrap();
    let mut file = BufReader::new(output);
    let checksum = adler32(&mut file).unwrap();

    assert_eq!(checksum, 270_870_204);
}

#[test]
fn grayscale_alpha() {
    let input = read_file("tests/assets/test-grayscale-alpha.png").unwrap();
    let mut image = MoshCore::new();
    image.options.ansi = true;
    image.read_image(&input).unwrap();
    image.mosh().unwrap();
    write_file("moshed-grayscale-alpha.png", &image.data, &image.options).unwrap();

    let output = File::open("moshed-grayscale-alpha.png").unwrap();
    let mut file = BufReader::new(output);
    let checksum = adler32(&mut file).unwrap();

    assert_eq!(checksum, 2_083_001_249);
}

#[test]
fn indexed() {
    let input = read_file("tests/assets/test-indexed.png").unwrap();
    let mut image = MoshCore::new();
    image.read_image(&input).unwrap();
    image.mosh().unwrap();
    write_file("moshed-indexed.png", &image.data, &image.options).unwrap();

    let output = File::open("moshed-indexed.png").unwrap();
    let mut file = BufReader::new(output);
    let checksum = adler32(&mut file).unwrap();

    assert_eq!(checksum, 619_133_944);
}

#[test]
fn ansi_indexed() {
    let input = read_file("tests/assets/test-indexed.png").unwrap();
    let mut image = MoshCore::new();
    image.options.ansi = true;
    image.read_image(&input).unwrap();
    image.mosh().unwrap();
    write_file("moshed-ansi-indexed.png", &image.data, &image.options).unwrap();

    let output = File::open("moshed-ansi-indexed.png").unwrap();
    let mut file = BufReader::new(output);
    let checksum = adler32(&mut file).unwrap();

    assert_eq!(checksum, 3_221_649_406);
}

#[test]
fn seed() {
    let mut image = MoshCore::default();
    image.options.seed = 1;
    image.options.new_seed();

    assert_eq!(image.options.seed, 901_042_006);
}

#[test]
#[should_panic(expected = "EncodingError")]
fn encoding() {
    write_file("moshed.png", &MoshData::default(), &MoshOptions::default()).unwrap();
}

#[test]
#[should_panic(expected = "InvalidPalette")]
fn invalid_palette() {
    let input = read_file("tests/assets/test-indexed.png").unwrap();
    let mut image = MoshCore::new();
    image.read_image(&input).unwrap();
    image.options.ansi = true;
    image.data.palette = None;
    image.mosh().unwrap();
}

#[test]
fn invalid_parameters() {
    let input = read_file("tests/assets/test-grayscale.png").unwrap();
    let mut image = MoshCore::new();
    image.read_image(&input).unwrap();

    image.options.pixelation = 106;
    image.options.line_shift = 0.5;
    image.options.reverse = 0.4;
    image.options.flip = 0.3;
    image.options.channel_swap = 0.2;
    image.options.channel_shift = 0.1;
    image.options.seed = 42;

    image.mosh().unwrap();
}
