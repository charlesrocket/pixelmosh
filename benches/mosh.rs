#![feature(test)]
extern crate test;

use test::black_box;

use libmosh::{MoshData, MoshOptions};

use crate::images::{GRAYSCALE, RGB, RGBA};

mod images;

struct Bench {}

impl Bench {
    fn bench_image(input: u64, image: &[u8]) {
        let options = MoshOptions {
            min_rate: 3,
            max_rate: 3,
            pixelation: 2,
            line_shift: 1.0,
            reverse: 1.0,
            flip: 1.0,
            channel_swap: 1.0,
            channel_shift: 1.0,
            seed: input,
        };

        let mut image = MoshData::new(image).unwrap();

        image.mosh(&options).unwrap();
    }
}

fn rgb(value: u64) {
    Bench::bench_image(value, RGB);
}

fn rgba(value: u64) {
    Bench::bench_image(value, RGBA);
}

fn grayscale(value: u64) {
    Bench::bench_image(value, GRAYSCALE);
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
