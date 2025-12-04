use clap::{Parser, ValueEnum};
use lz77_project::*;
use rayon::prelude::*;
use std::fs::{self, File};
use std::io::Write;

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

    let data = match fs::read(&cli.input) {
        Ok(data) => data,
        Err(error) => panic!("problem with the input file : {error}"),
    };

    let chunks: Vec<&[u8]> = data.chunks(cli.chunk_size).collect();
    let mut res: Vec<u8> = Vec::new();

    match cli.mode {
        Mode::Singlethreaded => {
            for chunk in chunks {
                let mut comp_chunk = compress(&chunk);
                res.append(&mut comp_chunk);
            }
        }
        Mode::Multithreaded => {
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
