//The following code is adapted from the "png" crate - https://github.com/image-rs/image-png/, https://crates.io/crates/png
// And is distributed with MIT license:

// Copyright (c) 2015 nwin

// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:

// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.


use core::convert::TryInto;

pub enum BytesPerPixel {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Six = 6,
    Eight = 8,
}

impl BytesPerPixel {
    pub fn from_u8(value: u8) -> Option<BytesPerPixel> {
        match value {
            1 => Some(BytesPerPixel::One),
            2 => Some(BytesPerPixel::Two),
            3 => Some(BytesPerPixel::Three),
            4 => Some(BytesPerPixel::Four),
            6 => Some(BytesPerPixel::Six),
            8 => Some(BytesPerPixel::Eight),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FilterType {
    NoFilter = 0,
    Sub = 1,
    Up = 2,
    Avg = 3,
    Paeth = 4,
}
impl Default for FilterType {
    fn default() -> Self {
        FilterType::Sub
    }
}
impl FilterType {
    
    pub fn from_u8(n: u8) -> Option<FilterType> {
        match n {
            0 => Some(FilterType::NoFilter),
            1 => Some(FilterType::Sub),
            2 => Some(FilterType::Up),
            3 => Some(FilterType::Avg),
            4 => Some(FilterType::Paeth),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AdaptiveFilterType {
    // Adaptive,
    NonAdaptive,
}
impl Default for AdaptiveFilterType {
    fn default() -> Self {
        AdaptiveFilterType::NonAdaptive
    }
}
fn filter_paeth_decode(a: u8, b: u8, c: u8) -> u8 {
    
    let pa = (i16::from(b) - i16::from(c)).abs();
    let pb = (i16::from(a) - i16::from(c)).abs();
    let pc = ((i16::from(a) - i16::from(c)) + (i16::from(b) - i16::from(c))).abs();
    let mut out = a;
    let mut min = pa;
    if pb < min {
        min = pb;
        out = b;
    }
    if pc < min {
        out = c;
    }
    out
}

pub fn unfilter(
    mut filter: FilterType,
    tbpp: &BytesPerPixel,
    previous: &[u8],
    current: &mut [u8],
) {
    use self::FilterType::*;
    
    if previous.is_empty() {
        if filter == Paeth {
            filter = Sub;
        } else if filter == Up {
            filter = NoFilter;
        }
    }
    
    match filter {
        NoFilter => {}
        Sub => match tbpp {
            BytesPerPixel::One => {
                current.iter_mut().reduce(|&mut prev, curr| {
                    *curr = curr.wrapping_add(prev);
                    curr
                });
            }
            BytesPerPixel::Two => {
                let mut prev = [0; 2];
                for chunk in current.chunks_exact_mut(2) {
                    let new_chunk = [
                        chunk[0].wrapping_add(prev[0]),
                        chunk[1].wrapping_add(prev[1]),
                    ];
                    *TryInto::<&mut [u8; 2]>::try_into(chunk).unwrap() = new_chunk;
                    prev = new_chunk;
                }
            }
            BytesPerPixel::Three => {
                let mut prev = [0; 3];
                for chunk in current.chunks_exact_mut(3) {
                    let new_chunk = [
                        chunk[0].wrapping_add(prev[0]),
                        chunk[1].wrapping_add(prev[1]),
                        chunk[2].wrapping_add(prev[2]),
                    ];
                    *TryInto::<&mut [u8; 3]>::try_into(chunk).unwrap() = new_chunk;
                    prev = new_chunk;
                }
            }
            BytesPerPixel::Four => {
                let mut prev = [0; 4];
                for chunk in current.chunks_exact_mut(4) {
                    let new_chunk = [
                        chunk[0].wrapping_add(prev[0]),
                        chunk[1].wrapping_add(prev[1]),
                        chunk[2].wrapping_add(prev[2]),
                        chunk[3].wrapping_add(prev[3]),
                    ];
                    *TryInto::<&mut [u8; 4]>::try_into(chunk).unwrap() = new_chunk;
                    prev = new_chunk;
                }
            }
            BytesPerPixel::Six => {
                let mut prev = [0; 6];
                for chunk in current.chunks_exact_mut(6) {
                    let new_chunk = [
                        chunk[0].wrapping_add(prev[0]),
                        chunk[1].wrapping_add(prev[1]),
                        chunk[2].wrapping_add(prev[2]),
                        chunk[3].wrapping_add(prev[3]),
                        chunk[4].wrapping_add(prev[4]),
                        chunk[5].wrapping_add(prev[5]),
                    ];
                    *TryInto::<&mut [u8; 6]>::try_into(chunk).unwrap() = new_chunk;
                    prev = new_chunk;
                }
            }
            BytesPerPixel::Eight => {
                let mut prev = [0; 8];
                for chunk in current.chunks_exact_mut(8) {
                    let new_chunk = [
                        chunk[0].wrapping_add(prev[0]),
                        chunk[1].wrapping_add(prev[1]),
                        chunk[2].wrapping_add(prev[2]),
                        chunk[3].wrapping_add(prev[3]),
                        chunk[4].wrapping_add(prev[4]),
                        chunk[5].wrapping_add(prev[5]),
                        chunk[6].wrapping_add(prev[6]),
                        chunk[7].wrapping_add(prev[7]),
                    ];
                    *TryInto::<&mut [u8; 8]>::try_into(chunk).unwrap() = new_chunk;
                    prev = new_chunk;
                }
            }
        },
        Up => {
            for (curr, &above) in current.iter_mut().zip(previous) {
                *curr = curr.wrapping_add(above);
            }
        }
        Avg if previous.is_empty() => match tbpp {
            BytesPerPixel::One => {
                current.iter_mut().reduce(|&mut prev, curr| {
                    *curr = curr.wrapping_add(prev / 2);
                    curr
                });
            }
            BytesPerPixel::Two => {
                let mut prev = [0; 2];
                for chunk in current.chunks_exact_mut(2) {
                    let new_chunk = [
                        chunk[0].wrapping_add(prev[0] / 2),
                        chunk[1].wrapping_add(prev[1] / 2),
                    ];
                    *TryInto::<&mut [u8; 2]>::try_into(chunk).unwrap() = new_chunk;
                    prev = new_chunk;
                }
            }
            BytesPerPixel::Three => {
                let mut prev = [0; 3];
                for chunk in current.chunks_exact_mut(3) {
                    let new_chunk = [
                        chunk[0].wrapping_add(prev[0] / 2),
                        chunk[1].wrapping_add(prev[1] / 2),
                        chunk[2].wrapping_add(prev[2] / 2),
                    ];
                    *TryInto::<&mut [u8; 3]>::try_into(chunk).unwrap() = new_chunk;
                    prev = new_chunk;
                }
            }
            BytesPerPixel::Four => {
                let mut prev = [0; 4];
                for chunk in current.chunks_exact_mut(4) {
                    let new_chunk = [
                        chunk[0].wrapping_add(prev[0] / 2),
                        chunk[1].wrapping_add(prev[1] / 2),
                        chunk[2].wrapping_add(prev[2] / 2),
                        chunk[3].wrapping_add(prev[3] / 2),
                    ];
                    *TryInto::<&mut [u8; 4]>::try_into(chunk).unwrap() = new_chunk;
                    prev = new_chunk;
                }
            }
            BytesPerPixel::Six => {
                let mut prev = [0; 6];
                for chunk in current.chunks_exact_mut(6) {
                    let new_chunk = [
                        chunk[0].wrapping_add(prev[0] / 2),
                        chunk[1].wrapping_add(prev[1] / 2),
                        chunk[2].wrapping_add(prev[2] / 2),
                        chunk[3].wrapping_add(prev[3] / 2),
                        chunk[4].wrapping_add(prev[4] / 2),
                        chunk[5].wrapping_add(prev[5] / 2),
                    ];
                    *TryInto::<&mut [u8; 6]>::try_into(chunk).unwrap() = new_chunk;
                    prev = new_chunk;
                }
            }
            BytesPerPixel::Eight => {
                let mut prev = [0; 8];
                for chunk in current.chunks_exact_mut(8) {
                    let new_chunk = [
                        chunk[0].wrapping_add(prev[0] / 2),
                        chunk[1].wrapping_add(prev[1] / 2),
                        chunk[2].wrapping_add(prev[2] / 2),
                        chunk[3].wrapping_add(prev[3] / 2),
                        chunk[4].wrapping_add(prev[4] / 2),
                        chunk[5].wrapping_add(prev[5] / 2),
                        chunk[6].wrapping_add(prev[6] / 2),
                        chunk[7].wrapping_add(prev[7] / 2),
                    ];
                    *TryInto::<&mut [u8; 8]>::try_into(chunk).unwrap() = new_chunk;
                    prev = new_chunk;
                }
            }
        },
        Avg => match tbpp {
            BytesPerPixel::One => {
                let mut lprev = [0; 1];
                for (chunk, above) in current.chunks_exact_mut(1).zip(previous.chunks_exact(1)) {
                    let new_chunk =
                        [chunk[0].wrapping_add(((above[0] as u16 + lprev[0] as u16) / 2) as u8)];
                    *TryInto::<&mut [u8; 1]>::try_into(chunk).unwrap() = new_chunk;
                    lprev = new_chunk;
                }
            }
            BytesPerPixel::Two => {
                let mut lprev = [0; 2];
                for (chunk, above) in current.chunks_exact_mut(2).zip(previous.chunks_exact(2)) {
                    let new_chunk = [
                        chunk[0].wrapping_add(((above[0] as u16 + lprev[0] as u16) / 2) as u8),
                        chunk[1].wrapping_add(((above[1] as u16 + lprev[1] as u16) / 2) as u8),
                    ];
                    *TryInto::<&mut [u8; 2]>::try_into(chunk).unwrap() = new_chunk;
                    lprev = new_chunk;
                }
            }
            BytesPerPixel::Three => {
                let mut lprev = [0; 3];
                for (chunk, above) in current.chunks_exact_mut(3).zip(previous.chunks_exact(3)) {
                    let new_chunk = [
                        chunk[0].wrapping_add(((above[0] as u16 + lprev[0] as u16) / 2) as u8),
                        chunk[1].wrapping_add(((above[1] as u16 + lprev[1] as u16) / 2) as u8),
                        chunk[2].wrapping_add(((above[2] as u16 + lprev[2] as u16) / 2) as u8),
                    ];
                    *TryInto::<&mut [u8; 3]>::try_into(chunk).unwrap() = new_chunk;
                    lprev = new_chunk;
                }
            }
            BytesPerPixel::Four => {
                let mut lprev = [0; 4];
                for (chunk, above) in current.chunks_exact_mut(4).zip(previous.chunks_exact(4)) {
                    let new_chunk = [
                        chunk[0].wrapping_add(((above[0] as u16 + lprev[0] as u16) / 2) as u8),
                        chunk[1].wrapping_add(((above[1] as u16 + lprev[1] as u16) / 2) as u8),
                        chunk[2].wrapping_add(((above[2] as u16 + lprev[2] as u16) / 2) as u8),
                        chunk[3].wrapping_add(((above[3] as u16 + lprev[3] as u16) / 2) as u8),
                    ];
                    *TryInto::<&mut [u8; 4]>::try_into(chunk).unwrap() = new_chunk;
                    lprev = new_chunk;
                }
            }
            BytesPerPixel::Six => {
                let mut lprev = [0; 6];
                for (chunk, above) in current.chunks_exact_mut(6).zip(previous.chunks_exact(6)) {
                    let new_chunk = [
                        chunk[0].wrapping_add(((above[0] as u16 + lprev[0] as u16) / 2) as u8),
                        chunk[1].wrapping_add(((above[1] as u16 + lprev[1] as u16) / 2) as u8),
                        chunk[2].wrapping_add(((above[2] as u16 + lprev[2] as u16) / 2) as u8),
                        chunk[3].wrapping_add(((above[3] as u16 + lprev[3] as u16) / 2) as u8),
                        chunk[4].wrapping_add(((above[4] as u16 + lprev[4] as u16) / 2) as u8),
                        chunk[5].wrapping_add(((above[5] as u16 + lprev[5] as u16) / 2) as u8),
                    ];
                    *TryInto::<&mut [u8; 6]>::try_into(chunk).unwrap() = new_chunk;
                    lprev = new_chunk;
                }
            }
            BytesPerPixel::Eight => {
                let mut lprev = [0; 8];
                for (chunk, above) in current.chunks_exact_mut(8).zip(previous.chunks_exact(8)) {
                    let new_chunk = [
                        chunk[0].wrapping_add(((above[0] as u16 + lprev[0] as u16) / 2) as u8),
                        chunk[1].wrapping_add(((above[1] as u16 + lprev[1] as u16) / 2) as u8),
                        chunk[2].wrapping_add(((above[2] as u16 + lprev[2] as u16) / 2) as u8),
                        chunk[3].wrapping_add(((above[3] as u16 + lprev[3] as u16) / 2) as u8),
                        chunk[4].wrapping_add(((above[4] as u16 + lprev[4] as u16) / 2) as u8),
                        chunk[5].wrapping_add(((above[5] as u16 + lprev[5] as u16) / 2) as u8),
                        chunk[6].wrapping_add(((above[6] as u16 + lprev[6] as u16) / 2) as u8),
                        chunk[7].wrapping_add(((above[7] as u16 + lprev[7] as u16) / 2) as u8),
                    ];
                    *TryInto::<&mut [u8; 8]>::try_into(chunk).unwrap() = new_chunk;
                    lprev = new_chunk;
                }
            }
        },
        Paeth => {
            
            match tbpp {
                BytesPerPixel::One => {
                    let mut a_bpp = [0; 1];
                    let mut c_bpp = [0; 1];
                    for (chunk, b_bpp) in current.chunks_exact_mut(1).zip(previous.chunks_exact(1))
                    {
                        let new_chunk = [chunk[0]
                            .wrapping_add(filter_paeth_decode(a_bpp[0], b_bpp[0], c_bpp[0]))];
                        *TryInto::<&mut [u8; 1]>::try_into(chunk).unwrap() = new_chunk;
                        a_bpp = new_chunk;
                        c_bpp = b_bpp.try_into().unwrap();
                    }
                }
                BytesPerPixel::Two => {
                    let mut a_bpp = [0; 2];
                    let mut c_bpp = [0; 2];
                    for (chunk, b_bpp) in current.chunks_exact_mut(2).zip(previous.chunks_exact(2))
                    {
                        let new_chunk = [
                            chunk[0]
                                .wrapping_add(filter_paeth_decode(a_bpp[0], b_bpp[0], c_bpp[0])),
                            chunk[1]
                                .wrapping_add(filter_paeth_decode(a_bpp[1], b_bpp[1], c_bpp[1])),
                        ];
                        *TryInto::<&mut [u8; 2]>::try_into(chunk).unwrap() = new_chunk;
                        a_bpp = new_chunk;
                        c_bpp = b_bpp.try_into().unwrap();
                    }
                }
                BytesPerPixel::Three => {
                    {
                        let mut a_bpp = [0; 3];
                        let mut c_bpp = [0; 3];
                        for (chunk, b_bpp) in
                            current.chunks_exact_mut(3).zip(previous.chunks_exact(3))
                        {
                            let new_chunk = [
                                chunk[0].wrapping_add(filter_paeth_decode(
                                    a_bpp[0], b_bpp[0], c_bpp[0],
                                )),
                                chunk[1].wrapping_add(filter_paeth_decode(
                                    a_bpp[1], b_bpp[1], c_bpp[1],
                                )),
                                chunk[2].wrapping_add(filter_paeth_decode(
                                    a_bpp[2], b_bpp[2], c_bpp[2],
                                )),
                            ];
                            *TryInto::<&mut [u8; 3]>::try_into(chunk).unwrap() = new_chunk;
                            a_bpp = new_chunk;
                            c_bpp = b_bpp.try_into().unwrap();
                        }
                    }
                }
                BytesPerPixel::Four => {
                    let mut a_bpp = [0; 4];
                    let mut c_bpp = [0; 4];
                    for (chunk, b_bpp) in current.chunks_exact_mut(4).zip(previous.chunks_exact(4))
                    {
                        let new_chunk = [
                            chunk[0]
                                .wrapping_add(filter_paeth_decode(a_bpp[0], b_bpp[0], c_bpp[0])),
                            chunk[1]
                                .wrapping_add(filter_paeth_decode(a_bpp[1], b_bpp[1], c_bpp[1])),
                            chunk[2]
                                .wrapping_add(filter_paeth_decode(a_bpp[2], b_bpp[2], c_bpp[2])),
                            chunk[3]
                                .wrapping_add(filter_paeth_decode(a_bpp[3], b_bpp[3], c_bpp[3])),
                        ];
                        *TryInto::<&mut [u8; 4]>::try_into(chunk).unwrap() = new_chunk;
                        a_bpp = new_chunk;
                        c_bpp = b_bpp.try_into().unwrap();
                    }
                }
                BytesPerPixel::Six => {
                    {
                        let mut a_bpp = [0; 6];
                        let mut c_bpp = [0; 6];
                        for (chunk, b_bpp) in
                            current.chunks_exact_mut(6).zip(previous.chunks_exact(6))
                        {
                            let new_chunk = [
                                chunk[0].wrapping_add(filter_paeth_decode(
                                    a_bpp[0], b_bpp[0], c_bpp[0],
                                )),
                                chunk[1].wrapping_add(filter_paeth_decode(
                                    a_bpp[1], b_bpp[1], c_bpp[1],
                                )),
                                chunk[2].wrapping_add(filter_paeth_decode(
                                    a_bpp[2], b_bpp[2], c_bpp[2],
                                )),
                                chunk[3].wrapping_add(filter_paeth_decode(
                                    a_bpp[3], b_bpp[3], c_bpp[3],
                                )),
                                chunk[4].wrapping_add(filter_paeth_decode(
                                    a_bpp[4], b_bpp[4], c_bpp[4],
                                )),
                                chunk[5].wrapping_add(filter_paeth_decode(
                                    a_bpp[5], b_bpp[5], c_bpp[5],
                                )),
                            ];
                            *TryInto::<&mut [u8; 6]>::try_into(chunk).unwrap() = new_chunk;
                            a_bpp = new_chunk;
                            c_bpp = b_bpp.try_into().unwrap();
                        }
                    }
                }
                BytesPerPixel::Eight => {
                    let mut a_bpp = [0; 8];
                    let mut c_bpp = [0; 8];
                    for (chunk, b_bpp) in current.chunks_exact_mut(8).zip(previous.chunks_exact(8))
                    {
                        let new_chunk = [
                            chunk[0]
                                .wrapping_add(filter_paeth_decode(a_bpp[0], b_bpp[0], c_bpp[0])),
                            chunk[1]
                                .wrapping_add(filter_paeth_decode(a_bpp[1], b_bpp[1], c_bpp[1])),
                            chunk[2]
                                .wrapping_add(filter_paeth_decode(a_bpp[2], b_bpp[2], c_bpp[2])),
                            chunk[3]
                                .wrapping_add(filter_paeth_decode(a_bpp[3], b_bpp[3], c_bpp[3])),
                            chunk[4]
                                .wrapping_add(filter_paeth_decode(a_bpp[4], b_bpp[4], c_bpp[4])),
                            chunk[5]
                                .wrapping_add(filter_paeth_decode(a_bpp[5], b_bpp[5], c_bpp[5])),
                            chunk[6]
                                .wrapping_add(filter_paeth_decode(a_bpp[6], b_bpp[6], c_bpp[6])),
                            chunk[7]
                                .wrapping_add(filter_paeth_decode(a_bpp[7], b_bpp[7], c_bpp[7])),
                        ];
                        *TryInto::<&mut [u8; 8]>::try_into(chunk).unwrap() = new_chunk;
                        a_bpp = new_chunk;
                        c_bpp = b_bpp.try_into().unwrap();
                    }
                }
            }
        }
    }
}
