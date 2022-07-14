use super::*;
use adler::adler32;

#[test]
fn libmosh() {
    let min_rate = 5;
    let max_rate = 7;
    let line_shift_rng = 0.8;
    let reverse_rng = 0.4;
    let flip_rng = 0.3;
    let channel_swap_rng = 0.9;
    let channel_shift_rng = 0.5;
    let options = libmosh::Options {
        min_rate,
        max_rate,
        line_shift_rng,
        reverse_rng,
        flip_rng,
        channel_swap_rng,
        channel_shift_rng,
    };

    let mut rng = ChaCha8Rng::seed_from_u64(901042006);
    let (mut buf, info) = cli::read_file("src/util/test.png".to_string()).unwrap();

    libmosh::mosh(&info, &mut buf, &mut rng, &options);
    cli::write_file(&"moshed.png".to_string(), &buf, &info).unwrap();

    let output = std::fs::File::open("moshed.png").unwrap();
    let mut file = std::io::BufReader::new(output);
    let checksum = adler32(&mut file).unwrap();

    assert_eq!(checksum, 1914553783);
}