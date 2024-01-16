#[derive(Debug)]
pub struct Header{
    format : u16,
    size : u32,
    special1 : u16,
    special2 : u16,
    offset : u32,
}
#[derive(Debug)]
pub struct DIB {
    dib_size: u32,
    width: u16,
    height: u16,
    planes: u16,
    bits_per_pixel: u16
}
#[derive(Debug)]
pub struct Bmp {
    header: Header,
    dib_header: DIB,

}