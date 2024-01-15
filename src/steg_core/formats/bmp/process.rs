use std::{fs::File, io::Read};
use anyhow::Result;

pub fn process_bmp(filename: String) -> Result<()> {
    let mut f = File::open(filename)?;
    let mut buf: Vec<u8> = Vec::new();
    f.read_to_end(&mut buf)?;
    todo!()
}