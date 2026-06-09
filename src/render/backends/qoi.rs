// This file containst stuff to decode qoi files
use crate::render::argb::Argb;
use crate::render::image::ImageBackend;

// Tag enum removed, parsing is now implemented directly in parse_rgb :(

pub struct Qoi;

impl Qoi {
    fn get_index(pixel: &Argb) -> usize {
        (pixel.r as usize * 3 + pixel.g as usize * 5 + pixel.b as usize * 7 + pixel.a as usize * 11)
            % 64
    }
}

// Honestly should just be a function pointer
impl ImageBackend for Qoi {
    fn parse_rgb(path: &str) -> std::io::Result<Vec<Argb>> { // Should probably just return the vec instead of a reference
        let mut data: Vec<Argb> = Vec::new();
        // Read entire file into memory for simpler parsing
        let bytes = std::fs::read(path)?;
        if bytes.len() < 14 + 8 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "File too small to be a qoi image",
            ));
        }

        // Header
        if &bytes[0..4] != b"qoif" { // b"" means that it is a byte string
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Not a qoi file",
            ));
        }
        // Find a way to use these, possibly with reserving the vec dimensions
        let _width = ((bytes[4] as u32) << 24)
            | ((bytes[5] as u32) << 16)
            | ((bytes[6] as u32) << 8)
            | (bytes[7] as u32);
        let _height = ((bytes[8] as u32) << 24)
            | ((bytes[9] as u32) << 16)
            | ((bytes[10] as u32) << 8)
            | (bytes[11] as u32);
        let _channels = bytes[12]; // 3 = RGB, 4 = RGBA, we ignore this
        let _colorspace = bytes[13]; // Ngl I have no clue what this is

        let mut pixel_array: [Argb; 64] = [Argb::default(); 64];
        // QOI spec starts with previous pixel = (r=0,g=0,b=0,a=255)
        let mut prev_pixel = Argb::new(255, 0, 0, 0);

        let mut pos: usize = 14;
        // Stop before the 8-byte end marker
        let end_marker_pos = bytes.len().saturating_sub(8); // Overflow instead of wrapping arithmetic

        while pos < end_marker_pos {
            let b = bytes[pos];
            pos += 1;

            if b == 0xFE {
                // OP_RGB: next 3 bytes r,g,b (alpha unchanged)
                if pos + 3 > end_marker_pos + 8 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::UnexpectedEof,
                        "Unexpected EOF in OP_RGB",
                    ));
                }
                let r = bytes[pos];
                let g = bytes[pos + 1];
                let bl = bytes[pos + 2];
                pos += 3;
                let current = Argb::new(prev_pixel.a, r, g, bl);
                pixel_array[Self::get_index(&current)] = current;
                data.push(current);
                prev_pixel = current;
                continue;
            }
            if b == 0xFF {
                // OP_RGBA: next 4 bytes r,g,b,a
                if pos + 4 > end_marker_pos + 8 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::UnexpectedEof,
                        "Unexpected EOF in OP_RGBA",
                    ));
                }
                let r = bytes[pos];
                let g = bytes[pos + 1];
                let bl = bytes[pos + 2];
                let a = bytes[pos + 3];
                pos += 4;
                let current = Argb::new(a, r, g, bl);
                pixel_array[Self::get_index(&current)] = current;
                data.push(current);
                prev_pixel = current;
                continue;
            }

            match b >> 6 {
                0b00 => {
                    // QOI_OP_INDEX: lower 6 bits
                    let idx = (b & 0x3F) as usize;
                    let current = pixel_array[idx];
                    data.push(current);
                    prev_pixel = current;
                }
                0b01 => {
                    // QOI_OP_DIFF: 2-bit diffs with bias 2
                    let dr = ((b >> 4) & 0x03) as i16 - 2;
                    let dg = ((b >> 2) & 0x03) as i16 - 2;
                    let db = (b & 0x03) as i16 - 2;
                    let r = (prev_pixel.r as i16 + dr) as u8;
                    let g = (prev_pixel.g as i16 + dg) as u8;
                    let bl = (prev_pixel.b as i16 + db) as u8;
                    let current = Argb::new(prev_pixel.a, r, g, bl);
                    pixel_array[Self::get_index(&current)] = current;
                    data.push(current);
                    prev_pixel = current;
                }
                0b10 => {
                    // QOI_OP_LUMA: 6-bit dg (bias 32) then one byte with dr_dg and db_dg (4 bits each, bias 8)
                    if pos >= end_marker_pos + 8 {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::UnexpectedEof,
                            "Unexpected EOF in OP_LUMA",
                        ));
                    }
                    let b2 = bytes[pos];
                    pos += 1;
                    let dg = (b & 0x3F) as i16 - 32;
                    let dr_dg = ((b2 >> 4) & 0x0F) as i16 - 8;
                    let db_dg = (b2 & 0x0F) as i16 - 8;
                    let dr = dr_dg + dg;
                    let db = db_dg + dg;
                    let r = (prev_pixel.r as i16 + dr) as u8;
                    let g = (prev_pixel.g as i16 + dg) as u8;
                    let bl = (prev_pixel.b as i16 + db) as u8;
                    let current = Argb::new(prev_pixel.a, r, g, bl);
                    pixel_array[Self::get_index(&current)] = current;
                    data.push(current);
                    prev_pixel = current;
                }
                0b11 => {
                    // QOI_OP_RUN: lower 6 bits + 1
                    let run = (b & 0x3F) as usize + 1;
                    for _ in 0..run {
                        data.push(prev_pixel);
                    }
                }
                _ => unreachable!(), // Shouldn't be possible to reach
            }
        }
        Ok(data)
    }
}

#[test]
fn img_test() {
    let mut rgb: Vec<Argb> = Qoi::parse_rgb("qoi_test_images/qoi_logo.qoi").unwrap(); // Relies on the filesystem, bad test, I know
    rgb.shrink_to_fit(); // Vec doesn't need to grow anymore
}
