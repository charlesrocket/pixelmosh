use clap::{value_parser, Arg, ArgAction, Command};
use indicatif::{ProgressBar, ProgressStyle};

use std::env;
use std::path::PathBuf;

use libmosh::{ops, MoshOptions};

// Logo
const BANNER: &str = "┌─────────────────────────────────────┐\n\
                      │ █▀▄ █ ▀▄▀ ██▀ █   █▄ ▄█ ▄▀▄ ▄▀▀ █▄█ │\n\
                      │ █▀  █ █ █ █▄▄ █▄▄ █ ▀ █ ▀▄▀ ▄██ █ █ │\n\
                      └─────────────────────────────────────┘";

// TTY animation
const SPINNER_1: [&str; 7] = [
    "∙∙∙∙∙",
    "●∙∙∙∙",
    "∙●∙∙∙",
    "∙∙●∙∙",
    "∙∙∙●∙",
    "∙∙∙∙●",
    "∙∙∙∙∙",
];

// Terminal animation
const SPINNER_2: [&str; 7] = [
    "▱▱▱▱▱",
    "▰▱▱▱▱",
    "▱▰▱▱▱",
    "▱▱▰▱▱",
    "▱▱▱▰▱",
    "▱▱▱▱▰",
    "▰▰▰▰▰",
];

// Checks for TTY
#[must_use]
fn display_var() -> bool {
    matches!(env::var("DISPLAY"), Ok(_))
}

fn main() {
    let defaults = MoshOptions::default();
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(BANNER)
        .version(env!("CARGO_PKG_VERSION"))
        .help_template("{about-with-newline}PNG corrupter\n\n{usage-heading} {usage}\n\n{all-args}{after-help}")
        .arg(Arg::new("file")
            .action(ArgAction::Set)
            .value_name("FILE")
            .help("Path to target")
            .required(true).value_parser(value_parser!(PathBuf)),)
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .value_name("OUTPUT")
            .help("Output filename")
            .default_value("moshed.png"),)
        .arg(Arg::new("min-rate")
            .short('n')
            .long("min-rate")
            .value_name("MIN RATE")
            .help("Minimum chunks to process")
            .value_parser(value_parser!(u16)),)
        .arg(Arg::new("max-rate")
            .short('m')
            .long("max-rate")
            .value_name("MAX RATE")
            .help("Maximum chunks to process")
            .value_parser(value_parser!(u16)),)
        .arg(Arg::new("pixelation")
            .short('p')
            .long("pixelation")
            .value_name("PIXELATION")
            .help("Pixelation rate")
            .value_parser(value_parser!(u8)),)
        .arg(Arg::new("line-shift")
            .short('l')
            .long("line-shift")
            .value_name("LINE SHIFT")
            .help("Line shift rate")
            .value_parser(value_parser!(f64)),)
        .arg(Arg::new("reverse")
            .short('r')
            .long("reverse")
            .value_name("REVERSE")
            .help("Reverse rate")
            .value_parser(value_parser!(f64)),)
        .arg(Arg::new("flip")
            .short('f')
            .long("flip")
            .value_name("FLIP")
            .help("Flip rate")
            .value_parser(value_parser!(f64)),)
        .arg(Arg::new("channel-swap")
            .short('c')
            .long("channel-swap")
            .value_name("CHANNEL SWAP")
            .help("Channel swap rate")
            .value_parser(value_parser!(f64)),)
        .arg(Arg::new("channel-shift")
            .short('t')
            .long("channel-shift")
            .value_name("CHANNEL SHIFT")
            .help("Channel shift rate")
            .value_parser(value_parser!(f64)),)
        .arg(Arg::new("seed")
            .short('s')
            .long("seed")
            .value_name("SEED")
            .help("Random seed")
            .value_parser(value_parser!(u64)),)
        .get_matches();

    let spinner = ProgressBar::new_spinner();
    let input = matches.get_one::<PathBuf>("file").unwrap();
    let output = matches.get_one::<String>("output").unwrap();
    let options = MoshOptions {
        min_rate: *matches
            .get_one::<u16>("min-rate")
            .unwrap_or(&defaults.min_rate),
        max_rate: *matches
            .get_one::<u16>("max-rate")
            .unwrap_or(&defaults.max_rate),
        pixelation: *matches
            .get_one::<u8>("pixelation")
            .unwrap_or(&defaults.pixelation),
        line_shift: *matches
            .get_one::<f64>("line-shift")
            .unwrap_or(&defaults.line_shift),
        reverse: *matches
            .get_one::<f64>("reverse")
            .unwrap_or(&defaults.reverse),
        flip: *matches.get_one::<f64>("flip").unwrap_or(&defaults.flip),
        channel_swap: *matches
            .get_one::<f64>("channel-swap")
            .unwrap_or(&defaults.channel_swap),
        channel_shift: *matches
            .get_one::<f64>("channel-shift")
            .unwrap_or(&defaults.channel_shift),
        seed: *matches.get_one::<u64>("seed").unwrap_or(&defaults.seed),
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

    println!("file: {}", &input.display());
    println!("seed: \x1b[3m{}\x1b[0m", options.seed);

    spinner.enable_steady_tick(std::time::Duration::from_millis(90));
    spinner.set_style(ProgressStyle::default_spinner().tick_strings(&spinner_style));
    spinner.set_message("\x1b[36mreading input\x1b[0m");

    let (mut buf, info) = match ops::read_file(input) {
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
    match ops::write_file(output, &buf, &info) {
        Ok(()) => spinner.finish_with_message("\x1b[1;32mDONE\x1b[0m"),
        Err(error) => {
            eprintln!("\x1b[1;31merror:\x1b[0m {error}");
            std::process::exit(1)
        }
    };
}
