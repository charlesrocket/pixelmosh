use adler::adler32;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use super::*;

#[test]
fn rgb() {
    let mut rng = ChaCha8Rng::seed_from_u64(901_042_006);
    let (mut buf, info) = ops::read_file("src/util/test-rgb.png".to_string()).unwrap();

    mosh(&info, &mut buf, &mut rng, &Options::default()).unwrap();
    ops::write_file("moshed.png", &buf, &info).unwrap();

    let output = std::fs::File::open("moshed.png").unwrap();
    let mut file = std::io::BufReader::new(output);
    let checksum = adler32(&mut file).unwrap();

    assert_eq!(checksum, 2_675_922_770);
}

#[test]
fn rgba() {
    let mut rng = ChaCha8Rng::seed_from_u64(901_042_006);
    let (mut buf, info) = ops::read_file("src/util/test-rgb-alpha.png".to_string()).unwrap();

    mosh(&info, &mut buf, &mut rng, &Options::default()).unwrap();
    ops::write_file("moshed.png", &buf, &info).unwrap();

    let output = std::fs::File::open("moshed.png").unwrap();
    let mut file = std::io::BufReader::new(output);
    let checksum = adler32(&mut file).unwrap();

    assert_eq!(checksum, 3_414_819_387);
}

#[test]
fn grayscale() {
    let mut rng = ChaCha8Rng::seed_from_u64(901_042_006);
    let (mut buf, info) = ops::read_file("src/util/test-grayscale.png".to_string()).unwrap();

    mosh(&info, &mut buf, &mut rng, &Options::default()).unwrap();
    ops::write_file("moshed.png", &buf, &info).unwrap();

    let output = std::fs::File::open("moshed.png").unwrap();
    let mut file = std::io::BufReader::new(output);
    let checksum = adler32(&mut file).unwrap();

    assert_eq!(checksum, 376_952_495);
}
