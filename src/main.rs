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

    let old = hash_file_contents(&args[1]);

    let reader = BufReader::new(File::open(&args[2]).expect(&format!("file not found: {}", &args[2])));
    let mut hasher = Md5::new();

    for (idx, line) in reader.lines().enumerate() {
        let line = line.expect(&format!("cannot read {}@{}", &args[2], idx));
        let line = line.trim();

        hasher.input_str(&line);
        let hashed = hasher.result_str();
        if old.contains(&hashed) {
            println!("{}", line);
        }

        hasher.reset();
    }
}

fn hash_file_contents(path: &String) -> HashSet<String> {
    let mut hs = HashSet::new();
    let mut hasher = Md5::new();

    let reader = BufReader::new(File::open(path).expect(&format!("file not found: {}", path)));

    for (idx, line) in reader.lines().enumerate() {
        let line = line.expect(&format!("cannot read {}@{}", path, idx));
        let line = line.trim();
        
        hasher.input_str(&line);
        let hashed = hasher.result_str();
        
        hs.insert(hashed);
        
        hasher.reset();
    }

    hs
}
