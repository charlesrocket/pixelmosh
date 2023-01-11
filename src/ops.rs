//! File operations

use png::{Decoder, Encoder};

use std::{fs::File, io::BufWriter, path::Path};

use crate::MoshError;

/// Reads provided file
///
/// # Errors
///
/// It may fail if input format is not supported.
pub fn read_file(file: impl AsRef<Path>) -> Result<(Vec<u8>, png::OutputInfo), MoshError> {
    let input = File::open(file)?;
    let decoder = Decoder::new(input);
    let mut reader = decoder.read_info()?;
    let mut buf = vec![0_u8; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf)?;

    Ok((buf, info))
}

/// Writes a new file from the provided buffer
///
/// # Errors
///
/// It may fail if parameters are invalid or due I/O error.
pub fn write_file(dest: &str, buf: &[u8], info: &png::OutputInfo) -> Result<(), MoshError> {
    let path = Path::new(&dest);
    let output = File::create(path)?;
    let buf_writer = &mut BufWriter::new(output);
    let mut encoder = Encoder::new(buf_writer, info.width, info.height);

    encoder.set_color(info.color_type);
    encoder.set_depth(info.bit_depth);

    let mut writer = encoder.write_header()?;
    writer.write_image_data(buf)?;

    Ok(())
}
