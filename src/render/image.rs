use crate::util::not_null::NotNull;
use crate::util::units::angle::Angle;

use super::argb::Argb;

use std::convert::TryInto;

#[allow(unused)]
pub type Pixels = u32; // Maybe make this usize

// Represents an image as an array of pixels (rgbas)
// Can be rotated
#[derive(Debug)]
pub struct Image {
    data: Vec<Argb>, // Think of this Vec as a coordinate plane only in the first quadrant, the image is encoded from top left to bottom right but coordinates are accesed as if in a regular first quadrant coordinate plane
    ptr: NotNull<u32>, // Pointer that will be written to, maybe should be a slice to be more explicit about size?
    width: Pixels,
    height: Pixels,
}

impl Image {
    pub fn new(ptr: NotNull<u32>, width: Pixels, height: Pixels) -> Self {
        Self {
            data: Vec::new(),
            ptr: ptr,
            width: width,
            height: height,
        }
    }
    // Might make this a standalone function and remove ptr from Image
    pub fn write_to_buffer(&mut self) {
        for i in 0..((self.width / 4) * self.height) {
            unsafe {
                *self
                    .ptr
                    .get_mut()
                    .offset(i.try_into().expect(
                        "Pointer offset failed due to the value not fitting into an isize",
                    )) = self.data[i as usize].as_precalculated_alpha();
            }
        }
    }
    // Data shouldn't be mutated so this is for testing
    #[cfg(test)]
    pub fn get_data_mut(&mut self) -> &mut Vec<Argb> {
        &mut self.data
    }
    // Data can be borrowed to be written to a buffer or stored by the user because I haven't yet found a safe way to make an api for buffers that doesn't just copy all the info (which is thread unsafe becuase of pointers, it would be great if I could use self to consume the object for the api or use Arc or Rc)
    pub fn get_data(&self) -> &Vec<Argb> {
        &self.data
    }
    pub fn decode_image<T: ImageBackend>(&mut self, path: &str) -> std::io::Result<()> {
        self.data = T::parse_rgb(path)?;
        Ok(())
    }
    pub fn apply_request(request: &ImageRequest) {}
    fn get_coordinate_mut(&mut self, x: usize, y: usize) -> &mut Argb {
        let len = self.data.len();
        &mut self.data[(len - (y * self.width as usize)) + x]
    }
    fn get_coordinate(&self, x: usize, y: usize) -> &Argb {
        let len = self.data.len();
        &self.data[(len - (y * self.width as usize)) + x]
    }
}

pub trait ImageBackend {
    fn parse_rgb(path: &str) -> std::io::Result<Vec<Argb>>; // I would rather not use &str to represent a path but std::path::Path doesn't work well
}

#[test]
fn buffer_write_test() {
    let mut buffer: [u32; (20 / 4) * 2] = [0_u32; (20 / 4) * 2];
    let ptr = NotNull::try_from(buffer.as_mut_ptr()).unwrap();
    let mut image = Image::new(ptr, 20, 2);
    image.get_data_mut().resize(10, Argb::new(1, 2, 3, 4));
    image.write_to_buffer();
    println!("{:?}", buffer);
}

// Simmilar to a swerve request in ctre, represents an object that stores rotations and such that are requested for an image
#[derive(Debug, PartialEq, Eq)] // New does the same as default so it isn't derived
pub struct ImageRequest {
    actions: Vec<ImageAction>,
}

impl ImageRequest {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }
    pub fn from(actions: Vec<ImageAction>) -> Self {
        Self { actions: actions }
    }
    pub fn add_action(&mut self, action: ImageAction) -> &mut Self {
        self.actions.push(action);
        self
    }
    // I wouldve liked for this to consume itself but then there would allocations for no need sometimes
    pub fn get_actions(&self) -> &Vec<ImageAction> {
        &self.actions
    }
}

#[allow(unused)]
pub type PixelCoordinate = (Pixels, Pixels); // Tuple of pixels, X and Y respectively

// Enum that represents an action that can be done to an image
#[non_exhaustive] // More can be added in the future
#[derive(Debug, PartialEq, Eq)]
pub enum ImageAction {
    Rotate(Angle),
    Translate(PixelCoordinate),
    Transparency(i32), // Will just add or subtract transparency
    Resize(u32),
}
