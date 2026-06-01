use crate::util::not_null::NotNull;

// It is literally used, why does it say it isn't
#[allow(unused)]
pub type Pixels = u32;

#[derive(Debug)]
pub struct Config {
    ptr: NotNull<u8>, // Might have to be stored in the image struct
    width: Pixels,
    height: Pixels,
}

impl Config {
    pub fn new (ptr: NotNull<u8>, width: Pixels, height: Pixels) -> Self {
        Self {
            ptr: ptr,
            width: width,
            height: height
        }
    }
    pub fn get_width (&self) -> Pixels {
        self.width
    }
    pub fn get_height (&self) -> Pixels {
        self.height
    }
}