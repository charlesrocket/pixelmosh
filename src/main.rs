use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};

use libmosh::{cli, ops, Options as MoshOptions};

#[derive(Parser, Debug)]
#[clap(version, author = cli::BANNER, about, long_about = None)]
struct Args {
    #[clap(required = true, display_order = 1)]
    file: String,

    #[clap(short, long, display_order = 2,
        default_value_t = MoshOptions::default().min_rate
    )]
    min_rate: u16,

    #[clap(short = 'n', long, display_order = 3,
        default_value_t = MoshOptions::default().max_rate
    )]
    max_rate: u16,

    #[clap(short, long, display_order = 4,
        default_value_t = MoshOptions::default().pixelation
    )]
    pixelation: u8,

    #[clap(short, long, display_order = 5,
        default_value_t = MoshOptions::default().line_shift_rng
    )]
    line_shift: f64,

    #[clap(short, long, display_order = 6,
        default_value_t = MoshOptions::default().reverse_rng
    )]
    reverse: f64,

    #[clap(short, long, display_order = 7,
        default_value_t = MoshOptions::default().flip_rng
    )]
    flip: f64,

    #[clap(short, long, display_order = 8,
        default_value_t = MoshOptions::default().channel_swap_rng
    )]
    channel_swap: f64,

    #[clap(short = 't', long, display_order = 9,
        default_value_t = MoshOptions::default().channel_shift_rng
    )]
    channel_shift: f64,

    #[clap(short, long, display_order = 10,
        default_value_t = MoshOptions::default().seed,
        hide_default_value = true
    )]
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
    let seed = args.seed;
    let pixelation = if args.pixelation == 0 {
        1
    } else {
        args.pixelation
    };

    let options = MoshOptions {
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

    let spinner_style = if cfg!(unix) {
        if cli::display_var() | cfg!(target_os = "macos") {
            cli::SPINNER_2
        } else {
            cli::SPINNER_1
        }
    } else {
        cli::SPINNER_1
    };

    println!("file: {}", args.file);
    println!("seed: \x1b[3m{}\x1b[0m", seed);

    spinner.enable_steady_tick(std::time::Duration::from_millis(90));
    spinner.set_style(ProgressStyle::default_spinner().tick_strings(&spinner_style));
    spinner.set_message("\x1b[36mreading input\x1b[0m");

    let (mut buf, info) = match ops::read_file(args.file) {
        Ok((buf, info)) => (buf, info),
        Err(error) => {
            eprintln!("\x1b[1;31merror:\x1b[0m {}", error);
            std::process::exit(1)
        }
    };

    spinner.set_message("\x1b[94mprocessing\x1b[0m");
    match libmosh::mosh(&info, &mut buf, &options) {
        Ok(image) => (image),
        Err(error) => {
            eprintln!("\x1b[1;31merror:\x1b[0m {}", error);
            std::process::exit(1)
        }
    };

    spinner.set_message("\x1b[33mwriting output\x1b[0m");
    match ops::write_file(&output, &buf, &info) {
        Ok(()) => (spinner.finish_with_message("\x1b[1;32mDONE\x1b[0m")),
        Err(error) => {
            eprintln!("\x1b[1;31merror:\x1b[0m {}", error);
            std::process::exit(1)
        }
    };
}
