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

pub struct Qoi {
    pix_array: [Argb; 64],
    prev_pix: Argb,
}

impl Qoi {
    pub fn new() -> Self {
        Self {
            pix_array: [Argb::default(); 64],
            prev_pix: Argb::default(),
        }
    }
    fn get_index(pixel: Argb) -> usize {
        pixel.r as usize * 3 + pixel.g as usize * 5 + pixel.b as usize * 7 + pixel.a as usize * 11
    }
}

impl ImageBackend for Qoi {
    fn parse_rgb(&mut self, path: &str, data: &mut Vec<Argb>) -> std::io::Result<()> {
        Ok(())
    }
}
