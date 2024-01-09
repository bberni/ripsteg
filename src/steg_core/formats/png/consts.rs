pub const VALID_CHUNK_NAMES: [&str; 20] = ["IHDR", "PLTE", "IDAT", "IEND", "bKGD", "cHRM", "dSIG", "eXIf", "gAMA", "hIST", "iCCP", "iTXt", "pHYs", "sBIT", "sPLT", "sRGB", "sTER", "tEXt", "tIME", "tRNS"];
pub const VALID_PNG_SIGNATURE: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
pub const OUTPUT_DIR: &str = "ripsteg_out";

