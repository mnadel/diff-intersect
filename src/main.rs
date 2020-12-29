use std::env;
use std::collections::HashSet;
use std::io::BufRead;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        let prog_name = args[0].split(std::path::MAIN_SEPARATOR).last().unwrap();
        println!("usage: {} <file-1> <file-2>", prog_name);
        std::process::exit(1);
    }

    let reader_one = diff_intersect::buf_reader_from_path(&args[1]);
    let reader_two = diff_intersect::buf_reader_from_path(&args[2]);

    let mut hs = HashSet::new();

    reader_one.lines().for_each(|v| {
        let line = v.unwrap();
        let line = line.trim();

        hs.insert(diff_intersect::hash(&line));
    });

    reader_two.lines().for_each(|v| {
        let line = v.unwrap();
        let line = line.trim();

        if hs.contains(&diff_intersect::hash(&line)) {
            println!("{}", &line); 
        }
    });
}
