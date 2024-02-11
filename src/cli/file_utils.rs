use std::fs::File;
use std::io::{Read, Write, Result};

pub fn read_file_into_bytes(file_path: &str) -> Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

pub fn write_bytes_to_file(file_path: &str, data: &[u8]) -> Result<()> {
    let mut file = File::create(file_path)?;

    file.write_all(data)?;

    Ok(())
}
