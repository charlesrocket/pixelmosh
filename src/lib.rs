//! # libmosh
//!
//! Glitch and pixelate PNG images.

use png::{ColorType, OutputInfo};
use rand::distributions::{Distribution, Uniform};
use rand::{RngCore, SeedableRng};
use resize::Pixel::{Gray8, RGB8, RGBA8};
use resize::Type::Point;
use rgb::FromSlice;

use crate::err::MoshError;
use crate::fx::{Mosh, MoshChunk, MoshLine};

pub mod err;
pub mod fx;
pub mod ops;

/// Processing options
///
/// Minimal `pixelation` value is `1` (OFF).
pub struct MoshOptions {
    /// Minimal amount of chunks to process.
    pub min_rate: u16,
    /// Maximal amount of chunks to process.
    pub max_rate: u16,
    /// Pixelation's intensity.
    pub pixelation: u8,
    /// Chance of line shift.
    pub line_shift: f64,
    /// Chance of reverse.
    pub reverse: f64,
    /// Chance of flip.
    pub flip: f64,
    /// Chance of channel swap.
    pub channel_swap: f64,
    /// Chance of channel shift.
    pub channel_shift: f64,
    /// Random seed.
    pub seed: u64,
}

impl Default for MoshOptions {
    fn default() -> Self {
        Self {
            min_rate: 1,
            max_rate: 7,
            pixelation: 10,
            line_shift: 0.3,
            reverse: 0.3,
            flip: 0.3,
            channel_swap: 0.3,
            channel_shift: 0.3,
            seed: if cfg!(test) {
                901_042_006
            } else {
                rand::thread_rng().next_u64()
            },
        }
    }
}

/// Processes provided image data
///
/// # Errors
///
/// * [`InvalidParameters`]: e.g. when pixelation value is more than `image_info.width`.
/// * [`OutOfMemory`]: `resize` may run out of memory.
/// * [`UnsupportedColorType`]: `ColorType::GrayscaleAlpha` is not supported.
///
/// # Example
/// ````
/// use std::fs::File;
///
/// let options = libmosh::MoshOptions {
///     min_rate: 5,
///     max_rate: 7,
///     pixelation: 10,
///     line_shift: 0.7,
///     reverse: 0.4,
///     flip: 0.3,
///     channel_swap: 0.5,
///     channel_shift: 0.5,
///     seed: 42,
/// };
///
/// let decoder = png::Decoder::new(File::open("example/delorean.png").unwrap());
/// let mut reader = decoder.read_info().unwrap();
/// let mut buf = vec![0; reader.output_buffer_size()];
/// let info = reader.next_frame(&mut buf).unwrap();
///
/// libmosh::mosh(&info, &mut buf, &options).unwrap();
/// ````
///
/// [`InvalidParameters`]: crate::err::MoshError::InvalidParameters
/// [`OutOfMemory`]: crate::err::MoshError::OutOfMemory
/// [`UnsupportedColorType`]: crate::err::MoshError::UnsupportedColorType
pub fn mosh(
    image_info: &OutputInfo,
    pixel_buffer: &mut [u8],
    options: &MoshOptions,
) -> Result<(), MoshError> {
    let (w1, h1) = (image_info.width as usize, image_info.height as usize);
    let (w2, h2) = (
        w1 / options.pixelation as usize,
        h1 / options.pixelation as usize,
    );

    let mut dest = vec![0u8; w2 * h2 * image_info.color_type.samples()];
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(options.seed);

    let chunk_count_dist = Uniform::from(options.min_rate..=options.max_rate);
    let mosh_rate = chunk_count_dist.sample(&mut rng);

    for _ in 0..mosh_rate {
        chunkmosh(image_info, pixel_buffer, &mut rng, options);
    }

    if options.pixelation > 1 {
        match image_info.color_type {
            ColorType::GrayscaleAlpha | ColorType::Indexed => {}
            ColorType::Grayscale => {
                resize::new(w1, h1, w2, h2, Gray8, Point)?
                    .resize(pixel_buffer.as_gray(), dest.as_gray_mut())?;
            }

            ColorType::Rgb => {
                resize::new(w1, h1, w2, h2, RGB8, Point)?
                    .resize(pixel_buffer.as_rgb(), dest.as_rgb_mut())?;
            }

            ColorType::Rgba => {
                resize::new(w1, h1, w2, h2, RGBA8, Point)?
                    .resize(pixel_buffer.as_rgba(), dest.as_rgba_mut())?;
            }
        };

        match image_info.color_type {
            ColorType::GrayscaleAlpha | ColorType::Indexed => {
                return Err(MoshError::UnsupportedColorType);
            }

            ColorType::Grayscale => {
                resize::new(w2, h2, w1, h1, Gray8, Point)?
                    .resize(dest.as_gray(), pixel_buffer.as_gray_mut())?;
            }

            ColorType::Rgb => {
                resize::new(w2, h2, w1, h1, RGB8, Point)?
                    .resize(dest.as_rgb(), pixel_buffer.as_rgb_mut())?;
            }

            ColorType::Rgba => {
                resize::new(w2, h2, w1, h1, RGBA8, Point)?
                    .resize(dest.as_rgba(), pixel_buffer.as_rgba_mut())?;
            }
        };
    }

    Ok(())
}

// Use pnglitch approach
//
// TODO
// Add more `rng` to `chunk_size`?
fn chunkmosh(
    image_info: &OutputInfo,
    pixel_buffer: &mut [u8],
    rng: &mut impl rand::Rng,
    options: &MoshOptions,
) {
    let line_count = pixel_buffer.len() / image_info.line_size;
    let channel_count = match image_info.color_type {
        ColorType::Grayscale | ColorType::Indexed => 1,
        ColorType::GrayscaleAlpha => 2,
        ColorType::Rgb => 3,
        ColorType::Rgba => 4,
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

    let reverse = rng.gen_bool(options.reverse);
    let flip = rng.gen_bool(options.flip);

    let line_shift = if rng.gen_bool(options.line_shift) {
        let line_shift_amount = line_shift_dist.sample(rng);

        Some(MoshLine::Shift(line_shift_amount))
    } else {
        None
    };

    let channel_shift = if rng.gen_bool(options.channel_shift) {
        let amount = line_shift_dist.sample(rng) / channel_count;
        let channel = channel_count_dist.sample(rng);

        Some(MoshLine::ChannelShift(amount, channel, channel_count))
    } else {
        None
    };

    let channel_swap = if rng.gen_bool(options.channel_swap) {
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

#[cfg(test)]
mod util;
