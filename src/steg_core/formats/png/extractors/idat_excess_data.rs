use std::io::Write;

use anyhow::Result;

use crate::create_dir_and_file;


pub fn idat_excess_data(idat_dump: &Vec<u8>, correct_idat_len: usize) -> Result<()> {
    let excess_data = &idat_dump[correct_idat_len..];
    let filename = String::from("idat_excess_data.bin");
    let mut file = create_dir_and_file(&"".to_string(), &filename)?;
    file.write_all(excess_data)?;
    println!("[+] Excess data from IDAT chunk saved to {}", filename);
    Ok(())
}