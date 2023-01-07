use glib::Cast;
use gtk::{gdk, glib};
use png::{BitDepth, ColorType, Decoder, OutputInfo};

use std::fs::File;
use std::path::PathBuf;

use libmosh::err::MoshError;
use libmosh::{mosh, MoshOptions};

pub struct Image {
    pub buf: Vec<u8>,
    pub buf_new: Vec<u8>,
    pub info: OutputInfo,
    pub texture: gdk::Texture,
}

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
        let input = File::open(file)?;
        let decoder = Decoder::new(input);
        let mut reader = decoder.read_info()?;
        let mut img_buf = vec![0_u8; reader.output_buffer_size()];
        let img_info = reader.next_frame(&mut img_buf)?;
        let img_texture = Self::generate_texture(&img_info, &img_buf);

        self.set_buf(img_buf);
        self.set_info(img_info);
        self.set_texture(img_texture.upcast());
        Ok(())
    }

    pub fn mosh(&mut self, options: &MoshOptions) {
        self.buf_new.clear();
        let mut buf = self.buf.clone();
        mosh(&self.info, &mut buf, options).unwrap();
        self.set_texture(Self::generate_texture(&self.info, &buf).upcast());
    }

    pub fn get_texture(&mut self) -> gdk::Texture {
        self.texture.clone()
    }

    pub fn set_buf(&mut self, value: Vec<u8>) {
        self.buf = value;
    }

    pub fn set_info(&mut self, value: OutputInfo) {
        self.info = value;
    }

    pub fn set_texture(&mut self, value: gdk::Texture) {
        self.texture = value;
    }
}

impl Default for Image {
    fn default() -> Self {
        Self::new()
    }
}
