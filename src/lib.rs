use std::fs::File;
use std::io::BufReader;

use fasthash::metro;

pub fn hash(s: &str) -> u64 {
    metro::hash64(&s)
}

pub fn buf_reader_from_path(file_path: &String) -> BufReader<File> {
    let f = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            println!("file `{}` not found: {}", file_path, e);
            std::process::exit(2);
        }
    };

    BufReader::new(f)
}
