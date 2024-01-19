use std::{fs::{File, self}, io::Write};

use crate::{OUTPUT_DIR, steg_core::formats::png::errors::FsError, create_dir_and_file};
use anyhow::Result;

pub fn xsb(mut flags: u8, raw_data: &Vec<u8>) -> Result<()> {
    let mut c = 0;
    while flags != 0 {
        let mut out_vec: Vec<u8> = Vec::new();
        if flags & 1 == 1 {
            let mut to_write: u8 = 0;
            for (index, byte) in raw_data.iter().enumerate() {
                
                if index % 8 != 0 || index == 0 {
                    if byte >> c & 1 == 1 {
                        to_write = (to_write | 1) << 1;
                    } else {
                        to_write = (to_write) << 1;
                    }
                } else if index % 8 == 0 {
                    if byte >> c & 1 == 1 {
                        to_write = to_write | 1;
                    }
                    out_vec.push(to_write);
                    to_write = 0;
                }
                if index == raw_data.len() - 1 {
                    if byte >> c & 1 == 1 {
                        to_write = to_write | 1;
                    }
                    out_vec.push(to_write);
                    to_write = 0;
                }
            }
        }
        let filename = match c {
            0 => "lsb.bin".to_string(),
            8 => "msb.bin".to_string(),
            _ => format!("{}_significant_bit.bin",  8 - c)
        };
        let mut file = create_dir_and_file(&"xsb".to_string(), &filename)?;
        match file.write_all(&out_vec) {
            Ok(_) => { 
                println!("[+] contents of {} significant bit extracted to {}", 8 - c, filename);

            },
            Err(x) => return Err(FsError::WriteError(x).into())
        };
        flags = flags >> 1;
        c += 1;
    }
    Ok(())
}