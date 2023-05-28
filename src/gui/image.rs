use glib::Cast;
use gtk::{gdk, glib};
use png::ColorType;

use std::path::{Path, PathBuf};

use libmosh::{
    err::MoshError,
    ops::{read_file, write_file},
    MoshCore, MoshOptions,
};

pub struct Image {
    pub core: MoshCore,
    pub texture: gdk::Texture,
    pub settings: Option<MoshOptions>,
    pub is_present: bool,
}

impl Image {
    fn new() -> Self {
        Self {
            core: MoshCore::new(),
            texture: gdk::MemoryTexture::new(
                1_i32,
                1_i32,
                gdk::MemoryFormat::R8g8b8,
                &glib::Bytes::from_owned([1, 1, 1]),
                3,
            )
            .upcast(),
            settings: None,
            is_present: false,
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
            ColorType::Indexed => {
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

            ColorType::GrayscaleAlpha => {
                let mut rgba = Vec::with_capacity(buf.len());
                for row in buf.chunks_exact(line_size) {
                    for pixels in row.chunks_exact(2) {
                        let gray = pixels[0];
                        let alpha = pixels[1];

                        rgba.extend_from_slice(&[gray, gray, gray, alpha]);
                    }
                }

                return gdk::MemoryTexture::new(
                    width as i32,
                    height as i32,
                    gdk::MemoryFormat::R8g8b8a8,
                    &glib::Bytes::from_owned(rgba),
                    width as usize * 4,
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

        self.core.read_image(&input)?;

        if self.core.data.color_type != ColorType::Indexed {
            let texture = Self::generate_texture(
                &self.core.data.buf,
                self.core.data.width,
                self.core.data.height,
                self.core.data.color_type,
                self.core.data.line_size,
            );

            self.texture = texture.upcast();
            self.is_present = true;
        } else {
            self.is_present = false;
        }

        Ok(())
    }

    pub fn save_file(&mut self, file: &Path) -> Result<(), MoshError> {
        write_file(
            file.to_str().unwrap(),
            &self.core.data.buf,
            self.core.data.width,
            self.core.data.height,
            self.core.data.color_type,
            self.core.data.bit_depth,
        )?;

        Ok(())
    }

    pub fn mosh_file(&mut self) {
        self.core.mosh().unwrap();
        self.texture = Self::generate_texture(
            &self.core.data.buf,
            self.core.data.width,
            self.core.data.height,
            self.core.data.color_type,
            self.core.data.line_size,
        )
        .upcast();
    }

    pub fn get_texture(&mut self) -> gdk::Texture {
        self.texture.clone()
    }
}

impl Image {
    pub fn new_seed(&mut self) {
        self.core.options.new_seed();
    }

    pub fn get_seed(&self) -> u64 {
        self.core.options.seed
    }

    pub fn set_seed(&mut self, value: u64) {
        self.core.options.seed = value;
    }

    pub fn set_min_rate(&mut self, value: u16) {
        self.core.options.min_rate = value;
    }

    pub fn set_max_rate(&mut self, value: u16) {
        self.core.options.max_rate = value;
    }

    pub fn set_pixelation(&mut self, value: u8) {
        self.core.options.pixelation = value;
    }

    pub fn set_line_shift(&mut self, value: f64) {
        self.core.options.line_shift = value;
    }

    pub fn set_reverse(&mut self, value: f64) {
        self.core.options.reverse = value;
    }

    pub fn set_flip(&mut self, value: f64) {
        self.core.options.flip = value;
    }

    pub fn set_channel_swap(&mut self, value: f64) {
        self.core.options.channel_swap = value;
    }

    pub fn set_channel_shift(&mut self, value: f64) {
        self.core.options.channel_shift = value;
    }

    pub fn save_settings(&mut self) {
        self.settings = Some(self.core.options.clone());
    }

    pub fn load_settings(&mut self) {
        if let Some(settings) = self.settings.clone() {
            self.core.options = settings;
            self.settings = None;
        }
    }
}

impl Default for Image {
    fn default() -> Self {
        Self::new()
    }
}
