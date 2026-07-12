use crate::util::not_null::NotNull;
use crate::util::units::angle::Angle;

use super::argb::Argb;

use std::convert::TryInto;

#[allow(unused)]
pub type Pixels = u32; // Maybe make this usize

// Represents an image as an array of pixels (rgbas)
// Can be mutated with ImageRequests
#[derive(Debug)]
pub struct Image {
    data: Vec<Argb>, // Think of this Vec as a coordinate plane only in the first quadrant, the image is encoded from top left to bottom right but coordinates are accesed as if in a regular first quadrant coordinate plane
    ptr: NotNull<u32>, // Pointer that will be written to, maybe should be a slice to be more explicit about size?
    width: Option<Pixels>,
    height: Option<Pixels>,
}

#[allow(unused)]
impl Image {
    pub fn new(ptr: NotNull<u32>) -> Self {
        Self {
            data: Vec::new(),
            ptr: ptr,
            width: None,
            height: None,
        }
    }
    // Might make this a standalone function and remove ptr from Image
    #[must_use]
    pub fn write_to_buffer(&mut self) -> Option<()> {
        for i in 0..((self.width?) * self.height?) {
            unsafe {
                *self.ptr.get_mut().offset(i.try_into().expect(
                    "Pointer offset failed due to the value not fitting into an isize", // Probably a good idea to panic from here because if you try to recover, you have corruption
                )) = self.data[i as usize].as_precalculated_alpha();
            }
        }
        Some(())
    }
    // Data shouldn't be mutated so this is for testing
    #[cfg(test)]
    pub fn get_data_mut(&mut self) -> &mut Vec<Argb> {
        &mut self.data
    }
    // Dimensions shouldn't be altered from outside so this is only for tests
    #[cfg(test)]
    pub fn set_dimensions(&mut self, width: Pixels, height: Pixels) {
        self.width = Some(width);
        self.height = Some(height);
    }
    pub fn decode_image<T: ImageBackend>(&mut self, path: &str) -> std::io::Result<()> {
        let (data, width, height) = T::parse_rgb(path)?;
        self.data = data;
        self.width = Some(width);
        self.height = Some(height);
        Ok(())
    }
    pub fn apply_request(request: &ImageRequest) {
        todo!()
    }
    pub fn get_height_width(&self) -> Option<(Pixels, Pixels)> { // This is not a coordinate so it is not a PixelCoordinate
        Some((self.width?, self.height?))
    }
    fn get_coordinate_mut(&mut self, x: usize, y: usize) -> Option<&mut Argb> {
        let len = self.data.len();
        Some(&mut self.data[(len - (y * self.width? as usize)) + x])
    }
    fn get_coordinate(&self, x: usize, y: usize) -> Option<&Argb> {
        let len = self.data.len();
        Some(&self.data[(len - (y * self.width? as usize)) + x])
    }
    pub fn change_ptr(&mut self, ptr: NotNull<u32>) {
        self.ptr = ptr;
    }
    pub fn into_buffer(mut self) -> Option<ImageBuffer> {
        let mut data: Vec<Argb> = self.data.into_iter().map(|mut i| { i.precalculate_alpha_mut(); i}).collect();
        data.shrink_to_fit(); // Maybe not needed and might hurt performance
        Some(ImageBuffer {
            data: data,
            ptr: self.ptr,
            size: self.width? * self.height?
        })
    }
}

// A struct that stores and image in an immutable state with precalculated alpha
// It is used to store an image in memory and is destroyed upon writing
// Useful for prerendering buffers before submiting it to the compositor
// This model does have some issues with data races when writing to pointers
pub struct ImageBuffer {
    data: Vec<Argb>,
    ptr: NotNull<u32>,
    size: Pixels,
}

impl ImageBuffer {
    pub fn change_ptr(&mut self, ptr: NotNull<u32>) {
        self.ptr = ptr;
    }
    pub fn write_to_buffer(&mut self) {
        // This copy is faster and uses SIMD
        unsafe {
            std::ptr::copy_nonoverlapping(self.data.as_ptr() as *const u32, self.ptr.get_mut(), self.size.try_into().expect("Memcopy failed due to size value not fitting into usize"));
        }
    }
}

pub trait ImageBackend {
    fn parse_rgb(path: &str) -> std::io::Result<(Vec<Argb>, Pixels, Pixels)>; // I would rather not use &str to represent a path but std::path::Path doesn't work well
}

#[test]
fn buffer_write_test() {
    let mut buffer: [u32; 20 * 2] = [0_u32; 20 * 2];
    let ptr = NotNull::try_from(buffer.as_mut_ptr()).unwrap();
    let mut image = Image::new(ptr);
    image.set_dimensions(20, 2);
    image.get_data_mut().resize(40, Argb::new(1, 2, 3, 4));
    image.write_to_buffer().expect("Write failed");
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
