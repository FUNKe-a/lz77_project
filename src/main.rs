// use rayon::prelude::*;
use std::io::Read;
// use std::sync::mpsc;
use std::env;
use std::fs::{self, File};

mod input_output;
mod lz_77;

const CHUNK_SIZE: usize = 4096;

fn main() {
    let mut file_buffer = input_output::open_file_buffer(CHUNK_SIZE);
    let mut buffer = [u8::MIN; CHUNK_SIZE];
    let mut chunk_id = 0;

    let args: Vec<String> = env::args().collect();

    let data = fs::read(&args[1]).unwrap();

    lz_77::compress(&data);

    // while let Ok(n) = file_buffer.read(&mut buffer) {
    //     if n == 0 {
    //         break;
    //     }
    //
    //     lz_77::compress(&buffer);
    // }
}
