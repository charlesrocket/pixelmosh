use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

pub mod cli;

#[derive(Parser, Debug)]
#[clap(version, author = libmosh::INTRO, about, long_about = None)]
struct Args {
    #[clap(required = true, display_order = 1)]
    file: String,

    #[clap(short, long, default_value_t = 1, display_order = 2)]
    min_rate: u16,

    #[clap(short = 'n', long, default_value_t = 7, display_order = 3)]
    max_rate: u16,

    #[clap(short, long, default_value_t = 0.3, display_order = 4)]
    line_shift: f64,

    #[clap(short, long, default_value_t = 0.3, display_order = 5)]
    reverse: f64,

    #[clap(short, long, default_value_t = 0.3, display_order = 6)]
    flip: f64,

    #[clap(short, long, default_value_t = 0.3, display_order = 7)]
    channel_swap: f64,

    #[clap(short = 'k', long, default_value_t = 0.3, display_order = 8)]
    channel_shift: f64,

    #[clap(short, long, default_value_t = 30, display_order = 9)]
    pixelation: u32,

    #[clap(short, long, default_value_t = thread_rng().next_u64(),
           hide_default_value = true, display_order = 10)]
    seed: u64,

    #[clap(
        short,
        long,
        default_value = "moshed.png",
        hide_default_value = true,
        display_order = 11
    )]
    output: String,
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
    let pixelation = args.pixelation;
    let seed = args.seed;
    let options = libmosh::Options {
        min_rate,
        max_rate,
        line_shift_rng,
        reverse_rng,
        flip_rng,
        channel_swap_rng,
        channel_shift_rng,
    };

    // $TERM?
    let spinner_style = if cfg!(unix) {
        if cli::display_var() {
            libmosh::SPINNER_2
        } else {
            libmosh::SPINNER_1
        }
    } else {
        libmosh::SPINNER_1
    };

    println!("file: {}", args.file);
    println!("seed: \x1b[3m{}\x1b[0m", seed);

    spinner.enable_steady_tick(90);
    spinner.set_style(ProgressStyle::default_spinner().tick_strings(&spinner_style));
    spinner.set_message("\x1b[36mreading input\x1b[0m");

    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let (mut buf, info) = match cli::read_file(args.file) {
        Ok((buf, info)) => (buf, info),
        Err(error) => {
            eprintln!("\x1b[1;31merror:\x1b[0m {}", error);
            std::process::exit(1)
        }
    };

    spinner.set_message("\x1b[94mprocessing\x1b[0m");
    libmosh::mosh(&info, &mut buf, &mut rng, &options);

    spinner.set_message("\x1b[36mwriting output\x1b[0m");
    match cli::write_file(&output, &buf, &info) {
        Ok(()) => (spinner.set_message("\x1b[95mpixelating\x1b[0m")),
        Err(error) => {
            eprintln!("\x1b[1;31merror:\x1b[0m {}", error);
            std::process::exit(1)
        }
    };

    if pixelation > 0 {
        match libmosh::pixelmosh(&info, &output, pixelation) {
            Ok(()) => (spinner.finish_with_message("\x1b[1;32mDONE\x1b[0m")),
            Err(error) => {
                eprintln!("\x1b[1;31mpixelation error:\x1b[0m {}", error);
                std::process::exit(1)
            }
        }
    }

    spinner.finish_with_message("\x1b[1;32mDONE\x1b[0m");
}

#[cfg(test)]
mod util;
