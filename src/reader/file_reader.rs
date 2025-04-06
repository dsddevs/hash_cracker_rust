use std::error::Error;
use std::fs::File;
use std::io::BufReader;

pub struct FileReader;

impl FileReader {
    pub fn from_path(file_path: &str) -> Result<BufReader<File>, Box<dyn Error>> {
        let file = File::open(file_path)?;
        Ok(BufReader::new(file))
    }
}