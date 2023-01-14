use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashSet;

use ahash::AHasher;
use std::hash::Hasher;

pub fn hash(s: &str) -> u64 {
    let mut h = AHasher::default();
    h.write(s.as_bytes());
    h.finish()
}

pub fn build_hashes(f: &File) -> HashSet<u64> {
    let mut hs = HashSet::new();

    BufReader::new(f).lines().for_each(|v| {
        let line = v.unwrap();
        let line = line.trim();

        hs.insert(hash(&line));
    });

    return hs;
}
