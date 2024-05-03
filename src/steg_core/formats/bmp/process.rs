use super::parser::Bmp;
use anyhow::Result;
use std::{fs::File, io::Read};

pub fn process_bmp(filename: String) -> Result<()> {
    let mut f = File::open(filename)?;
    let mut buf: Vec<u8> = Vec::new();
    f.read_to_end(&mut buf)?;
    let bmp: Bmp = Bmp::parse(buf)?;
    Ok(())
}
