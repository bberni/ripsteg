use std::{fs::{File, self}, io::Write};

use crate::{OUTPUT_DIR, steg_core::formats::png::errors::FsError};
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
        let outfile = format!("{}/xsb/{}_significant_bit.bin", OUTPUT_DIR.read().unwrap(), 8 - c);
        fs::create_dir_all(format!("{}/xsb", OUTPUT_DIR.read().unwrap())).unwrap();
        let mut file = File::create(&outfile)?;
        match file.write_all(&out_vec) {
            Ok(_) => { 
                println!("[+] contents of {} significant bit extracted to {}", 8 - c, outfile);

            },
            Err(x) => return Err(FsError::WriteError(x).into())
        };
        flags = flags >> 1;
        c += 1;
    }
    Ok(())
}