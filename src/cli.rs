use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

#[must_use]
pub fn display_var() -> bool {
    matches!(env::var("DISPLAY"), Ok(_))
}

#[must_use]
pub fn read_file(file: String) -> (std::vec::Vec<u8>, png::OutputInfo) {
    let input = File::open(file);
    let input = match input {
        Ok(file) => file,
        Err(error) => {
            eprintln!("\x1b[1;31merror:\x1b[0m {}", error);
            std::process::exit(1)
        }
    };

    let decoder = png::Decoder::new(input);
    let reader = decoder.read_info();
    let mut reader = match reader {
        Ok(reader) => reader,
        Err(error) => {
            eprintln!("\x1b[1;31merror:\x1b[0m {} Exiting", error);
            std::process::exit(0)
        }
    };

    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    (buf, info)
}

pub fn write_file(path: &str, buf: &[u8], info: &png::OutputInfo) {
    let path = Path::new(&path);
    let output = File::create(path).unwrap();
    let buf_writer = &mut BufWriter::new(output);
    let mut encoder = png::Encoder::new(buf_writer, info.width, info.height);

    encoder.set_color(info.color_type);
    encoder.set_depth(info.bit_depth);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(buf).unwrap();
}
