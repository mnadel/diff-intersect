use std::fs::File;
use std::io::BufReader;

use ahash::AHasher;
use std::hash::Hasher;

pub fn hash(s: &str) -> u64 {
    let mut h = AHasher::default();
    h.write(s.as_bytes());
    h.finish()
}

pub fn buf_reader_from_path(file_path: &String) -> Result<BufReader<File>, std::io::Error> {
    let f = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    Ok(BufReader::new(f))
}
