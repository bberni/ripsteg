use std::{io, sync::RwLock};
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