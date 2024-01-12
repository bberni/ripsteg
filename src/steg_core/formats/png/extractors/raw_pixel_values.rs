use super::super::{parser::Png, errors::DumpError};
use anyhow::Result;
pub fn raw_pixel_values(png: Png) -> Result<Vec<u8>> {
    let bytes_per_pixel: u8 = match png.ihdr.color_type {
        0 => 1,
        2 => 2,
        4 => 3,
        6 => 4,
        3 => {return Err(DumpError::IndexedNotImplemented().into());},
        _ => {return Err(DumpError::InvalidColorType(png.ihdr.color_type).into())}
    };
    
    todo!()
}