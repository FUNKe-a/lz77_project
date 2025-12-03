use clap::{Parser, ValueEnum};
use rayon::prelude::*;
use std::fs;
use inzinerinis_projektas::*;

mod input_output;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(value_enum)]
    mode: Mode,
    #[arg(short, long)]
    output: String,
    #[arg(short, long)]
    input: String,
    #[arg(short, long, default_value_t = 64 * 1024)]
    chunk_size: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    Singlethreaded,
    Multithreaded,
    Decompress,
}

fn main() {
    let cli = Cli::parse();

    let data = fs::read(&cli.input).unwrap();
    let chunks: Vec<&[u8]> = data.chunks(cli.chunk_size).collect();
    let mut res: Vec<u8> = Vec::new();

    match cli.mode {
        Mode::Singlethreaded => {
            // let mut reader = input_output::open_file_buffer(&cli.input, cli.io_chunk_size);
            // let mut buffer = vec![0u8; cli.io_chunk_size];

            for chunk in chunks {
                let mut comp_chunk = compress(&chunk);
                res.append(&mut comp_chunk);
            }
        },
        Mode::Multithreaded => {
            let comp_chunks = chunks
                .par_iter()
                .map(|chunk| compress(chunk))
                .collect::<Vec<Vec<u8>>>();

            for chunk in comp_chunks { res.extend(chunk); }
        },
        Mode::Decompress => {
            res = decompress(&data);
        },
    }

    input_output::output_to_file(&res, &cli.output);
}
