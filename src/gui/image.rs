use glib::Cast;
use gtk::{gdk, glib};
use png::{BitDepth, ColorType, OutputInfo};

use std::path::PathBuf;

use libmosh::err::MoshError;
use libmosh::{mosh, MoshOptions};

use libmosh::ops::read_file;

pub struct Image {
    pub buf: Vec<u8>,
    pub buf_new: Vec<u8>,
    pub info: OutputInfo,
    pub texture: gdk::Texture,
}

#[derive(Default)]
pub struct Options(MoshOptions);

impl Image {
    pub fn new() -> Self {
        Self {
            buf: vec![0_u8],
            buf_new: vec![0_u8],
            info: OutputInfo {
                width: 1,
                height: 1,
                color_type: ColorType::Rgb,
                bit_depth: BitDepth::Eight,
                line_size: 1,
            },

            texture: gdk::MemoryTexture::new(
                1_i32,
                1_i32,
                gdk::MemoryFormat::R8g8b8,
                &glib::Bytes::from_owned([1, 1, 1]),
                3,
            )
            .upcast(),
        }
    }

    pub fn generate_texture(info: &OutputInfo, buf: &Vec<u8>) -> gdk::MemoryTexture {
        let (format, stride) = match &info.color_type {
            ColorType::GrayscaleAlpha | ColorType::Indexed => {
                todo!()
            }

            ColorType::Grayscale => {
                let mut rgb = Vec::with_capacity(buf.len());
                for row in buf.chunks_exact(info.line_size) {
                    for &pixel in &row[..info.width as usize] {
                        rgb.extend_from_slice(&[pixel, pixel, pixel]);
                    }
                }

                return gdk::MemoryTexture::new(
                    info.width as i32,
                    info.height as i32,
                    gdk::MemoryFormat::R8g8b8,
                    &glib::Bytes::from_owned(rgb),
                    info.width as usize * 3,
                )
                .upcast();
            }

            ColorType::Rgb => (gdk::MemoryFormat::R8g8b8, (info.width * 3)),
            ColorType::Rgba => (gdk::MemoryFormat::R8g8b8a8, (info.width * 4)),
        };

        gdk::MemoryTexture::new(
            info.width as i32,
            info.height as i32,
            format,
            &glib::Bytes::from(buf),
            stride as usize,
        )
    }

    pub fn open_file(&mut self, file: &PathBuf) -> Result<(), MoshError> {
        let (buf, info) = read_file(file)?;
        let texture = Self::generate_texture(&info, &buf);

        self.buf = buf;
        self.info = info;
        self.texture = texture.upcast();

        Ok(())
    }

    pub fn mosh(&mut self, options: &Options) {
        self.buf_new.clear();
        let mut buf = self.buf.clone();
        mosh(&self.info, &mut buf, &options.0).unwrap();
        self.texture = Self::generate_texture(&self.info, &buf).upcast();
    }

    pub fn get_texture(&mut self) -> gdk::Texture {
        self.texture.clone()
    }
}

impl Options {
    pub fn min_rate(&self) -> u16 {
        self.0.min_rate
    }

    pub fn max_rate(&self) -> u16 {
        self.0.max_rate
    }

    pub fn pixelation(&self) -> u8 {
        self.0.pixelation
    }

    pub fn line_shift(&self) -> f64 {
        self.0.line_shift
    }

    pub fn reverse(&self) -> f64 {
        self.0.reverse
    }

    pub fn flip(&self) -> f64 {
        self.0.flip
    }

    pub fn channel_swap(&self) -> f64 {
        self.0.channel_swap
    }

    pub fn channel_shift(&self) -> f64 {
        self.0.channel_shift
    }

    pub fn seed(&self) -> u64 {
        self.0.seed
    }

    pub fn set_min_rate(&mut self, value: u16) {
        self.0.min_rate = value;
    }

    pub fn set_max_rate(&mut self, value: u16) {
        self.0.max_rate = value;
    }

    pub fn set_pixelation(&mut self, value: u8) {
        self.0.pixelation = value;
    }

    pub fn set_line_shift(&mut self, value: f64) {
        self.0.line_shift = value;
    }

    pub fn set_reverse(&mut self, value: f64) {
        self.0.reverse = value;
    }

    pub fn set_flip(&mut self, value: f64) {
        self.0.flip = value;
    }

    pub fn set_channel_swap(&mut self, value: f64) {
        self.0.channel_swap = value;
    }

    pub fn set_channel_shift(&mut self, value: f64) {
        self.0.channel_shift = value;
    }

    pub fn set_seed(&mut self, value: u64) {
        self.0.seed = value;
    }
}

impl Default for Image {
    fn default() -> Self {
        Self::new()
    }
}
