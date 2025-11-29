// use rayon::prelude::*;
// use std::io::Read;
// use std::sync::mpsc;

mod input_output;
mod lz_77;

const CHUNK_SIZE: usize = 4096;

fn main() {
    // let mut file_buffer = input_output::open_file_buffer(CHUNK_SIZE);
    // let mut buffer = [u8::MIN; CHUNK_SIZE];
    // let mut chunk_id = 0;
    //
    // let (sender, receiver) = mpsc::channel::<(Vec<(u8, u8)>, usize)>();
    //
    // rayon::spawn(move || {});
    //
    // while let Ok(n) = file_buffer.read(&mut buffer) {
    //     if n == 0 {
    //         break;
    //     }
    //     let sender_clone = sender.clone();
    //
    //     rayon::spawn(move || {
    //         let encoded = run_length_encoding(&buffer[0..n]);
    //         sender_clone.send((encoded, chunk_id.clone())).unwrap();
    //     });
    // }
}
