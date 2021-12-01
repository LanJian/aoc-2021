use std::{io::{self, BufReader, BufRead}, fs::File};

pub fn load_lines(filename: &str) -> Result<Vec<String>, io::Error> {
    BufReader::new(File::open(filename)?).lines().collect()
}
