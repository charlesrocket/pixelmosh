#![feature(test)]
extern crate test;

use libmosh::MoshCore;

use crate::images::{GRAYSCALE, RGB, RGBA};

mod images;

struct Bench {}

impl Bench {
    fn bench_image(image: &[u8]) {
        let mut core = MoshCore::new();

        core.read_image(image).unwrap();

        core.options.min_rate = 3;
        core.options.max_rate = 3;
        core.options.pixelation = 2;
        core.options.line_shift = 1.0;
        core.options.reverse = 1.0;
        core.options.flip = 1.0;
        core.options.channel_swap = 1.0;
        core.options.channel_shift = 1.0;
        core.options.seed = 42;

        core.mosh().unwrap();
    }
}

fn rgb() {
    Bench::bench_image(RGB);
}

fn rgba() {
    Bench::bench_image(RGBA);
}

fn grayscale() {
    Bench::bench_image(GRAYSCALE);
}

#[cfg(test)]
mod benches {
    use super::{grayscale, rgb, rgba, test};

    #[bench]
    fn bench_rgb(b: &mut test::Bencher) {
        b.iter(|| rgb());
    }

    #[bench]
    fn bench_rgba(b: &mut test::Bencher) {
        b.iter(|| rgba());
    }

    #[bench]
    fn bench_grayscale(b: &mut test::Bencher) {
        b.iter(|| grayscale());
    }
}
