use crate::{
    steg_core::formats::png::{
        errors::{DumpError, GenericError},
        parser::{Png, IHDR},
    },
    yes_no,
};
use anyhow::Result;


fn calculate_bpp(ihdr: &IHDR) -> Result<u8> {
    let channels = match ihdr.color_type {
        0 => 1,
        2 => 3,
        4 => 2,
        8 => 4,
        _ => {
            return Err(DumpError::InvalidBitDepth(ihdr.bit_depth, ihdr.color_type).into());
        }
    };
    return Ok((channels * ihdr.bit_depth) % 8);
}

fn paeth_predictor(a: i32, b: i32, c: i32) -> Result<u8> {

    let p = a + b - c;        
    let pa = (p - a).abs();      
    let pb = (p - b).abs();
    let pc = (p - c).abs();
    if pa <= pb && pa <= pc {
        return Ok(a.try_into()?);
    }
    else if pb <= pc {
        return Ok(b.try_into()?);
    }
    else {return Ok(c.try_into()?);}
}

fn none_filter(scanline: &[u8]) -> &[u8] {
    return &scanline[..];
}

fn sub_filter(scanline: &[u8], ihdr: &IHDR, bpp: &u8) -> Result<Vec<u8>> {
    let mut unfiltered: Vec<u8> = Vec::new();
    for x in 0..scanline.len() {
        match x as isize - *bpp as isize {
            result if result >= 0 => {unfiltered.push(scanline[x].wrapping_add(scanline[result as usize]));},
            _ => {unfiltered.push(scanline[x]);}
        };
    }
    return Ok(unfiltered)
}

fn up_filter(prev_scanline: &[u8], scanline: &[u8]) -> Vec<u8> {
    let mut unfiltered: Vec<u8> = Vec::new();
    for x in 0..scanline.len() {
        unfiltered.push(scanline[x].wrapping_add(prev_scanline[x]));
    }

    return unfiltered;
}

fn average_filter(prev_scanline: &[u8], scanline: &[u8], bpp: &u8) -> Vec<u8> {
    let mut unfiltered: Vec<u8> = Vec::new();
    for x in 0..scanline.len() {
        match x as isize - *bpp as isize {
            result if result >= 0 => {
                unfiltered.push(scanline[x].wrapping_add((((scanline[result as usize] + prev_scanline[x]) as f64) / 2.0) as u8));
            },
            _ => {
                unfiltered.push(scanline[x].wrapping_add((((0 + prev_scanline[x]) as f64) / 2.0) as u8));
            }
        };
    }
    return unfiltered;
}

fn paeth_filter(prev_scanline: &[u8], scanline: &[u8], bpp: &u8) -> Result<Vec<u8>> {
    let mut unfiltered: Vec<u8> = Vec::new();
    for x in 0..scanline.len() {
        match x as isize - *bpp as isize {
            result if result >= 0 => {
                let a = scanline[result as usize] as i32;
                let b = prev_scanline[x as usize] as i32;
                let c = prev_scanline[result as usize] as i32;
                unfiltered.push(scanline[x].wrapping_add(scanline[x] + paeth_predictor(a, b, c)?))
            },
            _ => {
                let a = 0;
                let b = prev_scanline[x as usize] as i32;
                let c = 0;
                unfiltered.push(scanline[x].wrapping_add(scanline[x] + paeth_predictor(a, b, c)?))
            }
        }
    }
    return Ok(unfiltered);
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
