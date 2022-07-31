use adler::adler32;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use super::*;

#[test]
fn rgb() {
    let min_rate = 5;
    let max_rate = 7;
    let line_shift_rng = 0.8;
    let reverse_rng = 0.4;
    let flip_rng = 0.3;
    let channel_swap_rng = 0.9;
    let channel_shift_rng = 0.5;
    let pixelation = 10;
    let options = Options {
        min_rate,
        max_rate,
        line_shift_rng,
        reverse_rng,
        flip_rng,
        channel_swap_rng,
        channel_shift_rng,
        pixelation,
    };

    let mut rng = ChaCha8Rng::seed_from_u64(901_042_006);
    let (mut buf, info) = cli::read_file("src/util/test-rgb.png".to_string()).unwrap();

    mosh(&info, &mut buf, &mut rng, &options).unwrap();
    cli::write_file("moshed.png", &buf, &info).unwrap();

    let output = std::fs::File::open("moshed.png").unwrap();
    let mut file = std::io::BufReader::new(output);
    let checksum = adler32(&mut file).unwrap();

    assert_eq!(checksum, 2_761_705_494);
}

#[test]
fn rgba() {
    let min_rate = 5;
    let max_rate = 7;
    let line_shift_rng = 0.8;
    let reverse_rng = 0.4;
    let flip_rng = 0.3;
    let channel_swap_rng = 0.9;
    let channel_shift_rng = 0.5;
    let pixelation = 10;
    let options = Options {
        min_rate,
        max_rate,
        line_shift_rng,
        reverse_rng,
        flip_rng,
        channel_swap_rng,
        channel_shift_rng,
        pixelation,
    };

    let mut rng = ChaCha8Rng::seed_from_u64(901_042_006);
    let (mut buf, info) = cli::read_file("src/util/test-rgb-alpha.png".to_string()).unwrap();

    mosh(&info, &mut buf, &mut rng, &options).unwrap();
    cli::write_file("moshed.png", &buf, &info).unwrap();

    let output = std::fs::File::open("moshed.png").unwrap();
    let mut file = std::io::BufReader::new(output);
    let checksum = adler32(&mut file).unwrap();

    assert_eq!(checksum, 1_601_040_326);
}

#[test]
fn grayscale() {
    let min_rate = 5;
    let max_rate = 7;
    let line_shift_rng = 0.8;
    let reverse_rng = 0.4;
    let flip_rng = 0.3;
    let channel_swap_rng = 0.9;
    let channel_shift_rng = 0.5;
    let pixelation = 10;
    let options = Options {
        min_rate,
        max_rate,
        line_shift_rng,
        reverse_rng,
        flip_rng,
        channel_swap_rng,
        channel_shift_rng,
        pixelation,
    };

    let mut rng = ChaCha8Rng::seed_from_u64(901_042_006);
    let (mut buf, info) = cli::read_file("src/util/test-grayscale.png".to_string()).unwrap();

    mosh(&info, &mut buf, &mut rng, &options).unwrap();
    cli::write_file("moshed.png", &buf, &info).unwrap();

    let output = std::fs::File::open("moshed.png").unwrap();
    let mut file = std::io::BufReader::new(output);
    let checksum = adler32(&mut file).unwrap();

    assert_eq!(checksum, 220_441_887);
}
