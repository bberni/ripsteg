use crate::steg_core::formats::bmp::parser::{Bmp, BmpData};
use anyhow::Result;


fn calculate_padding(bmp: &Bmp) -> u32 {
    return 4 - ((bmp.header.width * (bmp.header.color_depth as u32 / 8)) % 4)
}

fn calculate_valid_size(bmp: &Bmp, padding_len: u32) -> u32 {
    return bmp.header.height * ((bmp.header.width * (bmp.header.color_depth as u32 / 8)) + padding_len);
}
pub fn pixel_values_and_padding(bmp: Bmp) -> Result<BmpData> {
    todo!()
}