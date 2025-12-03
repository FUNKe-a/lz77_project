use std::fs::File;
use std::io::Write;

pub fn output_to_file(bytes: &Vec<u8>, file_path: &str) {
    let mut file = match File::create(file_path) { 
        Ok(file) => file,
        Err(error) => panic!("problem creating file : {error:?}"),
    };

    file.write_all(bytes).unwrap();
}
