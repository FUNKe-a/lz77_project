// use rayon::prelude::*;
// use std::sync::mpsc;
use std::env;
use std::fs::{self, File};
use clap::Parser;

mod input_output;
mod lz_77;

const CHUNK_SIZE: usize = 4096;

#[derive(Parser, Debug)]
struct Args {
}

fn main() {
    let mut file_buffer = input_output::open_file_buffer(CHUNK_SIZE);
    let mut buffer = [u8::MIN; CHUNK_SIZE];
    let mut chunk_id = 0;

    let args: Vec<String> = env::args().collect();

    let data = fs::read(&args[1]).unwrap();

    let compressed = lz_77::compress(&data);

    input_output::output_to_file(&compressed, "/home/LinRob/KTU/Multithreading_code/inzinerinis_projektas/inzinerinis_projektas/results/pog.lz77");

    let data_comp = fs::read("/home/LinRob/KTU/Multithreading_code/inzinerinis_projektas/inzinerinis_projektas/results/pog.lz77").unwrap();

    let decomp = lz_77::decompress(&data_comp);

    input_output::output_to_file(&decomp, "/home/LinRob/KTU/Multithreading_code/inzinerinis_projektas/inzinerinis_projektas/results/pog_dec.txt");

    // while let Ok(n) = file_buffer.read(&mut buffer) {
    //     if n == 0 {
    //         break;
    //     }
    //
    //     lz_77::compress(&buffer);
    // }
}
