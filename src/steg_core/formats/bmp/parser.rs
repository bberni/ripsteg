#[derive(Debug)]
pub struct Header{
    format : u16,
    size : u32,
    special1 : u16,
    special2 : u16,
    offset : u32,
}
#[derive(Debug)]
pub struct Bmp {
    header: Header,
}