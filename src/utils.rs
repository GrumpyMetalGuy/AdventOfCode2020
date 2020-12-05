use std::fs::File;
use std::io::{BufRead, BufReader, Read, Result};

pub fn read_file(filename: &str) -> String {
    let mut output = String::new();

    File::open(filename)
        .expect("Unable to open file")
        .read_to_string(&mut output)
        .expect("Unable to read filename");

    output
}

pub fn lines_from_file(filename: &str) -> Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}
