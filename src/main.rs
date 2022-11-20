use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};

use std::env;

use libmosh::{ops, MoshOptions};

#[derive(Parser, Debug)]
#[command(before_help = BANNER, about, author, version, long_about = None)]
struct Args {
    #[arg(display_order = 1, help = "File path")]
    file: String,

    #[arg(short, long, display_order = 2, help = "Minimum chunks to process",
        default_value_t = MoshOptions::default().min_rate
    )]
    min_rate: u16,

    #[arg(short = 'n', long, display_order = 3, help = "Maximum chunks to process",
        default_value_t = MoshOptions::default().max_rate
    )]
    max_rate: u16,

    #[arg(short, long, display_order = 4, help = "Pixelation rate",
        default_value_t = MoshOptions::default().pixelation
    )]
    pixelation: u8,

    #[arg(short, long, display_order = 5, help = "Line shift rate",
        default_value_t = MoshOptions::default().line_shift
    )]
    line_shift: f64,

    #[arg(short, long, display_order = 6, help = "Reverse rate",
        default_value_t = MoshOptions::default().reverse
    )]
    reverse: f64,

    #[arg(short, long, display_order = 7, help = "Flip rate",
        default_value_t = MoshOptions::default().flip
    )]
    flip: f64,

    #[arg(short, long, display_order = 8, help = "Channel swap rate",
        default_value_t = MoshOptions::default().channel_swap
    )]
    channel_swap: f64,

    #[arg(short = 't', long, display_order = 9, help = "Channel shift rate",
        default_value_t = MoshOptions::default().channel_shift
    )]
    channel_shift: f64,

    #[arg(short, long, display_order = 10, help = "Random seed",
        default_value_t = MoshOptions::default().seed,
        hide_default_value = true
    )]
    seed: u64,

    #[arg(
        short,
        long,
        display_order = 11,
        help = "Output file",
        default_value = "moshed.png"
    )]
    output: String,
}

/// Logo
const BANNER: &str = "┌─────────────────────────────────────┐\n\
                      │ █▀▄ █ ▀▄▀ ██▀ █   █▄ ▄█ ▄▀▄ ▄▀▀ █▄█ │\n\
                      │ █▀  █ █ █ █▄▄ █▄▄ █ ▀ █ ▀▄▀ ▄██ █ █ │\n\
                      └─────────────────────────────────────┘";

/// TTY animation
const SPINNER_1: [&str; 7] = [
    "∙∙∙∙∙",
    "●∙∙∙∙",
    "∙●∙∙∙",
    "∙∙●∙∙",
    "∙∙∙●∙",
    "∙∙∙∙●",
    "∙∙∙∙∙",
];

/// Terminal animation
const SPINNER_2: [&str; 7] = [
    "▱▱▱▱▱",
    "▰▱▱▱▱",
    "▱▰▱▱▱",
    "▱▱▰▱▱",
    "▱▱▱▰▱",
    "▱▱▱▱▰",
    "▰▰▰▰▰",
];

/// Checks for TTY
///
/// Returns `true` if a terminal emulator is detected.
#[must_use]
fn display_var() -> bool {
    matches!(env::var("DISPLAY"), Ok(_))
}

fn main() {
    let spinner = ProgressBar::new_spinner();
    let args = Args::parse();
    let output = args.output;
    let options = MoshOptions {
        min_rate: args.min_rate,
        max_rate: args.max_rate,
        pixelation: args.pixelation.clamp(1, 255),
        line_shift: args.line_shift,
        reverse: args.reverse,
        flip: args.flip,
        channel_swap: args.channel_swap,
        channel_shift: args.channel_shift,
        seed: args.seed,
    };

    let spinner_style = if cfg!(unix) {
        if display_var() | cfg!(target_os = "macos") {
            SPINNER_2
        } else {
            SPINNER_1
        }
    } else {
        SPINNER_1
    };

    println!("file: {}", args.file);
    println!("seed: \x1b[3m{}\x1b[0m", args.seed);

    spinner.enable_steady_tick(std::time::Duration::from_millis(90));
    spinner.set_style(ProgressStyle::default_spinner().tick_strings(&spinner_style));
    spinner.set_message("\x1b[36mreading input\x1b[0m");

    let (mut buf, info) = match ops::read_file(args.file) {
        Ok((buf, info)) => (buf, info),
        Err(error) => {
            eprintln!("\x1b[1;31merror:\x1b[0m {error}");
            std::process::exit(1)
        }
    };

    spinner.set_message("\x1b[94mprocessing\x1b[0m");
    match libmosh::mosh(&info, &mut buf, &options) {
        Ok(image) => image,
        Err(error) => {
            eprintln!("\x1b[1;31merror:\x1b[0m {error}");
            std::process::exit(1)
        }
    };

    spinner.set_message("\x1b[33mwriting output\x1b[0m");
    match ops::write_file(&output, &buf, &info) {
        Ok(()) => spinner.finish_with_message("\x1b[1;32mDONE\x1b[0m"),
        Err(error) => {
            eprintln!("\x1b[1;31merror:\x1b[0m {error}");
            std::process::exit(1)
        }
    };
}
