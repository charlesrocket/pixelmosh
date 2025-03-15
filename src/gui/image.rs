use glib::prelude::Cast;
use gtk::{gdk, glib};
use png::ColorType;

use std::path::{Path, PathBuf};

use libmosh::{
    MoshCore, MoshData, MoshOptions,
    err::MoshError,
    ops::{read_file, write_file},
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

    fn generate_texture(data: &mut MoshData, options: &MoshOptions) -> gdk::MemoryTexture {
        let mut palette = None;
        let buf = &data.buf;
        let width = data.width;
        let height = data.height;
        let color_type = if options.ansi {
            ColorType::Indexed
        } else {
            data.color_type
        };

        if options.ansi {
            palette = Some(libmosh::generate_palette());
        } else if color_type == ColorType::Indexed {
            palette = data.palette.clone();
        };

        let (format, stride) = match color_type {
            ColorType::Grayscale => (gdk::MemoryFormat::G8, (width)),
            ColorType::GrayscaleAlpha => (gdk::MemoryFormat::G8a8, (width * 2)),
            ColorType::Rgb => (gdk::MemoryFormat::R8g8b8, (width * 3)),
            ColorType::Rgba => (gdk::MemoryFormat::R8g8b8a8, (width * 4)),
            ColorType::Indexed => {
                let p = palette.unwrap();
                let mut rgb = Vec::with_capacity(buf.len());
                for i in buf.iter().copied().map(usize::from) {
                    rgb.push(p[i * 3]);
                    rgb.push(p[i * 3 + 1]);
                    rgb.push(p[i * 3 + 2]);
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
        self.is_present = true;

        Ok(())
    }

    pub fn save_file(&mut self, file: &Path) -> Result<(), MoshError> {
        write_file(file.to_str().unwrap(), &self.core.data, &self.core.options)?;

        Ok(())
    }

    pub fn mosh_file(&mut self) {
        let min_rate = self.core.options.min_rate;
        let max_rate = std::cmp::max(self.core.options.max_rate, min_rate);

        if min_rate == max_rate {
            self.core.options.max_rate = max_rate + 1;
        };

        self.core.mosh().unwrap();
        self.texture = Self::generate_texture(&mut self.core.data, &self.core.options).upcast();
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

    pub fn set_ansi(&mut self, value: bool) {
        self.core.options.ansi = value;
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
