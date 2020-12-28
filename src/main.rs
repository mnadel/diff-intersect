use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

use fasthash::metro;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        let prog_name = args[0].split(std::path::MAIN_SEPARATOR).last().unwrap();
        println!("usage: {} <old file> <new file>", prog_name);
        std::process::exit(1);
    }

    let old_reader = buf_reader(&args[1]);
    let new_reader = buf_reader(&args[2]);

    let mut hs = HashSet::new();
    process_stream(old_reader, &mut hs, true);
    process_stream(new_reader, &mut hs, false);
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

fn process_stream(reader: BufReader<File>, hs: &mut HashSet<u64>, insert: bool) {
    reader.lines().for_each(|v: std::result::Result<String, std::io::Error>| {
        let line = v.unwrap();
        let line = line.trim();

        let hashed = metro::hash64(&line);
        
        if insert {
            hs.insert(hashed);
        } else if hs.contains(&hashed) {
            println!("{}", line);
        }
    });
}
