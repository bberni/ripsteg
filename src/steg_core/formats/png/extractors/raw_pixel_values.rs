use std::{fs::File, io::Write};
use png::filter;
use crate::{
    steg_core::formats::png::{
        errors::{DumpError, GenericError, FsError},
        parser::{Png, IHDR}
    },
    yes_no, OUTPUT_DIR,
};
use anyhow::Result;


fn calculate_bpp(ihdr: &IHDR) -> Result<u8> {
    let channels = match ihdr.color_type {
        0 => 1,
        2 => 3,
        4 => 2,
        6 => 4,
        _ => {
            return Err(DumpError::InvalidBitDepth(ihdr.bit_depth, ihdr.color_type).into());
        }
    };
    return Ok((channels * ihdr.bit_depth) / 8);
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

fn none_filter(scanline: &[u8]) -> Vec<u8> {
    return scanline.to_vec();
}

fn sub_filter(scanline: &[u8], bpp: &u8) -> Result<Vec<u8>> {
    let mut unfiltered: Vec<u8> = Vec::new();
    for x in 0..scanline.len() {
        match x as isize - *bpp as isize {
            result if result >= 0 => {unfiltered.push(scanline[x].wrapping_add(scanline[result as usize]));},
            _ => {unfiltered.push(scanline[x]);}
        };
    }
    return Ok(unfiltered)
}

fn up_filter(prev_scanline: &Vec<u8>, scanline: &[u8]) -> Vec<u8> {
    let mut unfiltered: Vec<u8> = Vec::new();
    for x in 0..scanline.len() {
        unfiltered.push(scanline[x].wrapping_add(prev_scanline[x]));
    }

    return unfiltered;
}

fn average_filter(prev_scanline: &Vec<u8>, scanline: &[u8], bpp: &u8) -> Vec<u8> {
    let mut unfiltered: Vec<u8> = Vec::new();
    for x in 0..scanline.len() {
        match x as isize - *bpp as isize {
            result if result >= 0 => {
                let average_x = scanline[result as usize] as i32;
                unfiltered.push(scanline[x].wrapping_add((((average_x + prev_scanline[x] as i32) as f64) / 2.0) as u8));
            },
            _ => {
                unfiltered.push(scanline[x].wrapping_add((((0 + prev_scanline[x] as i32) as f64) / 2.0) as u8));
            }
        };
    }
    return unfiltered;
}

fn paeth_filter(prev_scanline: &Vec<u8>, scanline: &[u8], bpp: &u8) -> Result<Vec<u8>> {
    let mut unfiltered: Vec<u8> = Vec::new();
    for x in 0..scanline.len() {
        match x as isize - *bpp as isize {
            result if result >= 0 => {
                let a = scanline[result as usize] as i32;
                let b = prev_scanline[x as usize] as i32;
                let c = prev_scanline[result as usize] as i32;
                unfiltered.push(scanline[x].wrapping_add(paeth_predictor(a, b, c)?))
            },
            _ => {
                let a = 0;
                let b = prev_scanline[x as usize] as i32;
                let c = 0;
                unfiltered.push(scanline[x].wrapping_add(paeth_predictor(a, b, c)?))
            }
        }
    }
    return Ok(unfiltered);
}

pub fn raw_pixel_values(png: Png, idat_dump: Vec<u8>) -> Result<Vec<u8>> {
    let idat = idat_dump;
    println!("{:?}", png.ihdr);
    let bytes_per_pixel: u32 = match png.ihdr.color_type {
        0 => 1,
        2 => 3,
        4 => 2,
        6 => 4,
        3 => {
            return Err(DumpError::IndexedNotImplemented().into());
        }
        _ => return Err(DumpError::InvalidColorType(png.ihdr.color_type).into()),
    };
    let bpp = calculate_bpp(&png.ihdr)?;
    let width = png.ihdr.width;
    let height = png.ihdr.height;
    let correct_idat_size = width * bytes_per_pixel * height + height;
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
    let total_width = width as usize * bytes_per_pixel as usize + 1;
    let scanlines: Vec<&[u8]> = idat.as_slice().chunks(total_width).collect();
    // println!("{:?}", scanlines);
    let check = match scanlines.last() {
        Some(len) => len,
        None => return Err(DumpError::EmptyIDAT().into()),
    };
    if check.len() != total_width {
        println!(r#"[!] Invalid dimensions error:
    The IDAT scanlines are incorrectly aligned (length of the last scanline is too small)
    This will probably cause corruption of raw bytes of image (filters will be incorrect)
    You should try to fix the corrupted header first.
    Do you want to continue anyway? (Y/n)"#);
        if !(yes_no()?) {return Err(DumpError::InvalidDimensions().into())};
    }
    let empty_scanline = vec![0 as u8; total_width];
    let mut raw_pixel_values: Vec<Vec<u8>> = Vec::new();
    for (index, scanline) in scanlines.iter().enumerate() {
        let prev_scanline = match index {
            0 => &empty_scanline,
            _ => raw_pixel_values.last().unwrap() // if index > 1, this cannot fail
        };
        let filter_type = scanline[0];
        let scanline_no_filter = &scanline[1..];
        // match filter_type {
        //     1 => {println!("sub")}
        //     2 => {println!("up")}
        //     3 => {println!("avg")}
        //     4 => {println!("paeth")}
        //     _ => {}// temp
        // }
        println!("{:?}", bpp);
        let raw_scanline = match filter_type {
            0 => none_filter(scanline_no_filter),
            1 => sub_filter(scanline_no_filter, &bpp)?,
            2 => up_filter(prev_scanline, scanline_no_filter),
            3 => average_filter(prev_scanline, scanline_no_filter, &bpp),
            4 => paeth_filter(prev_scanline, scanline_no_filter, &bpp)?,
            _ => Vec::new() // temp
        };
        raw_pixel_values.push(raw_scanline);
    }
    let raw_pixel_values = raw_pixel_values.concat();
    let outfile = format!("{}/raw_pixel_values.bin", OUTPUT_DIR.read().unwrap());
    let mut file = File::create(&outfile)?;

    match file.write_all(&raw_pixel_values) {
        Ok(_) => { 
            println!("[+] Raw pixel values dumped to {}", outfile);
            return Ok(raw_pixel_values)
        },
        Err(x) => return Err(FsError::WriteError(x).into())
    };
}
