use std::fs::File;
use std::io::Write;
use anyhow::Result;
use miniz_oxide::inflate;
use super::consts::*;
use super::errors::{DumpError, FsError};

pub fn idat_dump(idat_vec: Vec<Vec<u8>>) -> Result<Vec<u8>> {
    let outfile = format!("{}/idat_dump.bin", OUTPUT_DIR);
    let flattened_idat: Vec<u8> = idat_vec.into_iter().flatten().collect();
    let decompressed = match inflate::decompress_to_vec_zlib(&flattened_idat) {
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