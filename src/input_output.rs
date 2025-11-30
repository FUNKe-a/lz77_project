use std::env;
use std::fs::File;
use std::io::{Write, BufReader};

pub fn open_file_buffer(chunk_size: usize) -> BufReader<File> {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening file: {error:?}"),
    };

    BufReader::with_capacity(chunk_size, file)
}

pub fn output_to_file(bytes: &Vec<u8>, file_path: &str) {
    let mut file = match File::create(file_path) { 
        Ok(file) => file,
        Err(error) => panic!("problem creating file : {error:?}"),
    };

    file.write_all(bytes).unwrap();
}
