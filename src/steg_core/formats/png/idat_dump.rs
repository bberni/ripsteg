use std::fs::File;
use std::io::Write;
use anyhow::Result;
use miniz_oxide::inflate;
use crate::OUTPUT_DIR;
use super::errors::{DumpError, FsError};

pub fn idat_dump(idat_vec: &Vec<u8>) -> Result<Vec<u8>> {
    let outfile = format!("{}/idat_dump.bin", OUTPUT_DIR.read().unwrap());
    let decompressed = match inflate::decompress_to_vec_zlib(idat_vec) {
        Ok(dump) => dump,
        Err(x) => {return Err(DumpError::InflateError(x).into())}
    };
    let mut file = File::create(&outfile)?;
    
    match file.write_all(&decompressed) {
        Ok(_) => { 
            println!("[+] IDAT data decompressed and dumped to {}", outfile);
            return Ok(decompressed)
        },
        Err(x) => {return Err(FsError::WriteError(x).into())}
    };
}