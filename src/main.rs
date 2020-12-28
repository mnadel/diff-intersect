use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::os::macos::fs::MetadataExt;

use fasthash::metro;

const CONFIG_DIFFSECT_THRESH: &str = "DIFFSECT_THRESH";

const ERR_USAGE: i32 = 1;
const ERR_FILENOTFOUND: i32 = 2;
const ERR_FILESTAT: i32 = 3;
const ERR_FILEIO: i32 = 4;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        let prog_name = args[0].split(std::path::MAIN_SEPARATOR).last().unwrap();
        println!("usage: {} <file-1> <file-2>", prog_name);
        std::process::exit(ERR_USAGE);
    }

    let reader_one = buf_reader(&args[1]);
    let reader_two = buf_reader(&args[2]);

    let mut hs = new_hashset(&args[1]);
    process_stream(reader_one, &mut hs, true);
    process_stream(reader_two, &mut hs, false);
}

fn buf_reader(file_path: &String) -> BufReader<File> {
    let f = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            println!("file `{}` not found: {}", file_path, e);
            std::process::exit(ERR_FILENOTFOUND);
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

fn new_hashset(file_path: &String) -> HashSet<u64> {
    let mut hs = HashSet::new();

    let threshold: u64 = match std::env::var(CONFIG_DIFFSECT_THRESH) {
        Ok(v) => {
            let val = std::env::var(CONFIG_DIFFSECT_THRESH).unwrap();
            let msg = format!("cannot parse `{}`", val);
            v.parse::<u64>().expect(&msg)
        },
        _ => 10_000_000,
    };

    let metadata = match std::fs::metadata(file_path) {
        Ok(metadata) => metadata,
        Err(e) => {
            println!("cannot stat `{}`: {}", file_path, e);
            std::process::exit(ERR_FILESTAT);
        }
    };

    if metadata.st_size() > threshold {
        match File::open(file_path) {
            Ok(f) => {
                let lines_in_file = BufReader::new(f).lines().count();
                hs.reserve(lines_in_file);
            },
            Err(e) => {
                println!("cannot read from `{}`: {}", file_path, e);
                std::process::exit(ERR_FILEIO);
            }
        };    
    }

    hs
}