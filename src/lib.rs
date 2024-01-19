use std::{io, sync::RwLock, fs::{File, self}};
use anyhow::Result;
use lazy_static::lazy_static;

pub mod steg_core;

lazy_static! {
    pub static ref OUTPUT_DIR: RwLock<String> = RwLock::new(String::new());
}

pub fn yes_no() -> Result<bool> {
    let mut user_input = String::new();
    loop {
        io::stdin().read_line(&mut user_input)?;
        let user_input = user_input.trim();
        match user_input {
            "Y" | "y" => return Ok(true),
            "N" | "n" => return Ok(false),
            _ => println!("Invalid input!"),
        }
    }
}

pub fn create_dir_and_file(dir: &String, filename: &String) -> Result<File> {
    let outfile = format!("{}/{}/{}", OUTPUT_DIR.read().unwrap(), dir, filename);
    fs::create_dir_all(format!("{}/{}", OUTPUT_DIR.read().unwrap(), dir))?;
    let file = File::create(&outfile)?;
    return Ok(file);
}