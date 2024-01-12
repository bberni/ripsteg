use std::io::Error;

use miniz_oxide::inflate::DecompressError;
use thiserror::Error;


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

#[derive(Error, Debug)]
pub enum DumpError {
    #[error(r#"[-] IDAT decompression error: 
    {0}"#)]
    InflateError(DecompressError),

    #[error(r#"[-] Color type not implemented:
    Indexed images can not be parsed as of now."#)]
    IndexedNotImplemented(),

    #[error(r#"[-] Invalid color type:
    Color type: {0} is not a valid color type.
    Parser cannot continue without a valid color type"#)]
    InvalidColorType(u8)
}

#[derive(Error, Debug)]
pub enum FsError {
    #[error(r#"[-] Write error: 
    {0}"#)]
    WriteError(Error)
}

#[derive(Error, Debug)]
pub enum GenericError {
    #[error("[-] PNG analysis aborted")]
    Abort()
}