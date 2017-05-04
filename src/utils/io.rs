use std::fs::File;
use std::io::Error;
use std::io::Read;
use std::path::Path;
use std::result::Result;

pub fn read_bin<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, Error> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut file = try!(File::open(path));
    try!(file.read_to_end(&mut buffer));
    Ok(buffer)
}
