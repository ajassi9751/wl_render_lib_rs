// This file containst stuff to decode qoi files
use crate::render::argb::Argb;
use crate::render::image::ImageBackend;

use std::fs::File;
use std::io::{BufReader, Read};

// Stores options for the tags of the qoi file
enum Tag {
    RGB,
    RGBA,
    INDEX,
    DIFF,
    LUMA,
    RUN,
}

pub struct Qoi;

impl Qoi {
    fn get_index(pixel: &Argb) -> usize {
        (pixel.r as usize * 3 + pixel.g as usize * 5 + pixel.b as usize * 7 + pixel.a as usize * 11)
            % 64
    }
}

// Honestly should just be a function pointer
impl ImageBackend for Qoi {
    fn parse_rgb(path: &str, data: &mut Vec<Argb>) -> std::io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut pixel_array: [Argb; 64] = [Argb::new(0, 0, 0, 255); 64]; // Probably inefficient
        let mut byte_store: [u8; 14] = [0_u8; 14];
        // Will have to make some way to use this
        let mut _width: u32 = 0;
        let mut _height: u32 = 0;
        let mut isRgba: bool;
        let mut _colorspace: u8;
        let mut tag: Tag;
        let mut prev_pixel: Argb = Argb::default();
        let mut current_pixel: Argb = Argb::default();
        let mut store_bytes: (u8, u8) = (0, 0); // (Amount left, total)
        let mut tag_byte: u8 = 0_u8;
        for (i, byte_result) in reader.bytes().into_iter().enumerate() {
            let byte = byte_result?;
            if i < 14 {
                byte_store[i] = byte;
                continue;
            }
            if i == 14 {
                let validator = match std::str::from_utf8(byte_store[0..3].into()) {
                    Ok(s) => s,
                    Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
                };
                if validator != "qoif" {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Not a qoif file",
                    ));
                }
                for (i, num) in byte_store[4..7].into_iter().enumerate() {
                    _width |= (*num as u32) << (8 * (3 - i)); // Should make this a function and make a test for it
                }
                for (i, num) in byte_store[8..11].into_iter().enumerate() {
                    _height |= (*num as u32) << (8 * (3 - i));
                }
                isRgba = byte_store[12] == 4; // No clue about this warning
                _colorspace = byte_store[13]; // Why is there a warning!!!
            }
            if store_bytes.0 == 0 {
                let bits: u8 = byte >> 6; // Gives us the first two bits of the byte
                match bits {
                    0xFF => {
                        tag = Tag::RGBA;
                        store_bytes = (4, 4);
                        tag_byte = byte;
                    }
                    0xFE => {
                        tag = Tag::RGB;
                        store_bytes = (3, 3);
                        tag_byte = byte;
                    }
                    0x01..=0x3F => {
                        tag = Tag::INDEX;
                        store_bytes = (0, 0);
                        current_pixel = pixel_array[(15 & byte) as usize]
                    } // This range may be wrong, also the & removes the first two bits
                    0x40 => {
                        tag = Tag::DIFF; // Relies on wrap around arithmetic so it may not work in debug
                        let rval = prev_pixel.r + (bits >> 4); // No need to & her because shifting by 4 removes it
                        let gval = prev_pixel.g + (3 & (bits >> 2)); // 3 is 000011 so it with & it only preserves th first 3 bits
                        let bval = prev_pixel.b + (3 & bits);
                        current_pixel = Argb::new(prev_pixel.a, rval, gval, bval);
                    }
                    0x80 => {
                        tag = Tag::LUMA;
                        store_bytes = (1, 1);
                        tag_byte = byte;
                    }
                    0x00 => {
                        tag = Tag::RUN;
                        let times = (15 & byte) + 1;
                        current_pixel = prev_pixel;
                        for _ in 0..times {
                            data.push(prev_pixel.clone());
                        }
                    }
                    _ => {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "Invalid Tag bit",
                        ))
                    }
                }
                byte_store = [0_u8; 14]; // Reset the array
            } else {
                byte_store[(store_bytes.1 as usize) - (store_bytes.0 as usize)] = byte;
                continue;
            }
            // Implement the rest of the operations
            // Definitley needs a condition
            pixel_array[Self::get_index(&current_pixel)] = current_pixel;
            prev_pixel = current_pixel;
            data.push(current_pixel) // Seems wrong
        }
        Ok(())
    }
}
