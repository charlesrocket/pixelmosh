use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use indicatif::{ProgressBar, ProgressStyle};

use std::env;
use std::path::PathBuf;

use libmosh::ops::{read_file, write_file};
use libmosh::{mosh, MoshOptions};

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

fn defaults() -> MoshOptions {
    MoshOptions::default()
}

fn arg_matches() -> ArgMatches {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(BANNER)
        .version(env!("CARGO_PKG_VERSION"))
        .help_template("{about-with-newline}PNG corrupter\n\n{usage-heading} {usage}\n\n{all-args}{after-help}")
        .arg(Arg::new("file")
            .action(ArgAction::Set)
            .value_name("FILE")
            .help("File path")
            .required(true).value_parser(value_parser!(PathBuf)))
        .arg(Arg::new("minrate")
            .short('n')
            .long("min-rate")
            .value_name("MIN_RATE")
            .help("Minimum chunks to process")
            .value_parser(value_parser!(u16))
            .default_value(defaults().min_rate.to_string()))
        .arg(Arg::new("maxrate")
            .short('m')
            .long("max-rate")
            .value_name("MAX_RATE")
            .help("Maximum chunks to process")
            .value_parser(value_parser!(u16))
            .default_value(defaults().max_rate.to_string()))
        .arg(Arg::new("pixelation")
            .short('p')
            .long("pixelation")
            .value_name("PIXELATION")
            .help("Pixelation rate")
            .value_parser(value_parser!(u8).range(1..))
            .default_value(defaults().pixelation.to_string()))
        .arg(Arg::new("lineshift")
            .short('l')
            .long("line-shift")
            .value_name("LINE_SHIFT")
            .help("Line shift rate")
            .value_parser(value_parser!(f64))
            .default_value(defaults().line_shift.to_string()))
        .arg(Arg::new("reverse")
            .short('r')
            .long("reverse")
            .value_name("REVERSE")
            .help("Reverse rate")
            .value_parser(value_parser!(f64))
            .default_value(defaults().reverse.to_string()))
        .arg(Arg::new("flip")
            .short('f')
            .long("flip")
            .value_name("FLIP")
            .help("Flip rate")
            .value_parser(value_parser!(f64))
            .default_value(defaults().flip.to_string()))
        .arg(Arg::new("channelswap")
            .short('c')
            .long("channel-swap")
            .value_name("CHANNEL_SWAP")
            .help("Channel swap rate")
            .value_parser(value_parser!(f64))
            .default_value(defaults().channel_swap.to_string()))
        .arg(Arg::new("channelshift")
            .short('t')
            .long("channel-shift")
            .value_name("CHANNEL_SHIFT")
            .help("Channel shift rate")
            .value_parser(value_parser!(f64))
            .default_value(defaults().channel_shift.to_string()))
        .arg(Arg::new("seed")
            .short('s')
            .long("seed")
            .value_name("SEED")
            .help("Random seed")
            .hide_default_value(true)
            .value_parser(value_parser!(u64))
            .default_value(defaults().seed.to_string()))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .value_name("OUTPUT")
            .help("Output filename")
            .hide_default_value(true)
            .default_value("moshed.png"));

    matches.get_matches()
}

fn args() -> (PathBuf, String, MoshOptions) {
    let matches = arg_matches();
    let input = matches.get_one::<PathBuf>("file").unwrap();
    let output = matches.get_one::<String>("output").unwrap();
    let options = MoshOptions {
        min_rate: *matches.get_one::<u16>("minrate").unwrap(),
        max_rate: *matches.get_one::<u16>("maxrate").unwrap(),
        pixelation: *matches.get_one::<u8>("pixelation").unwrap(),
        line_shift: *matches.get_one::<f64>("lineshift").unwrap(),
        reverse: *matches.get_one::<f64>("reverse").unwrap(),
        flip: *matches.get_one::<f64>("flip").unwrap(),
        channel_swap: *matches.get_one::<f64>("channelswap").unwrap(),
        channel_shift: *matches.get_one::<f64>("channelshift").unwrap(),
        seed: *matches.get_one::<u64>("seed").unwrap(),
    };

    (input.clone(), output.to_string(), options)
}

fn cli(input: PathBuf, output: &str, options: &MoshOptions) {
    let spinner = ProgressBar::new_spinner();
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

    let (mut buf, info) = match read_file(input) {
        Ok((buf, info)) => (buf, info),
        Err(error) => {
            eprintln!("\x1b[1;31merror:\x1b[0m {error}");
            std::process::exit(1)
        }
    };

    spinner.set_message("\x1b[94mprocessing\x1b[0m");
    match mosh(&info, &mut buf, options) {
        Ok(image) => image,
        Err(error) => {
            eprintln!("\x1b[1;31merror:\x1b[0m {error}");
            std::process::exit(1)
        }
    };

    spinner.set_message("\x1b[33mwriting output\x1b[0m");
    match write_file(output, &buf, &info) {
        Ok(()) => spinner.finish_with_message("\x1b[1;32mDONE\x1b[0m"),
        Err(error) => {
            eprintln!("\x1b[1;31merror:\x1b[0m {error}");
            std::process::exit(1)
        }
    };
}

pub fn start() {
    let (input, output, options) = args();
    cli(input, &output, &options);
}
