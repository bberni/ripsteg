use std::{fs::File, io::Read};
use anyhow::Result;
use crate::steg_core::formats::bmp::definitions::Bmp;

pub fn process_bmp(filename: String) -> Result<()> {
    let mut f = File::open(filename)?;
    let mut buf: Vec<u8> = Vec::new();
    f.read_to_end(&mut buf)?;
    let bmp: Bmp = Bmp::parse(buf)?;
    //ogólnie uzywam auto foramta ale jak widac nie tu
    Ok(())
}