use rand::distributions::{Distribution, Uniform};
use rand::Rng;

pub struct Options {
    pub min_rate: u16,
    pub max_rate: u16,
    pub line_shift_rng: f64,
    pub channel_swap_rng: f64,
    pub channel_shift_rng: f64,
}

trait Mosh {
    fn run(&self, chunk: &mut [u8]);
}

enum MoshChunk {
    ChannelSwap(usize, usize, usize),
}

enum MoshLine {
    ChannelShift(usize, usize, usize),
    Shift(usize),
}

pub fn mosh(
    png_info: &png::OutputInfo,
    pixel_buf: &mut [u8],
    rng: &mut impl Rng,
    options: &Options,
) {
    let chunk_count_dist = Uniform::from(options.min_rate..=options.max_rate);
    let mosh_rate = chunk_count_dist.sample(rng);

    for _ in 0..mosh_rate {
        mosh_chunk(png_info, pixel_buf, rng, options);
    }
}

fn mosh_chunk(
    png_info: &png::OutputInfo,
    pixel_buf: &mut [u8],
    rng: &mut impl Rng,
    options: &Options,
) {
    let line_count = pixel_buf.len() / png_info.line_size;
    let channel_count = match png_info.color_type {
        png::ColorType::Indexed => 1,
        png::ColorType::Grayscale => 1,
        png::ColorType::GrayscaleAlpha => 2,
        png::ColorType::Rgb => 3,
        png::ColorType::Rgba => 4,
    };

    let line_shift_dist = Uniform::from(0..png_info.line_size);
    let line_number_dist = Uniform::from(0..line_count);
    let channel_count_dist = Uniform::from(0..channel_count);

    let first_line = line_number_dist.sample(rng);
    let chunk_size = line_number_dist.sample(rng) / 2;
    let last_line = if (first_line + chunk_size) > line_count {
        line_count
    } else {
        first_line + chunk_size
    };

    let line_shift = if rng.gen_bool(options.line_shift_rng) {
        let line_shift_amount = line_shift_dist.sample(rng);

        Some(MoshLine::Shift(line_shift_amount))
    } else {
        None
    };

    let shift_channel = if rng.gen_bool(options.channel_shift_rng) {
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
        let line_start = line_number * png_info.line_size;
        let line_end = line_start + png_info.line_size;
        let line = &mut pixel_buf[line_start..line_end];

        if let Some(shift_channel) = &shift_channel {
            shift_channel.run(line);
        }

        if let Some(ls) = &line_shift {
            ls.run(line);
        }
    }

    let chunk_start = first_line * png_info.line_size;
    let chunk_end = last_line * png_info.line_size;
    let chunk = &mut pixel_buf[chunk_start..chunk_end];

    if let Some(cs) = channel_swap {
        cs.run(chunk)
    };
}

impl Mosh for MoshChunk {
    fn run(&self, chunk: &mut [u8]) {
        match self {
            MoshChunk::ChannelSwap(channel_1, channel_2, channel_count) => {
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
        }
    }
}

impl Mosh for MoshLine {
    fn run(&self, line: &mut [u8]) {
        match self {
            MoshLine::ChannelShift(amount, channel, channel_count) => {
                let line_length = line.len();
                let channel_value_count = line_length / channel_count;

                for i in 0..channel_value_count {
                    line[(i * channel_count + channel) % line_length] =
                        line[(i * channel_count + channel + (channel + 1) * amount) % line_length];
                }
            }
            MoshLine::Shift(amount) => {
                line.rotate_left(*amount);
            }
        }
    }
}
