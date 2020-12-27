use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
    if env::args().len() != 3 {
        println!("usage: diff-intersect <old file> <new file>");
        std::process::exit(1);
    }

    let args: Vec<String> = env::args().collect();

    let old_reader = buf_reader(&args[1]);
    let new_reader = buf_reader(&args[2]);

    let old_hashes = md5_from(old_reader);
    let mut hasher = Md5::new();
    
    let mapper = |v: std::result::Result<String, std::io::Error>| {
        let line = v.unwrap();
        let line = line.trim();
        
        hasher.input_str(&line);
        if old_hashes.contains(&hasher.result_str()) {
            println!("{}", line);
        }
        
        hasher.reset();
    };

    new_reader.lines().for_each(mapper);
}

fn buf_reader(file_path: &String) -> BufReader<File> {
    let f = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            println!("file `{}` not found: {}", file_path, e);
            std::process::exit(2);
        }
    };
    BufReader::new(f)
}

fn md5_from(reader: BufReader<File>) -> HashSet<String> {
    let mut hs = HashSet::new();
    let mut hasher = Md5::new();

    let mapper = |v: std::result::Result<String, std::io::Error>| {
        let line = v.unwrap();
        let line = line.trim();
        
        hasher.input_str(&line);
        hs.insert(hasher.result_str());
        hasher.reset();
    };

    reader.lines().for_each(mapper);

    hs
}
