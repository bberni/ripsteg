use crate::steg_core::formats::bmp::definitions;
use definitions::Bmp;
use anyhow::Result;
use crate::steg_core::formats::bmp::definitions::Header;

impl Bmp {
    pub fn parse(buffer : Vec<u8>) -> Result<Bmp>{
        let header : Header;
        todo!()
    }
}

