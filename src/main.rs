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

    let old = md5_from(&args[1]);
    let mut hasher = Md5::new();
    
    let mapper = |v: std::result::Result<String, std::io::Error>| {
        let line = v.unwrap();
        let line = line.trim();
        
        hasher.input_str(&line);
        let hashed = hasher.result_str();
        if old.contains(&hashed) {
            println!("{}", line);
        }
        
        hasher.reset();
    };

    buf_reader(&args[2]).lines().for_each(mapper);
}

fn buf_reader(file_path: &String) -> BufReader<File> {
    let f = File::open(file_path).expect(&format!("file not found: {}", &file_path));
    BufReader::new(f)
}

fn md5_from(path: &String) -> HashSet<String> {
    let mut hs = HashSet::new();
    let mut hasher = Md5::new();

    let mapper = |v: std::result::Result<String, std::io::Error>| {
        let line = v.unwrap();
        let line = line.trim();
        
        hasher.input_str(&line);
        let hashed = hasher.result_str();
        
        hs.insert(hashed);
        
        hasher.reset();
    };

    buf_reader(path).lines().for_each(mapper);

    hs
}
