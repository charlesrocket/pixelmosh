//! File operations

use png::{BitDepth, ColorType, Encoder};

use std::{
    fs::File,
    io::{BufReader, BufWriter, Read},
    path::Path,
};

use crate::{MoshData, MoshError, MoshOptions};

/// Reads provided file
///
/// # Errors
///
/// It may fail if input format is not supported.
pub fn read_file(file: impl AsRef<Path>) -> Result<Vec<u8>, MoshError> {
    let input = File::open(file)?;
    let mut reader = BufReader::new(input);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer)?;

    Ok(buffer)
}

/// Writes a new file from the provided buffer
///
/// # Errors
///
/// It may fail if parameters are invalid or due I/O error.
pub fn write_file(dest: &str, data: &MoshData, options: &MoshOptions) -> Result<(), MoshError> {
    let path = Path::new(&dest);
    let output = File::create(path)?;
    let buf_writer = &mut BufWriter::new(output);
    let mut encoder = Encoder::new(buf_writer, data.width, data.height);

    encoder.set_color(if options.ansi {
        ColorType::Indexed
    } else {
        data.color_type
    });

    encoder.set_depth(if options.ansi {
        BitDepth::Eight
    } else {
        data.bit_depth
    });

    if data.color_type == ColorType::Indexed {
        if options.ansi {
            encoder.set_palette(crate::generate_palette());
        } else {
            encoder.set_palette(data.palette.clone().unwrap());
        }
    }

    if options.ansi {
        encoder.set_palette(crate::generate_palette());
    };

    let mut writer = encoder.write_header()?;
    writer.write_image_data(&data.buf)?;

    Ok(())
}
