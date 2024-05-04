use std::io::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FsError {
    #[error(r#"[-] Write error: 
    {0}"#)]
    WriteError(Error)
}