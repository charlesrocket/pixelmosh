#![feature(test)]
extern crate test;

use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha8Rng;
use test::black_box;

use libmosh::{mosh as libmosh, ops, Options};

pub fn rgb(seed: u64) {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let (mut buf, info) = ops::read_file("src/util/bench-rgb.png".to_string()).unwrap();

    libmosh(&info, &mut buf, &mut rng, &Options::default()).unwrap();
}

pub fn rgba(seed: u64) {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let (mut buf, info) = ops::read_file("src/util/bench-rgb-alpha.png".to_string()).unwrap();

    libmosh(&info, &mut buf, &mut rng, &Options::default()).unwrap();
}

pub fn grayscale(seed: u64) {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let (mut buf, info) = ops::read_file("src/util/bench-grayscale.png".to_string()).unwrap();

    libmosh(&info, &mut buf, &mut rng, &Options::default()).unwrap();
}


#[cfg(test)]
mod benches {
    use super::*;

    #[bench]
    fn bench_rgb(b: &mut test::Bencher) {
        b.iter(|| rgb(black_box(20)));
    }

    #[bench]
    fn bench_rgba(b: &mut test::Bencher) {
        b.iter(|| rgba(black_box(20)));
    }

    #[bench]
    fn bench_grayscale(b: &mut test::Bencher) {
        b.iter(|| grayscale(black_box(20)));
    }
}
