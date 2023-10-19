use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use indicatif::{ProgressBar, ProgressStyle};
use png::ColorType;

use std::{env, path::PathBuf};

use libmosh::{
    ops::{read_file, write_file},
    MoshCore,
};

// Logo
const BANNER: &str = "\u{250C}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\
    \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\
    \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\
    \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\
    \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\
    \u{2500}\u{2500}\u{2500}\u{2510}\n\u{2502} \u{2588}\
    \u{2580}\u{2584} \u{2588} \u{2580}\u{2584}\u{2580} \
    \u{2588}\u{2588}\u{2580} \u{2588}   \u{2588}\u{2584} \
    \u{2584}\u{2588} \u{2584}\u{2580}\u{2584} \u{2584}\
    \u{2580}\u{2580} \u{2588}\u{2584}\u{2588} \u{2502}\n\
    \u{2502} \u{2588}\u{2580}  \u{2588} \u{2588} \u{2588} \
    \u{2588}\u{2584}\u{2584} \u{2588}\u{2584}\u{2584} \
    \u{2588} \u{2580} \u{2588} \u{2580}\u{2584}\u{2580} \
    \u{2584}\u{2588}\u{2588} \u{2588} \u{2588} \u{2502}\n\
    \u{2514}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\
    \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\
    \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\
    \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\
    \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\
    \u{2500}\u{2500}\u{2500}\u{2518}";

// TTY animation
const SPINNER_1: [&str; 7] = [
    "\u{2219}\u{2219}\u{2219}\u{2219}\u{2219}",
    "\u{25CF}\u{2219}\u{2219}\u{2219}\u{2219}",
    "\u{2219}\u{25CF}\u{2219}\u{2219}\u{2219}",
    "\u{2219}\u{2219}\u{25CF}\u{2219}\u{2219}",
    "\u{2219}\u{2219}\u{2219}\u{25CF}\u{2219}",
    "\u{2219}\u{2219}\u{2219}\u{2219}\u{25CF}",
    "\u{2219}\u{2219}\u{2219}\u{2219}\u{2219}",
];

// Terminal animation
const SPINNER_2: [&str; 7] = [
    "\u{25B1}\u{25B1}\u{25B1}\u{25B1}\u{25B1}",
    "\u{25B0}\u{25B1}\u{25B1}\u{25B1}\u{25B1}",
    "\u{25B1}\u{25B0}\u{25B1}\u{25B1}\u{25B1}",
    "\u{25B1}\u{25B1}\u{25B0}\u{25B1}\u{25B1}",
    "\u{25B1}\u{25B1}\u{25B1}\u{25B0}\u{25B1}",
    "\u{25B1}\u{25B1}\u{25B1}\u{25B1}\u{25B0}",
    "\u{25B0}\u{25B0}\u{25B0}\u{25B0}\u{25B0}",
];

// Checks for TTY
fn display_var() -> bool {
    env::var("DISPLAY").is_ok()
}

fn color_type(container: &MoshCore) -> &str {
    match container.data.color_type {
        ColorType::Grayscale => "Grayscale",
        ColorType::Indexed => "Indexed",
        ColorType::GrayscaleAlpha => "Grayscale/A",
        ColorType::Rgb => "RGB",
        ColorType::Rgba => "RGB/A",
    }
}

