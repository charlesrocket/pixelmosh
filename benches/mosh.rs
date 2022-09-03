#![feature(test)]
extern crate test;

use test::black_box;

use libmosh::{mosh as libmosh, ops, MoshOptions};

struct Bench {}
impl Bench {
    fn bench_image(input: u64, image: String) {
        let options = MoshOptions {
            min_rate: 2,
            max_rate: 2,
            pixelation: 10,
            line_shift: 0.8,
            reverse: 0.4,
            flip: 0.3,
            channel_swap: 0.9,
            channel_shift: 0.5,
            seed: input,
        };

        let (mut buf, info) = ops::read_file(image).unwrap();

        libmosh(&info, &mut buf, &options).unwrap();
    }
}

fn rgb(value: u64) {
    Bench::bench_image(value, "benches/bench-rgb.png".to_string());
}

fn rgba(value: u64) {
    Bench::bench_image(value, "benches/bench-rgb-alpha.png".to_string());
}

fn grayscale(value: u64) {
    Bench::bench_image(value, "benches/bench-grayscale.png".to_string());
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
