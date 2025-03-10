use std::fs::File;
use std::io::Error;
use std::io::Read;
use std::path::Path;
use std::result::Result;

pub fn read_binary<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, Error> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut file = File::open(path)?;
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
