use anyhow::Result;
use crc32fast::Hasher;
use super::{errors::PngError, consts::*};
use crate::yes_no;

#[derive(Debug)]
pub struct IHDR {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8,
    pub color_type: u8,
    pub compression_method: u8,
    pub filter_method: u8,
    pub interlace_method: u8,
}

#[derive(Debug)]
pub struct Chunk {
    chunk_name: String,
    data: Vec<u8>,
    crc32: u32
}

#[derive(Debug)]
pub struct Png {
    pub ihdr: IHDR,
    pub idat_vec: Vec<u8>,
    pub all_chunks: Vec<Chunk>
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
            Some(ihdr) => {
                println!("[+] PNG parsed successfully:\n    {:?}", ihdr);
                let idat_vec: Vec<u8> = idat_vec.into_iter().flatten().collect();
                return Ok(Png {ihdr, idat_vec, all_chunks})
            }
            None => {return Err(PngError::NoIHDRError().into())}
        }
    }
}