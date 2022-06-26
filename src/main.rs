use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

pub mod engine;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    #[clap(required = true, display_order = 1)]
    file: String,

    #[clap(long, default_value_t = 1, display_order = 2)]
    min_rate: u16,

    #[clap(long, default_value_t = 7, display_order = 3)]
    max_rate: u16,

    #[clap(long, default_value_t = 0.5, display_order = 4)]
    line_shift: f64,

    #[clap(long, default_value_t = 0.5, display_order = 5)]
    channel_swap: f64,

    #[clap(long, default_value_t = 0.5, display_order = 6)]
    channel_shift: f64,

    #[clap(short, long, default_value_t = thread_rng().next_u64(), hide_default_value = true, display_order = 7)]
    seed: u64,
}

fn read_file(file: String) -> (std::vec::Vec<u8>, png::OutputInfo) {
    let decoder = png::Decoder::new(File::open(file).expect("File not found"));
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();

    (buf, info)
}

fn write_file(buf: std::vec::Vec<u8>, info: png::OutputInfo) {
    let path = Path::new("moshed.png");
    let output = File::create(path).unwrap();
    let buf_writer = &mut BufWriter::new(output);
    let mut encoder = png::Encoder::new(buf_writer, info.width, info.height);

    encoder.set_color(info.color_type);
    encoder.set_depth(info.bit_depth);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&buf).unwrap();
}

fn main() {
    let spinner = ProgressBar::new_spinner();
    let args = Args::parse();
    let min_rate = args.min_rate;
    let max_rate = args.max_rate;
    let line_shift_rng = args.line_shift;
    let channel_swap_rng = args.channel_swap;
    let channel_shift_rng = args.channel_shift;
    let seed = args.seed;
    let options = engine::Options {
        min_rate,
        max_rate,
        line_shift_rng,
        channel_swap_rng,
        channel_shift_rng,
    };

    println!("Seed: \x1b[100m{}\x1b[0m", seed);
    spinner.enable_steady_tick(140);
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&[
                "∙∙∙∙∙",
                "●∙∙∙∙",
                "∙●∙∙∙",
                "∙∙●∙∙",
                "∙∙∙●∙",
                "∙∙∙∙●",
                "∙∙∙∙∙"
            ]),
    );

    spinner.set_message("Reading input");
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let (mut buf, info) = read_file(args.file);

    spinner.set_message("\x1b[94mProcessing\x1b[0m");
    engine::mosh(&info, &mut buf, &mut rng, &options);

    spinner.set_message("Writing output");
    write_file(buf, info);
    spinner.finish_with_message("\x1b[32mDONE\x1b[0m");
}

#[cfg(test)]
mod tests;
