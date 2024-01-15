use clap::Parser;
use ripsteg::steg_core;
use ripsteg::OUTPUT_DIR;
use std::{process::exit, time, fs::create_dir_all};
#[derive(Parser)]
#[command(name = "ripsteg")]
#[command(author = "Michał Bernacki <michalb675@gmail.com>")]
#[command(version = "0.1")]
#[command(about = "Does awesome things", long_about = None)]
struct Cli {
    #[arg(action)]
    input_file: String,
    #[arg(action)]
    file_format: String,
    #[arg(action)]
    output_folder: String
}

fn main() {
    let t1 = time::Instant::now(); 
    let args = Cli::parse();


    match create_dir_all(&args.output_folder) {
        Ok(_) => {
            *OUTPUT_DIR.write().unwrap() = args.output_folder;
        },
        Err(e) => {
            println!(r#"[-] Error creating a folder {}:
    {}"#, args.output_folder, e);
            exit(1);
        }
    }
    match args.file_format.as_str() {
        "png" => match steg_core::formats::png::process::process_png(args.input_file) {
            Ok(_) => println!("[+] Done! Time: {:?}", t1.elapsed()),
            Err(why) => {
                println!("{}", why);
                exit(1);
            }
        },
        "bmp" => match steg_core::formats::bmp::process::process_bmp(args.input_file) {
            Ok(_) => println!("[+] Done! Time: {:?}", t1.elapsed()),
            Err(why) => {
                println!("{}", why);
                exit(1);
            }
        }
        _ => {
            println!("[-] Only png and bmp formats are currently supported");
            exit(1);
        }
    }
}
