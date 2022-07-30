//! # libmosh
//!
//! Glitch and pixelate PNG images.

use rand::distributions::{Distribution, Uniform};
use resize::Pixel::{Gray8, RGB8, RGBA8};
use resize::Type::Point;
use rgb::FromSlice;

/// Processing options
#[derive(Debug, Clone, Copy)]
pub struct Options {
    /// Minimal amount of chunks to process.
    pub min_rate: u16,
    /// Maximal amount of chunks to process.
    pub max_rate: u16,
    /// Chance of line shift.
    pub line_shift_rng: f64,
    /// Chance of reverse.
    pub reverse_rng: f64,
    /// Chance of flip.
    pub flip_rng: f64,
    /// Chance of channel swap.
    pub channel_swap_rng: f64,
    /// Chance of channel shift.
    pub channel_shift_rng: f64,
    /// Pixelation's intensity (`1` to OFF).
    pub pixelation: u8,
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

/// Processes provided image data.
///
/// # Errors
/// Pixelation may run out of memory.
///
/// # Example
/// ````
/// use std::fs::File;
/// use rand_chacha::ChaCha8Rng;
/// use rand_chacha::rand_core::SeedableRng;
///
/// let min_rate = 5;
/// let max_rate = 7;
/// let line_shift_rng = 0.8;
/// let reverse_rng = 0.4;
/// let flip_rng = 0.3;
/// let channel_swap_rng = 0.9;
/// let channel_shift_rng = 0.5;
/// let pixelation = 10;
/// let options = libmosh::Options {
///     min_rate,
///     max_rate,
///     line_shift_rng,
///     reverse_rng,
///     flip_rng,
///     channel_swap_rng,
///     channel_shift_rng,
///     pixelation,
/// };
///
/// let mut rng = ChaCha8Rng::seed_from_u64(42);
/// let decoder = png::Decoder::new(File::open("example/delorean.png").unwrap());
/// let mut reader = decoder.read_info().unwrap();
/// let mut buf = vec![0; reader.output_buffer_size()];
/// let info = reader.next_frame(&mut buf).unwrap();
///
/// libmosh::mosh(&info, &mut buf, &mut rng, &options).unwrap();
/// ````
pub fn mosh(
    image_info: &png::OutputInfo,
    pixel_buffer: &mut [u8],
    rng: &mut impl rand::Rng,
    options: &Options,
) -> Result<(), resize::Error> {
    let (w1, h1) = (image_info.width as usize, image_info.height as usize);
    let (w2, h2) = (
        w1 / options.pixelation as usize,
        h1 / options.pixelation as usize,
    );

    let chunk_count_dist = Uniform::from(options.min_rate..=options.max_rate);
    let mosh_rate = chunk_count_dist.sample(rng);
    let mut dest = vec![0u8; w2 * h2 * image_info.color_type.samples()];

    for _ in 0..mosh_rate {
        chunkmosh(image_info, pixel_buffer, rng, options);
    }

    match image_info.color_type {
        png::ColorType::Grayscale => {
            resize::new(w1, h1, w2, h2, Gray8, Point)?
                .resize(pixel_buffer.as_gray(), dest.as_gray_mut())?;
        }

        png::ColorType::GrayscaleAlpha | png::ColorType::Indexed => {
            eprintln!("\x1b[1;31merror:\x1b[0m Unsupported color type");
            std::process::exit(1)
        }

        png::ColorType::Rgb => {
            resize::new(w1, h1, w2, h2, RGB8, Point)?
                .resize(pixel_buffer.as_rgb(), dest.as_rgb_mut())?;
        }

        png::ColorType::Rgba => {
            resize::new(w1, h1, w2, h2, RGBA8, Point)?
                .resize(pixel_buffer.as_rgba(), dest.as_rgba_mut())?;
        }
    };

    match image_info.color_type {
        png::ColorType::Grayscale => {
            resize::new(w2, h2, w1, h1, Gray8, Point)?
                .resize(dest.as_gray(), pixel_buffer.as_gray_mut())?;
        }

        png::ColorType::GrayscaleAlpha | png::ColorType::Indexed => {
            eprintln!("\x1b[1;31merror:\x1b[0m Unsupported color type");
            std::process::exit(1)
        }

        png::ColorType::Rgb => {
            resize::new(w2, h2, w1, h1, RGB8, Point)?
                .resize(dest.as_rgb(), pixel_buffer.as_rgb_mut())?;
        }

        png::ColorType::Rgba => {
            resize::new(w2, h2, w1, h1, RGBA8, Point)?
                .resize(dest.as_rgba(), pixel_buffer.as_rgba_mut())?;
        }
    };

    Ok(())
}

// Use pnglitch approach
//
// TODO
// Add more `rng` to `chunk_size`?
fn chunkmosh(
    image_info: &png::OutputInfo,
    pixel_buffer: &mut [u8],
    rng: &mut impl rand::Rng,
    options: &Options,
) {
    let line_count = pixel_buffer.len() / image_info.line_size;
    let channel_count = match image_info.color_type {
        png::ColorType::Grayscale | png::ColorType::Indexed => 1,
        png::ColorType::GrayscaleAlpha => 2,
        png::ColorType::Rgb => 3,
        png::ColorType::Rgba => 4,
    };

    let line_shift_dist = Uniform::from(0..image_info.line_size);
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
        let line_start = line_number * image_info.line_size;
        let line_end = line_start + image_info.line_size;
        let line = &mut pixel_buffer[line_start..line_end];

        if let Some(channel_shift) = &channel_shift {
            channel_shift.run(line);
        }

        if let Some(line_shift) = &line_shift {
            line_shift.run(line);
        }
        if reverse {
            MoshLine::Reverse.run(line);
        }
    }

    let chunk_start = first_line * image_info.line_size;
    let chunk_end = last_line * image_info.line_size;
    let chunk = &mut pixel_buffer[chunk_start..chunk_end];

    if let Some(channel_swap) = channel_swap {
        channel_swap.run(chunk);
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
