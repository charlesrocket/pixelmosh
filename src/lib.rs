/*! # Overview

_Glitch and pixelate PNG images_

Provides the [`MoshCore`] type for image processing and I/O functions,
available in the [`ops`] module.

# Usage
Add `pixelmosh` to your dependencies in your project's `Cargo.toml`.

```toml
[dependencies]
pixelmosh = { version = "3.1", default-features = false }
```

# Example
```rust
use libmosh::{
    err::MoshError,
    ops::{read_file, write_file},
    MoshCore,
};

let input = read_file("src/util/test-rgb.png")?;
let output = "test.png";
let mut core = MoshCore::new();

core.read_image(&input)?;
core.mosh()?;
write_file(
    output,
    &core.data.buf,
    core.data.width,
    core.data.height,
    core.data.color_type,
    core.data.bit_depth,
)?;
# Ok::<(), MoshError>(())
```
*/
#![allow(deprecated)]

use fast_image_resize as fr;
use png::{BitDepth, ColorType, Decoder};
use rand::{
    distributions::{Distribution, Uniform},
    RngCore, SeedableRng,
};

use std::{cmp, num::NonZeroU32};

use crate::{
    err::MoshError,
    fx::{Mosh, MoshChunk, MoshLine},
};

pub mod err;
pub mod fx;
pub mod ops;

/// Image data.
///
/// It holds the original image, buffer and parameters.
#[derive(Clone)]
pub struct MoshData {
    /// Buffer.
    pub buf: Vec<u8>,
    /// Original image.
    pub image: Vec<u8>,
    /// Width.
    pub width: u32,
    /// Height.
    pub height: u32,
    /// Color type.
    pub color_type: ColorType,
    /// Bit depth.
    pub bit_depth: BitDepth,
    /// Line size.
    pub line_size: usize,
}

/// Processing options.
///
/// Minimal `pixelation` value is `1` (OFF).
#[derive(Clone, Debug)]
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

/// Core container.
///
/// Holds image data and processing options.
#[derive(Clone, Default)]
pub struct MoshCore {
    pub data: MoshData,
    pub options: MoshOptions,
}

impl MoshCore {
    /// Creates a new, empty instance of [`MoshCore`] with a random [seed].
    ///
    /// [seed]: MoshOptions::seed
    pub fn new() -> Self {
        Self {
            data: MoshData::default(),
            options: MoshOptions::default(),
        }
    }

    /// Reads provided image for future processing.
    ///
    /// # Errors
    ///
    /// It may fail if an image is not a valid PNG file.
    pub fn read_image(&mut self, input: &[u8]) -> Result<(), MoshError> {
        let decoder = Decoder::new(input);
        let mut reader = decoder.read_info()?;
        let mut buf = vec![0_u8; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf)?;

        self.data.buf = buf.clone();
        self.data.image = buf;
        self.data.width = info.width;
        self.data.height = info.height;
        self.data.color_type = info.color_type;
        self.data.bit_depth = info.bit_depth;
        self.data.line_size = info.line_size;

        Ok(())
    }

    /**
    Processes an image with current [settings], storing the result in a [buffer].

    [buffer]: MoshData::buf
    [settings]: MoshOptions

    # Errors

    * [`UnsupportedColorType`]: [`Indexed`] is not supported.

    [`Indexed`]: ColorType::Indexed

    # Example
    ```rust
    use libmosh::{
        err::MoshError,
        ops::{read_file, write_file},
        MoshCore,
    };

    let input = read_file("src/util/test-rgb.png")?;
    let output = "test.png";
    let mut image = MoshCore::new();

    image.options.min_rate = 5;
    image.options.max_rate = 7;
    image.options.pixelation = 10;
    image.options.line_shift = 0.7;
    image.options.reverse = 0.4;
    image.options.flip = 0.3;
    image.options.channel_swap = 0.5;
    image.options.channel_shift = 0.5;
    image.options.seed = 42;

    image.read_image(&input)?;
    image.mosh()?;
    write_file(
        output,
        &image.data.buf,
        image.data.width,
        image.data.height,
        image.data.color_type,
        image.data.bit_depth,
    )?;
    # Ok::<(), MoshError>(())
    ```

    [`UnsupportedColorType`]: crate::err::MoshError::UnsupportedColorType
    */
    pub fn mosh(&mut self) -> Result<(), MoshError> {
        self.data.mosh(&self.options)?;

        Ok(())
    }
}

impl MoshOptions {
    fn generate_seed() -> u64 {
        if cfg!(test) {
            TEST_SEED
        } else {
            rand::thread_rng().next_u64()
        }
    }

    /// Generates a new random seed.
    pub fn new_seed(&mut self) {
        self.seed = Self::generate_seed();
    }
}

impl MoshData {
    #[deprecated(since = "3.1.0", note = "Users should use MoshCore instead")]
    pub fn new(input: &[u8]) -> Result<Self, MoshError> {
        let decoder = Decoder::new(input);
        let mut reader = decoder.read_info()?;
        let mut buf = vec![0_u8; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf)?;

        Ok(Self {
            buf: vec![0_u8],
            image: buf,
            width: info.width,
            height: info.height,
            color_type: info.color_type,
            bit_depth: info.bit_depth,
            line_size: info.line_size,
        })
    }

