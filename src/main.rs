use clap::Parser;
use ripsteg::steg_core;
use std::{process::exit, time};
#[derive(Parser)]
#[command(name = "ripsteg")]
#[command(author = "Michał Bernacki <michalb675@gmail.com>")]
#[command(version = "0.1")]
#[command(about = "Does awesome things", long_about = None)]
struct Cli {
    #[arg(long)]
    input_file: String,
    #[arg(long)]
    file_format: String,
}

fn main() {
    let t1 = time::Instant::now(); 
    let args = Cli::parse();
    match args.file_format.as_str() {
        "png" => match steg_core::formats::png::process_png(args.input_file) {
            Ok(_) => println!("[+] Done! Time: {:?}", t1.elapsed()),
            Err(why) => {
                println!("{}", why);
                exit(1);
            }
        },
        "bmp" => {}
        _ => {
            println!("[-] Only png and bmp formats are currently supported");
            exit(1);
        }
    }
}
