use image::imageops::Nearest;
use image::ImageError;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;

pub const INTRO: &str = "┌─────────────────────────────────────┐\n\
                         │ █▀▄ █ ▀▄▀ ██▀ █   █▄ ▄█ ▄▀▄ ▄▀▀ █▄█ │\n\
                         │ █▀  █ █ █ █▄▄ █▄▄ █ ▀ █ ▀▄▀ ▄██ █ █ │\n\
                         └─────────────────────────────────────┘";

pub const SPINNER_1: [&str; 7] = [
    "∙∙∙∙∙",
    "●∙∙∙∙",
    "∙●∙∙∙",
    "∙∙●∙∙",
    "∙∙∙●∙",
    "∙∙∙∙●",
    "∙∙∙∙∙",
];

pub const SPINNER_2: [&str; 7] = [
    "▱▱▱▱▱",
    "▰▱▱▱▱",
    "▱▰▱▱▱",
    "▱▱▰▱▱",
    "▱▱▱▰▱",
    "▱▱▱▱▰",
    "▰▰▰▰▰",
];

pub struct Options {
    pub min_rate: u16,
    pub max_rate: u16,
    pub line_shift_rng: f64,
    pub reverse_rng: f64,
    pub flip_rng: f64,
    pub channel_swap_rng: f64,
    pub channel_shift_rng: f64,
}

trait Mosh {
    fn run(&self, chunk: &mut [u8]);
}

enum MoshChunk {
    ChannelSwap(usize, usize, usize),
    Flip,
}

enum MoshLine {
    ChannelShift(usize, usize, usize),
    Shift(usize),
    Reverse,
}

/// # Errors
/// TODO
pub fn pixelmosh(info: &png::OutputInfo, input: &String, rate: u32) -> Result<(), ImageError> {
    let mut img = image::io::Reader::open(input)?.decode()?;
    let (w1, h1) = (info.width / rate, info.height / rate);
    let (w2, h2) = (w1 * rate, h1 * rate);

    img = img.resize(w1, h1, Nearest);
    img = img.resize(w2, h2, Nearest);
    img.save(input)?;

    Ok(())
}

pub fn mosh(
    image_inf: &png::OutputInfo,
    pixel_buf: &mut [u8],
    rng: &mut impl Rng,
    options: &Options,
) {
    let chunk_count_dist = Uniform::from(options.min_rate..=options.max_rate);
    let mosh_rate = chunk_count_dist.sample(rng);

    for _ in 0..mosh_rate {
        chunkmosh(image_inf, pixel_buf, rng, options);
    }
}

fn chunkmosh(
    image_inf: &png::OutputInfo,
    pixel_buf: &mut [u8],
    rng: &mut impl Rng,
    options: &Options,
) {
    let line_count = pixel_buf.len() / image_inf.line_size;
    let channel_count = match image_inf.color_type {
        png::ColorType::Grayscale | png::ColorType::Indexed => 1,
        png::ColorType::GrayscaleAlpha => 2,
        png::ColorType::Rgb => 3,
        png::ColorType::Rgba => 4,
    };

    let line_shift_dist = Uniform::from(0..image_inf.line_size);
    let line_number_dist = Uniform::from(0..line_count);
    let channel_count_dist = Uniform::from(0..channel_count);

    let first_line = line_number_dist.sample(rng);
    let chunk_size = line_number_dist.sample(rng) / 2;
    let last_line = if (first_line + chunk_size) > line_count {
        line_count
    } else {
        first_line + chunk_size
    };

    let reverse = rng.gen_bool(options.reverse_rng);
    let flip = rng.gen_bool(options.flip_rng);

    let line_shift = if rng.gen_bool(options.line_shift_rng) {
        let line_shift_amount = line_shift_dist.sample(rng);

        Some(MoshLine::Shift(line_shift_amount))
    } else {
        None
    };

    let channel_shift = if rng.gen_bool(options.channel_shift_rng) {
        let amount = line_shift_dist.sample(rng) / channel_count;
        let channel = channel_count_dist.sample(rng);

        Some(MoshLine::ChannelShift(amount, channel, channel_count))
    } else {
        None
    };

    let channel_swap = if rng.gen_bool(options.channel_swap_rng) {
        let channel_1 = channel_count_dist.sample(rng);
        let channel_2 = channel_count_dist.sample(rng);

        Some(MoshChunk::ChannelSwap(channel_1, channel_2, channel_count))
    } else {
        None
    };

    for line_number in first_line..last_line {
        let line_start = line_number * image_inf.line_size;
        let line_end = line_start + image_inf.line_size;
        let line = &mut pixel_buf[line_start..line_end];

        if let Some(channel_shift) = &channel_shift {
            channel_shift.run(line);
        }

        if let Some(ls) = &line_shift {
            ls.run(line);
        }
        if reverse {
            MoshLine::Reverse.run(line);
        }
    }

    let chunk_start = first_line * image_inf.line_size;
    let chunk_end = last_line * image_inf.line_size;
    let chunk = &mut pixel_buf[chunk_start..chunk_end];

    if let Some(cs) = channel_swap {
        cs.run(chunk);
    };

    if flip {
        MoshChunk::Flip.run(chunk);
    };
}

impl Mosh for MoshChunk {
    fn run(&self, chunk: &mut [u8]) {
        match self {
            Self::ChannelSwap(channel_1, channel_2, channel_count) => {
                let chunk_length = chunk.len();
                let channel_value_count = chunk_length / channel_count;

                for i in 0..channel_value_count {
                    let channel_1_index = (i * channel_count) + channel_1;
                    let channel_2_index = (i * channel_count) + channel_2;
                    let channel_1_value = chunk[channel_1_index];
                    let channel_2_value = chunk[channel_2_index];

                    chunk[channel_1_index] = channel_2_value;
                    chunk[channel_2_index] = channel_1_value;
                }
            }

            Self::Flip => {
                chunk.reverse();
            }
        }
    }
}

impl Mosh for MoshLine {
    fn run(&self, line: &mut [u8]) {
        match self {
            Self::ChannelShift(amount, channel, channel_count) => {
                let line_length = line.len();
                let channel_value_count = line_length / channel_count;

                for i in 0..channel_value_count {
                    line[(i * channel_count + channel) % line_length] =
                        line[(i * channel_count + channel + (channel + 1) * amount) % line_length];
                }
            }

            Self::Shift(amount) => {
                line.rotate_left(*amount);
            }

            Self::Reverse => {
                line.reverse();
            }
        }
    }
}