    #[deprecated(since = "3.1.0")]
    pub fn mosh(&mut self, options: &MoshOptions) -> Result<(), MoshError> {
        self.buf.clone_from(&self.image);

        let min_rate = options.min_rate;
        let max_rate = cmp::max(options.min_rate, options.max_rate);
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(options.seed);
        let chunk_count_distrib = Uniform::from(min_rate..=max_rate);
        let mosh_rate = chunk_count_distrib.sample(&mut rng);

        for _ in 0..mosh_rate {
            Self::chunkmosh(self, &mut rng, options);
        }

        match self.color_type {
            ColorType::Indexed => {
                return Err(MoshError::UnsupportedColorType);
            }
            ColorType::Grayscale => {
                Self::pixelation(self, options, fr::PixelType::U8);
            }
            ColorType::GrayscaleAlpha => {
                Self::pixelation(self, options, fr::PixelType::U8x2);
            }
            ColorType::Rgb => {
                Self::pixelation(self, options, fr::PixelType::U8x3);
            }
            ColorType::Rgba => {
                Self::pixelation(self, options, fr::PixelType::U8x4);
            }
        }

        Ok(())
    }

    fn pixelation(&mut self, options: &MoshOptions, pixel_type: fr::PixelType) {
        if options.pixelation > 1 {
            let width = NonZeroU32::new(self.width).unwrap();
            let height = NonZeroU32::new(self.height).unwrap();
            let src_image =
                fr::Image::from_vec_u8(width, height, self.buf.clone(), pixel_type).unwrap();

            let dest_width = NonZeroU32::new(self.width / u32::from(options.pixelation)).unwrap();
            let dest_height = NonZeroU32::new(self.height / u32::from(options.pixelation)).unwrap();
            let orig_width = NonZeroU32::new(self.width).unwrap();
            let orig_height = NonZeroU32::new(self.height).unwrap();

            let mut dest_image = fr::Image::new(dest_width, dest_height, src_image.pixel_type());
            let mut orig_image = fr::Image::new(orig_width, orig_height, src_image.pixel_type());
            let mut dest_view = dest_image.view_mut();
            let mut orig_view = orig_image.view_mut();
            let mut resizer = fr::Resizer::new(fr::ResizeAlg::Nearest);

            resizer.resize(&src_image.view(), &mut dest_view).unwrap();
            resizer.resize(&dest_image.view(), &mut orig_view).unwrap();

            self.buf = orig_image.into_vec();
        }
    }

    // Use pnglitch approach
    //
    // TODO
    // Add more `rng` to `chunk_size`?
    fn chunkmosh(&mut self, rng: &mut impl rand::Rng, options: &MoshOptions) {
        let line_count = self.buf.len() / self.line_size;
        let channel_count = match self.color_type {
            ColorType::Grayscale | ColorType::Indexed => 1,
            ColorType::GrayscaleAlpha => 2,
            ColorType::Rgb => 3,
            ColorType::Rgba => 4,
        };

        let line_shift_distrib = Uniform::from(0..self.line_size);
        let line_number_distrib = Uniform::from(0..line_count);
        let channel_count_distrib = Uniform::from(0..channel_count);

        let first_line = line_number_distrib.sample(rng);
        let chunk_size = line_number_distrib.sample(rng) / 2;
        let last_line = if (first_line + chunk_size) > line_count {
            line_count
        } else {
            first_line + chunk_size
        };

        let reverse = rng.gen_bool(options.reverse);
        let flip = rng.gen_bool(options.flip);

        let line_shift = rng.gen_bool(options.line_shift).then(|| {
            let line_shift_amount = line_shift_distrib.sample(rng);
            MoshLine::Shift(line_shift_amount)
        });

        let channel_shift = rng.gen_bool(options.channel_shift).then(|| {
            let amount = line_shift_distrib.sample(rng) / channel_count;
            let channel = channel_count_distrib.sample(rng);
            MoshLine::ChannelShift(amount, channel, channel_count)
        });

        let channel_swap = rng.gen_bool(options.channel_swap).then(|| {
            let channel_1 = channel_count_distrib.sample(rng);
            let channel_2 = channel_count_distrib.sample(rng);
            MoshChunk::ChannelSwap(channel_1, channel_2, channel_count)
        });

        for line_number in first_line..last_line {
            let line_start = line_number * self.line_size;
            let line_end = line_start + self.line_size;
            let line = &mut self.buf[line_start..line_end];

            if let Some(do_channel_shift) = &channel_shift {
                do_channel_shift.glitch(line);
            }

            if let Some(do_line_shift) = &line_shift {
                do_line_shift.glitch(line);
            }
            if reverse {
                MoshLine::Reverse.glitch(line);
            }
        }

        let chunk_start = first_line * self.line_size;
        let chunk_end = last_line * self.line_size;
        let chunk = &mut self.buf[chunk_start..chunk_end];

        if let Some(do_channel_swap) = channel_swap {
            do_channel_swap.glitch(chunk);
        };

        if flip {
            MoshChunk::Flip.glitch(chunk);
        };
    }
}

impl Default for MoshData {
    fn default() -> Self {
        Self {
            buf: vec![0_u8],
            image: vec![0_u8],
            width: 1,
            height: 1,
            color_type: ColorType::Rgba,
            bit_depth: BitDepth::Eight,
            line_size: 1,
        }
    }
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
            seed: Self::generate_seed(),
        }
    }
}

const TEST_SEED: u64 = 901_042_006;

#[cfg(test)]
mod util;
