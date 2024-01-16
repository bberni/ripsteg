use std::{fs::File, io::Read};
use anyhow::Result;
use crate::steg_core::formats::png::idat_dump::idat_dump;

use super::{parser::Png, extractors::{raw_pixel_values::raw_pixel_values, xsb::xsb}};


pub fn process_png(filename: String) -> Result<()> {
    let mut f = File::open(filename)?;
    let mut buf: Vec<u8> = Vec::new();
    f.read_to_end(&mut buf)?;
    let png: Png = Png::parse(buf)?;
    let dump = idat_dump(&png.idat_vec)?;
    let raw_pixel_values = raw_pixel_values(png, dump)?;
    xsb(255, &raw_pixel_values)?;
    Ok(())
}