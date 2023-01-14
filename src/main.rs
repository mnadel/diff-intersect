use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        let prog_name = args[0].split(std::path::MAIN_SEPARATOR).last().unwrap();
        println!("usage: {} <file-1> <file-2>", prog_name);
        std::process::exit(1);
    }

    let file1 = match File::open(&args[1]) {
        Ok(file) => file,
        Err(_) => {
            println!("file not found: {}", &args[1]);
            std::process::exit(2);
        }
    };

    let file2 = match File::open(&args[2]) {
        Ok(file) => file,
        Err(_) => {
            println!("file not found: {}", &args[2]);
            std::process::exit(2);
        }
    };

    let shorter_file: File;
    let longer_file: File;

    if file1.metadata().unwrap().len() >= file2.metadata().unwrap().len() {
        longer_file = file1;
        shorter_file = file2;
    } else {
        longer_file = file2;
        shorter_file = file1;
    }

    let hashes = diff_intersect::build_hashes(&shorter_file);

    BufReader::new(longer_file).lines().for_each(|v| {
        let line = v.unwrap();
        let line = line.trim();

        if hashes.contains(&diff_intersect::hash(&line)) {
            println!("{}", &line); 
        }
    });
}
