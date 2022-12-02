use std::fs::File;
use std::io::{read_to_string, BufReader};
use std::path::PathBuf;

pub fn read_file(path: PathBuf) -> std::io::Result<String> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    Ok(read_to_string(reader)?.trim().to_string())
}
