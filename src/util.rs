use std::fs::File;
use std::io::{read_to_string, BufRead, BufReader};
use std::path::PathBuf;

fn buffered_file(path: PathBuf) -> std::io::Result<BufReader<File>> {
    let f = File::open(path)?;
    Ok(BufReader::new(f))
}

pub fn read_file(path: PathBuf) -> std::io::Result<String> {
    let reader = buffered_file(path)?;
    Ok(read_to_string(reader)?.trim().to_string())
}

pub fn lines_for_file(path: PathBuf) -> std::io::Result<Vec<String>> {
    let reader = buffered_file(path)?;
    Ok(reader
        .lines()
        .map(|line| line.expect("line contains invalid UTF-8"))
        .collect())
}
