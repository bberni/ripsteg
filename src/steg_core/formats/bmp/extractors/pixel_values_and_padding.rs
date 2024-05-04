use crate::{
    steg_core::{formats::bmp::{
        parser::{Bmp, BmpData},
    }, common::errors::FsError},
    OUTPUT_DIR,
};
use anyhow::Result;
use std::{cmp::Ordering, fs::File, io::Write};

fn calculate_padding(bmp: &Bmp) -> u32 {
    let modulus = (bmp.header.width * (bmp.header.color_depth as u32 / 8)) % 4;
    match modulus {
        0 => return 0,
        _ => return 4 - modulus
    }
}

fn calculate_valid_size(bmp: &Bmp, padding_len: u32) -> u32 {
    return bmp.header.height
        * ((bmp.header.width * (bmp.header.color_depth as u32 / 8)) + padding_len);
}
pub fn pixel_values_and_padding(bmp: &Bmp) -> Result<BmpData> {
    let padding_len = calculate_padding(bmp);
    let valid_size = calculate_valid_size(bmp, padding_len);
    let mut pixel_data: Vec<u8> = Vec::new();
    let mut padding_data: Vec<u8> = Vec::new();
    let mut excess_flag = false;
    let mut cursor: usize = 0;
    let width_byte_size = bmp.header.width * (bmp.header.color_depth / 8) as u32;
    match (valid_size as usize).cmp(&bmp.data.len()) {
        Ordering::Less => {excess_flag = true; println!("[!] Data stored in BMP is longer than valid size calculated from the dimensions of the image - there might be excess data at the end of the file.")},
        Ordering::Greater => println!("[!] Data stored in BMP is shorter than valid size calculated from the dimensions of the image - header or data might be corrupted"),
        _ => ()
    }
    for _ in 0..(&bmp.data.len() / (width_byte_size + padding_len) as usize) {
        pixel_data.append(&mut bmp.data[cursor..cursor + width_byte_size as usize].to_vec());
        cursor += width_byte_size as usize;
        padding_data.append(&mut bmp.data[cursor..cursor + padding_len as usize].to_vec());
        cursor += padding_len as usize;
    }
    let outfile = format!("{}/raw_pixel_values.bin", OUTPUT_DIR.read().unwrap());
    let mut file = File::create(&outfile)?;

    match file.write_all(&pixel_data) {
        Ok(_) => {
            println!("[+] Raw pixel values dumped to {}", outfile);
        }
        Err(x) => return Err(FsError::WriteError(x).into()),
    };
    if padding_data.len() > 0 {
        let outfile = format!("{}/padding_data.bin", OUTPUT_DIR.read().unwrap());
        let mut file = File::create(&outfile)?;

        match file.write_all(&padding_data) {
            Ok(_) => {
                println!("[+] Padding data dumped to {}", outfile);
            }
            Err(x) => return Err(FsError::WriteError(x).into()),
        };
    }
    if excess_flag {
        let excess_data = &bmp.data[cursor..];
        let outfile = format!("{}/excess_data.bin", OUTPUT_DIR.read().unwrap());
        let mut file = File::create(&outfile)?;
        match file.write_all(&excess_data) {
            Ok(_) => {
                println!("[+] Excess data dumped to {}", outfile);
            }
            Err(x) => return Err(FsError::WriteError(x).into()),
        };
    }

    return Ok(BmpData {pixel_data, padding_data});

}
