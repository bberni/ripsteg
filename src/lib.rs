use std::io;
use anyhow::Result;

pub mod steg_core;

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