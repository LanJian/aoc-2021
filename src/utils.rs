use std::{io::{self, BufReader, BufRead}, fs::File, env::{VarError, self}};

pub fn load_input(default_path: &str) -> Result<Vec<String>, io::Error> {
    let path = env::var("AOC_INPUT").unwrap_or(default_path.to_string());
    load_lines(&path)
}

pub fn load_lines(path: &str) -> Result<Vec<String>, io::Error> {
    BufReader::new(File::open(path)?).lines().collect()
}
