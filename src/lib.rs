use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

pub fn read_lines<P: AsRef<Path>>(p: P) -> impl Iterator<Item=String> {
    BufReader::new(File::open(p).unwrap())
        .lines()
        .map(Result::unwrap)
}