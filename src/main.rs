use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use rand::thread_rng;
use clap::Parser;

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
}

fn main() {
    let args = Args::parse();
    let mut rng = thread_rng();

    println!("Reading input");
    let decoder = png::Decoder::new(File::open(args.file).expect("File not found"));
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();

    let path = Path::new("moshed.png");
    let output = File::create(path).unwrap();
    let b_writer = &mut BufWriter::new(output);

    let mut encoder = png::Encoder::new(b_writer, info.width, info.height);
    encoder.set_color(info.color_type);
    encoder.set_depth(info.bit_depth);
    let mut writer = encoder.write_header().unwrap();

    let min_rate = args.min_rate;
    let max_rate = args.max_rate;
    let line_shift_rng = args.line_shift;
    let channel_swap_rng = args.channel_swap;
    let channel_shift_rng = args.channel_shift;

    let options = engine::Options {
        min_rate,
        max_rate,
        line_shift_rng,
        channel_swap_rng,
        channel_shift_rng,
    };

    println!("Processing");
    engine::mosh(&info, &mut buf, &mut rng, &options);
    println!("Writing output");
    writer.write_image_data(&buf).unwrap();
}
