use super::{cli, ChaCha8Rng, SeedableRng};
use adler::adler32;

#[test]
fn mosh() {
    let min_rate = 5;
    let max_rate = 7;
    let line_shift_rng = 0.8;
    let reverse_rng = 0.4;
    let flip_rng = 0.3;
    let channel_swap_rng = 0.9;
    let channel_shift_rng = 0.5;
    let pixelation = 10;
    let options = libmosh::Options {
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

    let image = libmosh::mosh(&info, &mut buf, &mut rng, &options).unwrap();
    cli::write_file("moshed.png", &image, &info).unwrap();

    let output = std::fs::File::open("moshed.png").unwrap();
    let mut file = std::io::BufReader::new(output);
    let checksum = adler32(&mut file).unwrap();

    assert_eq!(checksum, 1_601_040_326);
}
