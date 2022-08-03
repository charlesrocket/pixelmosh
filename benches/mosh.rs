#![feature(test)]
extern crate test;

use test::black_box;

use libmosh::{mosh as libmosh, ops, Options};

/// # Panics
///
/// TODO
pub fn rgb(bench: u64) {
     let min_rate = 5;
     let max_rate = 7;
     let pixelation = 10;
     let line_shift_rng = 0.8;
     let reverse_rng = 0.4;
     let flip_rng = 0.3;
     let channel_swap_rng = 0.9;
     let channel_shift_rng = 0.5;
     let seed = bench;
     let options = Options {
         min_rate,
         max_rate,
         pixelation,
         line_shift_rng,
         reverse_rng,
         flip_rng,
         channel_swap_rng,
         channel_shift_rng,
         seed,
     };

    let (mut buf, info) = ops::read_file("src/util/bench-rgb.png".to_string()).unwrap();

    libmosh(&info, &mut buf, &options).unwrap();
}

/// # Panics
///
/// TODO
pub fn rgba(bench: u64) {
    let min_rate = 5;
    let max_rate = 7;
    let pixelation = 10;
    let line_shift_rng = 0.8;
    let reverse_rng = 0.4;
    let flip_rng = 0.3;
    let channel_swap_rng = 0.9;
    let channel_shift_rng = 0.5;
    let seed = bench;
    let options = Options {
        min_rate,
        max_rate,
        pixelation,
        line_shift_rng,
        reverse_rng,
        flip_rng,
        channel_swap_rng,
        channel_shift_rng,
        seed,
    };

    let (mut buf, info) = ops::read_file("src/util/bench-rgb-alpha.png".to_string()).unwrap();

    libmosh(&info, &mut buf, &options).unwrap();
}

/// # Panics
///
/// TODO
pub fn grayscale(bench: u64) {
    let min_rate = 5;
    let max_rate = 7;
    let pixelation = 10;
    let line_shift_rng = 0.8;
    let reverse_rng = 0.4;
    let flip_rng = 0.3;
    let channel_swap_rng = 0.9;
    let channel_shift_rng = 0.5;
    let seed = bench;
    let options = Options {
        min_rate,
        max_rate,
        pixelation,
        line_shift_rng,
        reverse_rng,
        flip_rng,
        channel_swap_rng,
        channel_shift_rng,
        seed,
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
