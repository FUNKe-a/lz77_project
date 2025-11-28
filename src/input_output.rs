use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read};

pub fn open_file_buffer(chunk_size: usize) -> BufReader<File> {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening file: {error:?}"),
    };

    BufReader::with_capacity(chunk_size, file)
}
