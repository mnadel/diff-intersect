use std::fs::File;
use std::io::BufReader;

use fasthash::metro;

pub fn hash(s: &str) -> u64 {
    metro::hash64(&s)
}

pub fn buf_reader_from_path(file_path: &String) -> Result<BufReader<File>, std::io::Error> {
    let f = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    Ok(BufReader::new(f))
}
