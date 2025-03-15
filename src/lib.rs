/*! # Overview

_Glitch and pixelate PNG images_

Provides the [`MoshCore`] type for image processing and I/O functions,
available in the [`ops`] module.

# Usage
Add `pixelmosh` to your dependencies in your project's `Cargo.toml`.

```shell
cargo add pixelmosh --no-default-features
```

# Example
```rust
use libmosh::{
    err::MoshError,
    ops::{read_file, write_file},
    MoshCore,
};

let input = read_file("tests/assets/test-rgb.png")?;
let output = "test.png";
let mut core = MoshCore::new();

core.read_image(&input)?;
core.mosh()?;
write_file(
    output,
    &core.data,
    &core.options,
)?;
# Ok::<(), MoshError>(())
```
*/

use fast_image_resize as fr;

use png::{BitDepth, ColorType, Decoder};
use rand::{
    RngCore, SeedableRng,
    distr::{Distribution, Uniform},
};

use std::cmp;

use crate::{
    err::MoshError,
    fx::{Mosh, MoshChunk, MoshLine},
};

pub mod err;
pub mod fx;
pub mod ops;

const ANSI_COLORS: [(u8, u8, u8); 16] = [
    (0, 0, 0),       // Black
    (205, 0, 0),     // Red
    (0, 205, 0),     // Green
    (205, 205, 0),   // Yellow
    (0, 0, 205),     // Blue
    (205, 0, 205),   // Magenta
    (0, 205, 205),   // Cyan
    (229, 229, 229), // White
    (127, 127, 127), // Bright Black
    (255, 0, 0),     // Bright Red
    (0, 255, 0),     // Bright Green
    (255, 255, 0),   // Bright Yellow
    (0, 0, 255),     // Bright Blue
    (255, 0, 255),   // Bright Magenta
    (0, 255, 255),   // Bright Cyan
    (255, 255, 255), // Bright White
];

/// Image data.
///
/// It holds the original image, buffer and parameters.
#[non_exhaustive]
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
    /// Color palette.
    pub palette: Option<Vec<u8>>,
    /// Line size.
    pub line_size: usize,
}

/// Processing options.
///
/// Minimal `pixelation` value is `1` (OFF).
#[non_exhaustive]
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
    /// Convert to ANSI color palette.
    pub ansi: bool,
    /// Random seed.
    pub seed: u64,
}

/// Core container.
///
/// Holds image data and processing options.
#[non_exhaustive]
#[derive(Clone, Default)]
pub struct MoshCore {
    pub data: MoshData,
    pub options: MoshOptions,
}

impl MoshCore {
    /// Creates a new, empty instance of [`MoshCore`] with a random [seed].
    ///
    /// [seed]: MoshOptions::seed
    #[must_use]
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

        if let Some(palette) = &reader.info().palette {
            self.data.palette = Some(palette.to_vec());
        }

        self.data.buf.clone_from(&buf);
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

    let input = read_file("tests/assets/test-rgb.png")?;
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
        &image.data,
        &image.options,
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
            rand::rng().next_u64()
        }
    }

    /// Generates a new random seed.
    pub fn new_seed(&mut self) {
        self.seed = Self::generate_seed();
    }
}

impl MoshData {
    fn mosh(&mut self, options: &MoshOptions) -> Result<(), MoshError> {
        self.buf.clone_from(&self.image);

        let min_rate = options.min_rate;
        let max_rate = cmp::max(options.min_rate, options.max_rate);
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(options.seed);
        let chunk_count_distrib = Uniform::new(min_rate, max_rate)?;
        let mosh_rate = chunk_count_distrib.sample(&mut rng);

        for _ in 0..mosh_rate {
            Self::chunkmosh(self, &mut rng, options)?;
        }

        match self.color_type {
            ColorType::Grayscale | ColorType::Indexed => {
                self.pixelation(options, fr::PixelType::U8);
            }
            ColorType::GrayscaleAlpha => {
                self.pixelation(options, fr::PixelType::U8x2);
            }
            ColorType::Rgb => {
                self.pixelation(options, fr::PixelType::U8x3);
            }
            ColorType::Rgba => {
                self.pixelation(options, fr::PixelType::U8x4);
            }
        }

        if options.ansi {
            self.generate_ansi_data()?;
        }

        Ok(())
    }

    fn pixelation(&mut self, options: &MoshOptions, pixel_type: fr::PixelType) {
        if options.pixelation > 1 {
            let width = self.width;
            let height = self.height;
            let src_image =
                fr::images::Image::from_vec_u8(width, height, self.buf.clone(), pixel_type)
                    .unwrap();

            let dest_width = self.width / u32::from(options.pixelation);
            let dest_height = self.height / u32::from(options.pixelation);
            let orig_width = self.width;
            let orig_height = self.height;

            let mut dest_image =
                fr::images::Image::new(dest_width, dest_height, src_image.pixel_type());
            let mut orig_image =
                fr::images::Image::new(orig_width, orig_height, src_image.pixel_type());
            let mut resizer = fr::Resizer::new();

            resizer
                .resize(
                    &src_image,
                    &mut dest_image,
                    &fr::ResizeOptions::new().resize_alg(fr::ResizeAlg::Nearest),
                )
                .unwrap();
            resizer
                .resize(
                    &dest_image,
                    &mut orig_image,
                    &fr::ResizeOptions::new().resize_alg(fr::ResizeAlg::Nearest),
                )
                .unwrap();

            self.buf = orig_image.into_vec();
        }
    }

