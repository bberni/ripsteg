#[derive(Debug)]
pub struct Header {
    format: u16,
    size: u32,
    special1: u16,
    special2: u16,
    offset: u32,
}

#[derive(Debug)]
pub struct Standard {
    name: &'static str,
    size: u8,

}

#[derive(Debug)]
pub struct Info {
    standard: Standard,
    info_size: u32,
    width: u16,
    height: u16,
    planes: u16,
    bits_per_pixel: u16,
}

#[derive(Debug)]
pub struct Bmp {
    header: Header,
    dib_header: Info,

}

impl Standard {
    pub fn new(name: &'static str, size: u8) -> Standard {
        Standard { name: name, size: size }
    }
}

const VALID_BMP_HEADER : [[u8;2];2] = [[0x42,0x4D],[0x42,0x41]];
const STANDARDS: [Standard; 8] = [Standard { name: "BITMAPCOREHEADER/OS21XBITMAPHEADER", size: 12 },
    Standard { name: "OS22XBITMAPHEADER", size: 64 }, Standard { name: "OS22XBITMAPHEADER", size: 16 },
    Standard { name: "BITMAPINFOHEADER", size: 40 }, Standard { name: "BITMAPV2INFOHEADER", size: 52 },
    Standard { name: "BITMAPV3INFOHEADER", size: 56 }, Standard { name: "BITMAPV4HEADER", size: 108 },
    Standard { name: "BITMAPV5HEADER", size: 124 }];