use thiserror::Error;

#[derive(Debug, Error)]
pub enum BmpError {
    #[error("[-] BMP signature error.")]
    SigError(),
    #[error("[-] BMP file too short.
    The file is too short to be a valid bitmap in Windows format.")]
    FileTooShort(),
    #[error(r#"[-] Invalid file length: {0}
    File length found in the header differs from actual file size, which is {1}.
    Header might be corrupted - correct the data and try again."#)]
    FileLengthError(u32, usize),
    #[error(r#"[-] Invalid DIB header size."#)]
    DIBHeaderLengthError()
}

#[derive(Error, Debug)]
pub enum GenericError {
    #[error("[-] BMP analysis aborted")]
    Abort()
}