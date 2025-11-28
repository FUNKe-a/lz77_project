use std::io::Read;
use std::sync::mpsc;
use rayon::prelude::*;

mod input_output;

const CHUNK_SIZE: usize = 4096;

fn main() {
    let mut file_buffer = input_output::open_file_buffer(CHUNK_SIZE);
    let mut buffer = [u8::MIN; CHUNK_SIZE];
    let mut chunk_id = 0;

    let (sender, receiver) = mpsc::channel::<(Vec<(u8,u8)>, usize)>();

    rayon::spawn(move || {
    });

    while let Ok(n) = file_buffer.read(&mut buffer) {
        if n == 0 {
            break;
        }
        let sender_clone = sender.clone();

        rayon::spawn(move || {
            let encoded = run_length_encoding(&buffer[0..n]);
            sender_clone.send((encoded, chunk_id.clone())).unwrap();
        });
    }
}

fn run_length_encoding(data: &[u8]) -> Vec<(u8, u8)> {
    let mut encoded = Vec::new();
    let mut i = 0;

    while i < data.len() {
        let byte = data[i];
        let mut count = 1;

        while count < 255 && i + count < data.len() && byte == data[i + count] {
            count += 1;
        }

        encoded.push((byte, count as u8));
        i += count;
    }

    encoded
}

fn run_length_encoding_parallel(data: &[u8]) -> Vec<(u8, u8)> {
    let mut encoded = Vec::new();
    let mut i = 0;

    while i < data.len() {
        let byte = data[i];
        let mut count = 1;

        while count < 255 && i + count < data.len() && byte == data[i + count] {
            count += 1;
        }

        encoded.push((byte, count as u8));
        i += count;
    }

    encoded
}