fn arg_matches() -> (ArgMatches, MoshCore) {
    let container = MoshCore::new();
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(BANNER)
        .version(env!("CARGO_PKG_VERSION"))
        .help_template(
            "{name} v{version} \
                CLI\n{about-with-newline}\
                PNG corrupter\n\n{usage-heading} \
                {usage}\n\n{all-args}{after-help}",
        )
        .arg(
            Arg::new("file")
                .action(ArgAction::Set)
                .value_name("FILE")
                .help("File path")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("minrate")
                .short('n')
                .long("min-rate")
                .value_name("MIN_RATE")
                .help("Minimum chunks to process")
                .value_parser(value_parser!(u16))
                .default_value(container.options.min_rate.to_string()),
        )
        .arg(
            Arg::new("maxrate")
                .short('m')
                .long("max-rate")
                .value_name("MAX_RATE")
                .help("Maximum chunks to process")
                .value_parser(value_parser!(u16))
                .default_value(container.options.max_rate.to_string()),
        )
        .arg(
            Arg::new("pixelation")
                .short('p')
                .long("pixelation")
                .value_name("PIXELATION")
                .help("Pixelation rate")
                .value_parser(value_parser!(u8))
                .default_value(container.options.pixelation.to_string()),
        )
        .arg(
            Arg::new("lineshift")
                .short('l')
                .long("line-shift")
                .value_name("LINE_SHIFT")
                .help("Line shift rate")
                .value_parser(value_parser!(f64))
                .default_value(container.options.line_shift.to_string()),
        )
        .arg(
            Arg::new("reverse")
                .short('r')
                .long("reverse")
                .value_name("REVERSE")
                .help("Reverse rate")
                .value_parser(value_parser!(f64))
                .default_value(container.options.reverse.to_string()),
        )
        .arg(
            Arg::new("flip")
                .short('f')
                .long("flip")
                .value_name("FLIP")
                .help("Flip rate")
                .value_parser(value_parser!(f64))
                .default_value(container.options.flip.to_string()),
        )
        .arg(
            Arg::new("channelswap")
                .short('c')
                .long("channel-swap")
                .value_name("CHANNEL_SWAP")
                .help("Channel swap rate")
                .value_parser(value_parser!(f64))
                .default_value(container.options.channel_swap.to_string()),
        )
        .arg(
            Arg::new("channelshift")
                .short('t')
                .long("channel-shift")
                .value_name("CHANNEL_SHIFT")
                .help("Channel shift rate")
                .value_parser(value_parser!(f64))
                .default_value(container.options.channel_shift.to_string()),
        )
        .arg(
            Arg::new("seed")
                .short('s')
                .long("seed")
                .value_name("SEED")
                .help("Custom seed")
                .long_help("Set a custom seed value")
                .hide_default_value(true)
                .value_parser(value_parser!(u64))
                .default_value(container.options.seed.to_string()),
        )
        .arg(
            Arg::new("batch")
                .short('b')
                .long("batch")
                .value_name("BATCH")
                .help("Number of files to output")
                .long_help("Enable batch mode and set the number of files to output")
                .hide_default_value(true)
                .value_parser(value_parser!(u8))
                .default_value("1"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("OUTPUT")
                .help("Output filename")
                .hide_default_value(true)
                .default_value("moshed"),
        );

    (matches.get_matches(), container)
}

fn args() -> (PathBuf, String, MoshCore, u8) {
    let (matches, mut container) = arg_matches();
    let input = matches.get_one::<PathBuf>("file").unwrap();
    let output = matches.get_one::<String>("output").unwrap();
    let batch = matches.get_one::<u8>("batch").unwrap();

    container.options.min_rate = *matches.get_one::<u16>("minrate").unwrap();
    container.options.max_rate = *matches.get_one::<u16>("maxrate").unwrap();
    container.options.pixelation = *matches.get_one::<u8>("pixelation").unwrap();
    container.options.line_shift = *matches.get_one::<f64>("lineshift").unwrap();
    container.options.reverse = *matches.get_one::<f64>("reverse").unwrap();
    container.options.flip = *matches.get_one::<f64>("flip").unwrap();
    container.options.channel_swap = *matches.get_one::<f64>("channelswap").unwrap();
    container.options.channel_shift = *matches.get_one::<f64>("channelshift").unwrap();
    container.options.seed = *matches.get_one::<u64>("seed").unwrap();

    (input.to_path_buf(), output.to_string(), container, *batch)
}

fn filename(output: &str, index: u8, batch: u8) -> String {
    if batch > 1 {
        format!("{output}-{index:03}.png")
    } else {
        format!("{output}.png")
    }
}

fn cli(input: PathBuf, output: &str, mut container: MoshCore, batch: u8) {
    let mut index = 0;
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

    if let Some(file_name) = input.file_name().and_then(|name| name.to_str()) {
        println!("file: {}", file_name);
    }

    println!("seed: \x1b[3m{}\x1b[0m", &container.options.seed);

    spinner.enable_steady_tick(std::time::Duration::from_millis(90));
    spinner.set_style(ProgressStyle::default_spinner().tick_strings(&spinner_style));
    spinner.set_message("\x1b[36mreading input\x1b[0m");

    let image = match read_file(input) {
        Ok(image) => image,
        Err(error) => {
            spinner.finish_with_message("\x1b[1;31mERROR\x1b[0m");
            eprintln!("{error}");
            std::process::exit(1)
        }
    };

    match container.read_image(&image) {
        Ok(new_image) => new_image,
        Err(error) => {
            spinner.finish_with_message("\x1b[1;31mERROR\x1b[0m");
            eprintln!("{error}");
            std::process::exit(1)
        }
    };

    spinner.println(format!("mode: {}", color_type(&container)));

    for _ in 0..batch {
        spinner.set_message("\x1b[94mprocessing\x1b[0m");

        if let Err(error) = container.mosh() {
            spinner.finish_with_message("\x1b[1;31mERROR\x1b[0m");
            eprintln!("{error}");
            std::process::exit(1);
        }

        index += 1;
        container.options.seed += 1;
        spinner.set_message("\x1b[33mwriting output\x1b[0m");

        if let Err(error) = write_file(
            &filename(output, index, batch),
            &container.data.buf,
            container.data.width,
            container.data.height,
            container.data.color_type,
            container.data.bit_depth,
        ) {
            spinner.finish_with_message("\x1b[1;31mERROR\x1b[0m");
            eprintln!("{error}");
            std::process::exit(1);
        }
    }

    spinner.finish_with_message("\x1b[1;32mDONE\x1b[0m");
}

pub fn start() {
    let (input, output, container, series) = args();
    cli(input, &output, container, series);
}
