use std::env;
use std::fs::File;
use std::io::{BufWriter, Error};
use std::path::Path;

/// Logo
pub const BANNER: &str = "┌─────────────────────────────────────┐\n\
                          │ █▀▄ █ ▀▄▀ ██▀ █   █▄ ▄█ ▄▀▄ ▄▀▀ █▄█ │\n\
                          │ █▀  █ █ █ █▄▄ █▄▄ █ ▀ █ ▀▄▀ ▄██ █ █ │\n\
                          └─────────────────────────────────────┘";

/// TTY
pub const SPINNER_1: [&str; 7] = [
    "∙∙∙∙∙",
    "●∙∙∙∙",
    "∙●∙∙∙",
    "∙∙●∙∙",
    "∙∙∙●∙",
    "∙∙∙∙●",
    "∙∙∙∙∙",
];

/// Terminal
pub const SPINNER_2: [&str; 7] = [
    "▱▱▱▱▱",
    "▰▱▱▱▱",
    "▱▰▱▱▱",
    "▱▱▰▱▱",
    "▱▱▱▰▱",
    "▱▱▱▱▰",
    "▰▰▰▰▰",
];

#[must_use]
pub fn display_var() -> bool {
    // TODO
    // $TERM?
    matches!(env::var("DISPLAY"), Ok(_))
}

/// # Errors
/// TODO
pub fn read_file(file: String) -> Result<(Vec<u8>, png::OutputInfo), Error> {
    let input = File::open(file)?;
    let decoder = png::Decoder::new(input);
    let mut reader = decoder.read_info()?;
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf)?;

    Ok((buf, info))
}

/// # Errors
/// TODO
pub fn write_file(dest: &str, buf: &[u8], info: &png::OutputInfo) -> Result<(), Error> {
    let path = Path::new(&dest);
    let output = File::create(path)?;
    let buf_writer = &mut BufWriter::new(output);
    let mut encoder = png::Encoder::new(buf_writer, info.width, info.height);

    encoder.set_color(info.color_type);
    encoder.set_depth(info.bit_depth);

    let mut writer = encoder.write_header()?;
    writer.write_image_data(buf)?;

    Ok(())
}
