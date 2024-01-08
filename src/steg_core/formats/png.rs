use std::{fs::File, io::Read};
use crc32fast::Hasher;
use anyhow::Result;
use thiserror::Error;
use crate::yes_no;

const VALID_CHUNK_NAMES: [&str; 20] = ["IHDR", "PLTE", "IDAT", "IEND", "bKGD", "cHRM", "dSIG", "eXIf", "gAMA", "hIST", "iCCP", "iTXt", "pHYs", "sBIT", "sPLT", "sRGB", "sTER", "tEXt", "tIME", "tRNS"];
const VALID_PNG_SIGNATURE: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

#[derive(Error, Debug)]
pub enum PngError {
    #[error("[-] PNG signature error.")]
    SigError(),
    #[error(r#"[-] Chunk length error at offset: {1}
    Chunk length (0x{0:08X}) is bigger than the remaining image size, which is impossible.
    Check if the length is at correct offset.
    If it is, correct the length of the chunk and try again."#)]
    ChunkLengthError(u32, usize),
    #[error(r#"[-] Wrong chunk name: {0} at offset {1}
    {0} is not a valid PNG chunk name. This means that either:
        a) The length of the previous chunk is corrupted
        b) The name of this chunk is corrupted
        c) The whole image is corrupted
    Parser cannot continue without valid chunk name."#)]
    ChunkNameError(String, usize),

    #[error(r#"[-] IHDR chunk length invalid: {0}
    Size of the IHDR length must be exactly 13 bytes.
    Please correct the length of the IHDR header and try again."#)]
    IHDRLengthError(usize),
    #[error("[-] No IHDR present, cannot properly parse the image.")]
    NoIHDRError(),

    #[error("[-] CRC32 mismatch of chunk {0} at offset {1}, aborting.")]
    CRC32Error(String, usize)

}

#[derive(Debug)]
struct IHDR {
    width: u32,
    height: u32,
    bit_depth: u8,
    color_type: u8,
    compression_method: u8,
    filter_method: u8,
    interlace_method: u8,
}

#[derive(Debug)]
struct Chunk {
    chunk_name: String,
    data: Vec<u8>,
    crc32: u32
}

#[derive(Debug)]
struct Png {
    ihdr: IHDR,
    idat_vec: Vec<Vec<u8>>,
    all_chunks: Vec<Chunk>
}

impl Png {
    fn get_ihdr_data(chunk: &Chunk) -> IHDR {
        IHDR {
            width: u32::from_be_bytes(chunk.data[0..4].try_into().unwrap()),
            height: u32::from_be_bytes(chunk.data[4..8].try_into().unwrap()),
            bit_depth: chunk.data[8],
            color_type: chunk.data[9],
            compression_method: chunk.data[10],
            filter_method: chunk.data[11],
            interlace_method: chunk.data[12]
        }
    }
    fn verify_crc(chunk: &Chunk, offset: usize) -> Result<()> {
        let mut hasher = Hasher::new();
        hasher.update(chunk.chunk_name.as_bytes());
        hasher.update(chunk.data.as_slice());
        let correct_crc = hasher.finalize();
        if correct_crc == chunk.crc32 {
            return Ok(());
        } else {
            println!(r#"[!] Incorrect CRC32 checksum of {} chunk at offset {}: 0x{:08X} (should be 0x{:08X})
Do you want to continue? (Y/N) "#, chunk.chunk_name, offset - chunk.data.len() - 8, chunk.crc32, correct_crc);
            if !(yes_no()?) {return Err(PngError::CRC32Error(chunk.chunk_name.clone(), offset - chunk.data.len() - 8).into())} else {return Ok(())};
        }
    }

    pub fn parse(buffer: Vec<u8>) -> Result<Png> {

        let mut ihdr: Option<IHDR> = None;
        let mut idat_vec: Vec<Vec<u8>> = Vec::new();
        let mut all_chunks: Vec<Chunk> = Vec::new();
        if !(buffer[0..8] == VALID_PNG_SIGNATURE) {
            println!("[-] Invalid PNG file signature. There is a significant chance that the file is not in PNG format / is corrupted");
            print!("Do you want to proceed anyway? (Y/N) ");
            if !(yes_no()?) {return Err(PngError::SigError().into())}; 
        }
        let mut cursor = 8;
        while cursor + 12 <= buffer.len() {
            let length_bytes = &buffer[cursor..cursor + 4];
            let length = u32::from_be_bytes(length_bytes.try_into().unwrap());
            match (length as usize) > (buffer.len() - cursor) {
                true => return Err(PngError::ChunkLengthError(length, cursor).into()),
                false => (),
            }
            cursor += 4;
            let name_bytes= &buffer[cursor..cursor + 4];
            let chunk_name = String::from_utf8_lossy(name_bytes).into_owned();
            if !(VALID_CHUNK_NAMES.contains(&chunk_name.as_str())) {
                return Err(PngError::ChunkNameError(chunk_name, cursor).into());
            } 
            cursor += 4;
            let data_bytes = &buffer[cursor..cursor + length as usize];
            let data: Vec<u8> = Vec::from(data_bytes.to_vec());
            cursor += length as usize;
            let crc_bytes = &buffer[cursor..cursor + 4];
            cursor += 4;
            let crc32 = u32::from_be_bytes(crc_bytes.try_into().unwrap());
            match chunk_name.as_str() {
                "IHDR" => {
                    let ihdr_chunk = Chunk {chunk_name, data, crc32};
                    Png::verify_crc(&ihdr_chunk, cursor)?;
                    if ihdr_chunk.data.len() != 13 {return Err(PngError::IHDRLengthError(ihdr_chunk.data.len()).into())}
                    ihdr = Some(Png::get_ihdr_data(&ihdr_chunk));
                    all_chunks.push(ihdr_chunk);
                },
                "IDAT" => {
                    let idat_chunk = Chunk {chunk_name, data, crc32};
                    Png::verify_crc(&idat_chunk, cursor)?;
                    idat_vec.push(idat_chunk.data.clone());
                    all_chunks.push(idat_chunk);
                },
                "IEND" => {
                    let iend_chunk = Chunk {chunk_name, data, crc32};
                    Png::verify_crc(&iend_chunk, cursor)?;
                    if iend_chunk.data.len() != 0 {println!("[!] IEND chunk length > 0");}
                    all_chunks.push(iend_chunk);
                },
                _ => {
                    let other_chunk = Chunk {chunk_name, data, crc32};
                    Png::verify_crc(&other_chunk, cursor)?;
                    all_chunks.push(other_chunk);
                }

            }
        }
        match ihdr {
            Some(ihdr) => {return Ok(Png {ihdr, idat_vec, all_chunks})}
            None => {return Err(PngError::NoIHDRError().into())}
        }
    }
}


pub fn process_png(filename: String) -> Result<()> {
    let mut f = File::open(filename)?;
    let mut buf: Vec<u8> = Vec::new();
    f.read_to_end(&mut buf)?;
    let png: Png = Png::parse(buf)?;
    println!("[+] PNG parsed successfully:\n    {:?}", png.ihdr);
    Ok(())
}