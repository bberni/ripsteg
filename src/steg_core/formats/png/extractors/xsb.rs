use std::io::Write;

use crate::{create_dir_and_file, steg_core::formats::png::errors::FsError};
use anyhow::Result;

pub fn xsb(mut flags: u8, raw_data: &Vec<u8>) -> Result<()> {
    let mut c = 0;
    let mut out_vec = Vec::new();
    while flags != 0 {
        if flags & 1 == 1 {
            let mut to_write: u8 = 0;
            for (index, byte) in raw_data.iter().enumerate() {
                if (byte >> c) & 1 == 1 {
                    to_write |= 1;
                }
                if index % 8 == 7 || index == raw_data.len() - 1 {
                    out_vec.push(to_write);
                    to_write = 0;
                } else {
                    to_write <<= 1;
                }
            }
        }
        let filename = match c {
            0 => "lsb.bin".to_string(),
            7 => "msb.bin".to_string(),
            _ => format!("{}_significant_bit.bin", 8 - c),
        };
        let mut file = create_dir_and_file(&"xsb".to_string(), &filename)?;
        match file.write_all(&out_vec) {
            Ok(_) => {
                println!(
                    "[+] Contents of {} significant bit extracted to {}",
                    8 - c,
                    filename
                );
            }
            Err(x) => return Err(FsError::WriteError(x).into()),
        };
        flags = flags >> 1;
        c += 1;
        out_vec.clear();
    }
    Ok(())
}
