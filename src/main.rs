use std::fs::File;
use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();
    let decoder = png::Decoder::new(File::open("test.png").unwrap());
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    let bytes = &buf[..info.buffer_size()];
    dbg!(bytes);
}
