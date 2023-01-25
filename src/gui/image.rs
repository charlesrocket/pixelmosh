use glib::Cast;
use gtk::{gdk, glib};
use png::ColorType;

use std::path::{Path, PathBuf};

use libmosh::{
    err::MoshError,
    ops::{read_file, write_file},
    MoshData, MoshOptions,
};

pub struct Image {
    pub data: MoshData,
    pub texture: gdk::Texture,
}

#[derive(Default)]
pub struct Options(MoshOptions);

impl Image {
    fn new() -> Self {
        Self {
            data: MoshData::default(),
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

    fn generate_texture(
        buf: &Vec<u8>,
        width: u32,
        height: u32,
        color_type: ColorType,
        line_size: usize,
    ) -> gdk::MemoryTexture {
        let (format, stride) = match &color_type {
            ColorType::GrayscaleAlpha | ColorType::Indexed => {
                todo!()
            }

            ColorType::Grayscale => {
                let mut rgb = Vec::with_capacity(buf.len());
                for row in buf.chunks_exact(line_size) {
                    for &pixel in &row[..width as usize] {
                        rgb.extend_from_slice(&[pixel, pixel, pixel]);
                    }
                }

                return gdk::MemoryTexture::new(
                    width as i32,
                    height as i32,
                    gdk::MemoryFormat::R8g8b8,
                    &glib::Bytes::from_owned(rgb),
                    width as usize * 3,
                )
                .upcast();
            }

            ColorType::Rgb => (gdk::MemoryFormat::R8g8b8, (width * 3)),
            ColorType::Rgba => (gdk::MemoryFormat::R8g8b8a8, (width * 4)),
        };

        gdk::MemoryTexture::new(
            width as i32,
            height as i32,
            format,
            &glib::Bytes::from(buf),
            stride as usize,
        )
    }

    pub fn open_file(&mut self, file: &PathBuf) -> Result<(), MoshError> {
        let input = read_file(file)?;

        self.data = MoshData::new(&input).unwrap();

        if self.data.color_type != ColorType::GrayscaleAlpha {
            let texture = Self::generate_texture(
                &self.data.buf,
                self.data.width,
                self.data.height,
                self.data.color_type,
                self.data.line_size,
            );

            self.texture = texture.upcast();
        }

        Ok(())
    }

    pub fn save_file(&mut self, file: &Path) -> Result<(), MoshError> {
        write_file(
            file.to_str().unwrap(),
            &self.data.buf,
            self.data.width,
            self.data.height,
            self.data.color_type,
            self.data.bit_depth,
        )?;

        Ok(())
    }

    pub fn mosh(&mut self, options: &Options) {
        self.data.mosh(&options.0).unwrap();
        self.texture = Self::generate_texture(
            &self.data.buf,
            self.data.width,
            self.data.height,
            self.data.color_type,
            self.data.line_size,
        )
        .upcast();
    }

    pub fn get_texture(&mut self) -> gdk::Texture {
        self.texture.clone()
    }
}

impl Options {
    pub fn new_seed(&mut self) {
        self.0.new_seed();
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
}

impl Default for Image {
    fn default() -> Self {
        Self::new()
    }
}
