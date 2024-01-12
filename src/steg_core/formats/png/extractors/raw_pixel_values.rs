use crate::steg_core::formats::png::{
    errors::{DumpError, GenericError},
    parser::{Png, IHDR},
};
use crate::yes_no;
use anyhow::Result;

fn calculate_bpp(ihdr: &IHDR) -> Result<u8> {
    let channels = match ihdr.color_type {
        0 => 1,
        2 => 3,
        4 => 2,
        8 => 4,
        _ => {return Err(DumpError::InvalidBitDepth(ihdr.bit_depth, ihdr.color_type).into());}
    };
    return Ok((channels * ihdr.bit_depth) % 8)
}

fn none_filter(scanline: &[u8]) -> &[u8] {
    return &scanline[1..];
}

fn up_filter(prev_scanline: &[u8], scanline: &[u8]) -> Vec<u8> {
    let mut unfiltered: Vec<u8> = Vec::new();
    for x in 1..scanline.len() {
        unfiltered.push((scanline[x] + prev_scanline[x]) % 255);
    }

    return unfiltered;
}

fn sub_filter(scanline: &[u8], ihdr: &IHDR) -> Result<Vec<u8>>{
    let bpp = calculate_bpp(ihdr)?;
    todo!()
}

pub fn raw_pixel_values(png: Png) -> Result<Vec<u8>> {
    let idat = png.idat_vec;
    let bytes_per_pixel: u32 = match png.ihdr.color_type {
        0 => 1,
        2 => 2,
        4 => 3,
        6 => 4,
        3 => {
            return Err(DumpError::IndexedNotImplemented().into());
        }
        _ => return Err(DumpError::InvalidColorType(png.ihdr.color_type).into()),
    };
    let width = png.ihdr.width;
    let height = png.ihdr.height;
    let correct_idat_size = width * bytes_per_pixel * height + width;
    match correct_idat_size as usize == idat.len() {
        true => {}
        false => {
            println!(
                r#"[!] IDAT data size incorrect!
    IDAT data size: {}, does not match correct size: {}, which is calculated from width, height and color type of the image.
    That can mean that dimensions of the image are incorrect, the color type is incorrect, or there is additional data in IDAT chunk.
    It is suggested that you analyze "idat_dump.bin" before continuing.
    Do you want to continue? (Y/N)"#,
                idat.len(),
                correct_idat_size
            );
            if !(yes_no()?) {
                return Err(GenericError::Abort().into());
            };
        }
    }
    let scanlines: Vec<&[u8]> = idat.as_slice().chunks(width as usize + 1).collect();

    todo!()
}
