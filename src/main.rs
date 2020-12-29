use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

use fasthash::metro;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        let prog_name = args[0].split(std::path::MAIN_SEPARATOR).last().unwrap();
        println!("usage: {} <file-1> <file-2>", prog_name);
        std::process::exit(1);
    }

    let reader_one = buf_reader(&args[1]);
    let reader_two = buf_reader(&args[2]);

    let mut hs = HashSet::new();

    reader_one.lines().map(normalize).for_each(|line| {
        hs.insert(metro::hash64(&line));
    });

    reader_two.lines().map(normalize).for_each(|line| {
        if hs.contains(&metro::hash64(&line)) {
            println!("{}", &line); 
        }
    });
}

fn normalize(v: std::result::Result<String, std::io::Error>) -> String {
    String::from(v.unwrap().trim())
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
