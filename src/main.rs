use clap::{Parser, Subcommand};
use lz77_project::*;
use rayon::prelude::*;
use std::fs::{self, File};
use std::io::Write;

#[derive(Parser)]
#[command(version, about = "A program for compressing and decompressing files using the LZ77 algorithm")]
struct Cli {
    #[command(subcommand)]
    mode: Mode,
    #[arg(short, long, help = "path to a file that the program reads data from")]
    input: String,
    #[arg(
        short,
        long,
        help = "path to a file that the program uses for storing output"
    )]
    output: String,
}

#[derive(Subcommand)]
enum Mode {
    /// compresses the input file using a single thread
    Singlethreaded {
        #[arg(short, long, default_value_t = 64 * 1024, help="size of data chunks in bytes for file processing")]
        chunk_size: usize,
    },
    /// compresses the input file using multiple threads
    Multithreaded {
        #[arg(short, long, default_value_t = 64 * 1024, help="size of data chunks in bytes for file processing")]
        chunk_size: usize,
    },
    /// decompresses the given input file
    Decompress,
}

fn main() {
    let cli = Cli::parse();

    let data = match fs::read(&cli.input) {
        Ok(data) => data,
        Err(error) => panic!("problem with the input file : {error}"),
    };

    let mut res: Vec<u8> = Vec::new();

    match &cli.mode {
        Mode::Singlethreaded { chunk_size } => {
            let chunks: Vec<&[u8]> = data.chunks(*chunk_size).collect();
            for chunk in chunks {
                let mut comp_chunk = compress(&chunk);
                res.append(&mut comp_chunk);
            }
        }
        Mode::Multithreaded { chunk_size } => {
            let chunks: Vec<&[u8]> = data.chunks(*chunk_size).collect();
            let comp_chunks = chunks
                .par_iter()
                .map(|chunk| compress(chunk))
                .collect::<Vec<Vec<u8>>>();

            for chunk in comp_chunks {
                res.extend(chunk);
            }
        }
        Mode::Decompress => {
            res = decompress(&data);
        }
    }

    output_to_file(&res, &cli.output);
}

pub fn output_to_file(bytes: &Vec<u8>, file_path: &str) {
    let mut file = match File::create(file_path) {
        Ok(file) => file,
        Err(error) => panic!("problem with the output file : {error:?}"),
    };

    file.write_all(bytes).unwrap();
}
