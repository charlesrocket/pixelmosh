use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

pub mod engine;

#[derive(Parser, Debug)]
#[clap(version, author = engine::INTRO, about, long_about = None)]
struct Args {
    #[clap(required = true, display_order = 1)]
    file: String,

    #[clap(long, default_value_t = 1, display_order = 2)]
    min_rate: u16,

    #[clap(long, default_value_t = 7, display_order = 3)]
    max_rate: u16,

    #[clap(long, default_value_t = 0.3, display_order = 4)]
    line_shift: f64,

    #[clap(long, default_value_t = 0.3, display_order = 5)]
    reverse: f64,

    #[clap(long, default_value_t = 0.3, display_order = 6)]
    flip: f64,

    #[clap(long, default_value_t = 0.3, display_order = 7)]
    channel_swap: f64,

    #[clap(long, default_value_t = 0.3, display_order = 8)]
    channel_shift: f64,

    #[clap(short, long, default_value_t = thread_rng().next_u64(),
           hide_default_value = true, display_order = 9)]
    seed: u64,

    #[clap(
        short,
        long,
        default_value = "moshed.png",
        hide_default_value = true,
        display_order = 10
    )]
    output: String,
}

fn read_file(file: String) -> (std::vec::Vec<u8>, png::OutputInfo) {
    let input = File::open(file);
    let input = match input {
        Ok(file) => file,
        Err(error) => {
            eprintln!("\x1b[1;31merror:\x1b[0m {}", error);
            std::process::exit(1)
        }
    };

    let decoder = png::Decoder::new(input);
    let reader = decoder.read_info();
    let mut reader = match reader {
        Ok(reader) => reader,
        Err(error) => {
            eprintln!("\x1b[1;31merror:\x1b[0m {} Exiting", error);
            std::process::exit(0)
        }
    };

    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    (buf, info)
}

fn write_file(path: &str, buf: &[u8], info: &png::OutputInfo) {
    let path = Path::new(&path);
    let output = File::create(path).unwrap();
    let buf_writer = &mut BufWriter::new(output);
    let mut encoder = png::Encoder::new(buf_writer, info.width, info.height);

    encoder.set_color(info.color_type);
    encoder.set_depth(info.bit_depth);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(buf).unwrap();
}

fn display_var() -> bool {
    matches!(env::var("DISPLAY"), Ok(_))
}

fn main() {
    let spinner = ProgressBar::new_spinner();
    let args = Args::parse();
    let output = args.output;
    let min_rate = args.min_rate;
    let max_rate = args.max_rate;
    let line_shift_rng = args.line_shift;
    let reverse_rng = args.reverse;
    let flip_rng = args.flip;
    let channel_swap_rng = args.channel_swap;
    let channel_shift_rng = args.channel_shift;
    let seed = args.seed;
    let options = engine::Options {
        min_rate,
        max_rate,
        line_shift_rng,
        reverse_rng,
        flip_rng,
        channel_swap_rng,
        channel_shift_rng,
    };

    let spinner_style = if cfg!(unix) {
        if display_var() {
            engine::SPINNER_2
        } else {
            engine::SPINNER_1
        }
    } else {
        engine::SPINNER_1
    };

    println!("File: {}", args.file);
    println!("Seed: \x1b[3m{}\x1b[0m", seed);

    spinner.enable_steady_tick(90);
    spinner.set_style(ProgressStyle::default_spinner().tick_strings(&spinner_style));

    spinner.set_message("Reading input");
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let (mut buf, info) = read_file(args.file);

    spinner.set_message("\x1b[94mProcessing\x1b[0m");
    engine::mosh(&info, &mut buf, &mut rng, &options);

    spinner.set_message("Writing output");
    write_file(&output, &buf, &info);
    spinner.finish_with_message("\x1b[1;32mDONE\x1b[0m");
}

#[cfg(test)]
mod utils;
