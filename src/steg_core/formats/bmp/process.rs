use crate::steg_core::{
    common::extractors::xsb::xsb,
    formats::bmp::{extractors::pixel_values_and_padding::pixel_values_and_padding, parser::Bmp},
};
use anyhow::{Ok, Result};
use std::{fs::File, io::Read};

pub fn process_bmp(filename: String) -> Result<()> {
    let mut f = File::open(filename)?;
    let mut buf: Vec<u8> = Vec::new();
    f.read_to_end(&mut buf)?;
    let bmp: Bmp = Bmp::parse(buf)?;
    let data = pixel_values_and_padding(&bmp)?;
    xsb(255, &data.pixel_data)?;
    Ok(())
}