    fn get_palette_color(&self, idx: usize) -> Result<(u8, u8, u8), MoshError> {
        match &self.palette {
            Some(palette) => {
                let r = palette[idx * 3];
                let g = palette[idx * 3 + 1];
                let b = palette[idx * 3 + 2];
                Ok((r, g, b))
            }
            None => Err(MoshError::InvalidPalette),
        }
    }

    /// Converts an image buffer using the ANSI color set.
    ///
    /// # Errors
    ///
    /// It may fail if the image data has the wrong format.
    pub fn generate_ansi_data(&mut self) -> Result<(), MoshError> {
        let mut ansi_data: Vec<u8> = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = (y * self.width + x) as usize
                    * match self.color_type {
                        ColorType::Grayscale | ColorType::Indexed => 1,
                        ColorType::GrayscaleAlpha => 2,
                        ColorType::Rgb => 3,
                        ColorType::Rgba => 4,
                    };

                let r = match self.color_type {
                    ColorType::Indexed => {
                        let palette_idx = self.buf[idx] as usize;
                        let (r, _, _) = self.get_palette_color(palette_idx)?;
                        r
                    }
                    _ => self.buf[idx],
                };

                let g = match self.color_type {
                    ColorType::Rgb | ColorType::Rgba => self.buf[idx + 1],
                    ColorType::Indexed => {
                        let palette_idx = self.buf[idx] as usize;
                        let (_, g, _) = self.get_palette_color(palette_idx)?;
                        g
                    }
                    _ => self.buf[idx],
                };

                let b = match self.color_type {
                    ColorType::Rgb | ColorType::Rgba => self.buf[idx + 2],
                    ColorType::Indexed => {
                        let palette_idx = self.buf[idx] as usize;
                        let (_, _, b) = self.get_palette_color(palette_idx)?;
                        b
                    }
                    _ => self.buf[idx],
                };

                let ansi_color = get_ansi_color(r, g, b)?;
                ansi_data.push(ansi_color);
            }
        }

        self.buf = ansi_data;

        Ok(())
    }

    // Use pnglitch approach
    //
    // TODO
    // Add more `rng` to `chunk_size`?
    fn chunkmosh(
        &mut self,
        rng: &mut impl rand::Rng,
        options: &MoshOptions,
    ) -> Result<(), MoshError> {
        let line_count = self.buf.len() / self.line_size;
        let channel_count = match self.color_type {
            ColorType::Grayscale | ColorType::Indexed => 1,
            ColorType::GrayscaleAlpha => 2,
            ColorType::Rgb => 3,
            ColorType::Rgba => 4,
        };

        let line_shift_distrib = Uniform::new(0, self.line_size)?;
        let line_number_distrib = Uniform::new(0, line_count)?;
        let channel_count_distrib = Uniform::new(0, channel_count)?;

        let first_line = line_number_distrib.sample(rng);
        let chunk_size = line_number_distrib.sample(rng) / 2;
        let last_line = if (first_line + chunk_size) > line_count {
            line_count
        } else {
            first_line + chunk_size
        };

        let reverse = rng.random_bool(options.reverse);
        let flip = rng.random_bool(options.flip);

        let line_shift = rng.random_bool(options.line_shift).then(|| {
            let line_shift_amount = line_shift_distrib.sample(rng);
            MoshLine::Shift(line_shift_amount)
        });

        let channel_shift = rng.random_bool(options.channel_shift).then(|| {
            let amount = line_shift_distrib.sample(rng) / channel_count;
            let channel = channel_count_distrib.sample(rng);
            MoshLine::ChannelShift(amount, channel, channel_count)
        });

        let channel_swap = rng.random_bool(options.channel_swap).then(|| {
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

        Ok(())
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
            palette: None,
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
            ansi: false,
            seed: Self::generate_seed(),
        }
    }
}

fn get_ansi_color(r: u8, g: u8, b: u8) -> Result<u8, MoshError> {
    let mut closest_index = 0;
    let mut min_distance: i32 = i32::MAX;

    for (index, &color) in ANSI_COLORS.iter().enumerate() {
        // Calculate squared Euclidean distance between RGB colors
        let distance = (i32::from(r) - i32::from(color.0)).pow(2)
            + (i32::from(g) - i32::from(color.1)).pow(2)
            + (i32::from(b) - i32::from(color.2)).pow(2);

        if distance < min_distance {
            min_distance = distance;
            closest_index = index;
        }
    }

    let color = u8::try_from(closest_index)?;
    Ok(color)
}

#[must_use]
pub fn generate_palette() -> Vec<u8> {
    let mut palette = Vec::with_capacity(ANSI_COLORS.len() * 3);
    for &(r, g, b) in &ANSI_COLORS {
        palette.push(r);
        palette.push(g);
        palette.push(b);
    }

    palette
}

const TEST_SEED: u64 = 901_042_006;

#[cfg(test)]
mod tests;
