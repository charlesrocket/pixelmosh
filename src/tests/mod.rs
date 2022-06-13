use super::*;
use adler::adler32;

#[test]
    fn engine() {
        let min_rate = 5;
        let max_rate = 7;
        let line_shift_rng = 0.8;
        let channel_swap_rng = 0.9;
        let channel_shift_rng = 0.5;

        let options = engine::Options {
            min_rate,
            max_rate,
            line_shift_rng,
            channel_swap_rng,
            channel_shift_rng,
        };

        let mut rng = ChaCha8Rng::seed_from_u64(901042006);
        let (mut buf, info) = read_file("src/tests/test.png".to_string());

        engine::mosh(&info, &mut buf, &mut rng, &options);
        write_file(buf, info);

        let output = File::open("moshed.png").unwrap();
        let mut file = std::io::BufReader::new(output);

        let checksum = adler32(&mut file).unwrap();

        assert_eq!(checksum, 3361689054);
    }
