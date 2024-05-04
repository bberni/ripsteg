use std::fmt;
use crate::{yes_no, print_continue, print_continue_anyway};
use anyhow::Result;
use super::{errors::{BmpError, GenericError}, consts::*};

pub struct Header {
    pub file_size: u32,
    pub offset: u32,
    pub width: u32,
    pub height: u32,
    pub color_depth: u16,
}

pub struct BmpData {
    pub pixel_data: Vec<u8>,
    pub padding_data: Vec<u8>
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let color_depth = match self.color_depth {
            24 => "24 bits (RGB24)".to_owned(),
            _ => format!("{} bits (unsupported for now)", self.color_depth)
        };
        write!(f, r#"Image information:
        File Size: {},
        Pixel Array Offset: {},
        Width: {},
        Height: {},
        Bit Depth: {},"#,
    self.file_size,
    self.offset,
    self.width,
    self.height,
    &color_depth)
    }
}
pub struct Bmp {
    pub header: Header,
    pub data: Vec<u8>
}

impl Bmp {
    fn get_le_u32(buffer: &[u8], cursor: usize) -> u32 {
        let bytes: &[u8] = &buffer[cursor..cursor + 4];
        return u32::from_le_bytes(bytes.try_into().unwrap())
    }

    fn get_le_u16(buffer: &[u8], cursor: usize) -> u16{
        let bytes: &[u8] = &buffer[cursor..cursor + 2];
        return u16::from_le_bytes(bytes.try_into().unwrap())
    }

    pub fn parse(buffer: Vec<u8>) -> Result<Bmp> {
        if buffer.len() < 56 {
            return Err(BmpError::FileTooShort().into())
        }
        if !(buffer[0..2] == VALID_BMP_SIGNATURE) {
            println!("[-] Invalid BMP file signature. The program can only parse Windows format bitmaps (starting with \"BM\")");
            print_continue_anyway();
            if !(yes_no()?) {return Err(BmpError::SigError().into())}; 
        }

        let mut cursor = 2;
        let file_size = Self::get_le_u32(&buffer, cursor);
        if (file_size as usize) > buffer.len() {
            return Err(BmpError::FileLengthError(file_size, buffer.len()).into());
        }
        cursor += 4 + 2 + 2;
        let offset = Self::get_le_u32(&buffer, cursor);
        if offset != 0x36 {
            println!(r#"[!] Unusual pixel array offset: {}
            In Windows bitmaps, pixel array offset is usually 52 (2 bytes for signature + 12 for file header + 40 for DIB header).
            This can be a sign of corruption."#, offset);
            print_continue();
            if !(yes_no()?) {return Err(GenericError::Abort().into())}; 
        }
        cursor += 4;
        let dib_size = Self::get_le_u32(&buffer, cursor);
        if dib_size != 0x28 {
            println!("[-] Invalid DIB header size. BITMAPINFOHEADER should have size of 40. Reported size: {}", dib_size);
            print_continue_anyway();
            if !(yes_no()?) {return Err(BmpError::DIBHeaderLengthError().into())}; 
        }
        cursor += 4;
        let width = Self::get_le_u32(&buffer, cursor);
        cursor += 4;
        let height = Self::get_le_u32(&buffer, cursor);
        cursor += 4 + 2;
        let color_depth = Self::get_le_u16(&buffer, cursor);
        cursor += 2;
        let compression_level = Self::get_le_u32(&buffer, cursor);
        if compression_level > 0 {
            println!(r#"[-] Header contains information about BMP compression. This tool currently cannot parse compressed BMP images.
    If you believe, that there is no compression, and header is corrupted, you can continue."#);
            print_continue();
            if !(yes_no()?) {return Err(GenericError::Abort().into())};
        }
        cursor += 4 * 6;
        let data: Vec<u8> = buffer[cursor..].to_vec();
        let header = Header {
            file_size, offset, width, height, color_depth
        };
        println!("[+] BMP parsed successfully\n{}", header);
        return Ok(Bmp { header, data })
    }
}