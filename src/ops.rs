//! File operations

use png::{BitDepth, ColorType, Encoder};

use std::{
    fs::File,
    io::BufWriter,
    io::{BufReader, Read},
    path::Path,
};

use crate::MoshError;

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
pub fn write_file(
    dest: &str,
    buf: &[u8],
    width: u32,
    height: u32,
    color_type: ColorType,
    bit_depth: BitDepth,
) -> Result<(), MoshError> {
    let path = Path::new(&dest);
    let output = File::create(path)?;
    let buf_writer = &mut BufWriter::new(output);
    let mut encoder = Encoder::new(buf_writer, width, height);

    encoder.set_color(color_type);
    encoder.set_depth(bit_depth);

    let mut writer = encoder.write_header()?;
    writer.write_image_data(buf)?;

    Ok(())
}
