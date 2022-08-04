#![feature(test)]
extern crate test;

use test::black_box;

use libmosh::{mosh as libmosh, ops, Options};

/// # Panics
///
/// TODO
pub fn rgb(bench: u64) {
    let options = Options {
        min_rate: 5,
        max_rate: 7,
        pixelation: 10,
        line_shift_rng: 0.8,
        reverse_rng: 0.4,
        flip_rng: 0.3,
        channel_swap_rng: 0.9,
        channel_shift_rng: 0.5,
        seed: bench,
    };

    let (mut buf, info) = ops::read_file("src/util/bench-rgb.png".to_string()).unwrap();

    libmosh(&info, &mut buf, &options).unwrap();
}

/// # Panics
///
/// TODO
pub fn rgba(bench: u64) {
    let options = Options {
        min_rate: 5,
        max_rate: 7,
        pixelation: 10,
        line_shift_rng: 0.8,
        reverse_rng: 0.4,
        flip_rng: 0.3,
        channel_swap_rng: 0.9,
        channel_shift_rng: 0.5,
        seed: bench,
    };

    let (mut buf, info) = ops::read_file("src/util/bench-rgb-alpha.png".to_string()).unwrap();

    libmosh(&info, &mut buf, &options).unwrap();
}

/// # Panics
///
/// TODO
pub fn grayscale(bench: u64) {
    let options = Options {
        min_rate: 5,
        max_rate: 7,
        pixelation: 10,
        line_shift_rng: 0.8,
        reverse_rng: 0.4,
        flip_rng: 0.3,
        channel_swap_rng: 0.9,
        channel_shift_rng: 0.5,
        seed: bench,
    };

    let (mut buf, info) = ops::read_file("src/util/bench-grayscale.png".to_string()).unwrap();

    libmosh(&info, &mut buf, &options).unwrap();
}

#[cfg(test)]
mod benches {
    use super::{black_box, grayscale, rgb, rgba, test};

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
