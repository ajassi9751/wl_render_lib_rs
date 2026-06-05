use crate::util::not_null::NotNull;
use crate::util::units::angle::Angle;

use super::argb::Argb;

use std::path::Path;

#[allow(unused)]
pub type Pixels = u32; // Maybe make this usize

// Represents an image as an array of pixels (rgbas)
// Can be rotated
#[derive(Debug)]
pub struct Image {
    data: Vec<Argb>, // Contains raw Rgbas
    ptr: NotNull<u8>,
    width: Pixels,
    height: Pixels,
}

impl Image {
    pub fn new(ptr: NotNull<u8>, width: Pixels, height: Pixels) -> Self {
        Self {
            data: Vec::new(),
            ptr: ptr,
            width: width,
            height: height,
        }
    }
    pub fn apply_request(request: &ImageRequest) {}
    fn get_coordinate_mut(&mut self, x: usize, y: usize) -> &mut Argb {
        &mut self.data[y * self.width as usize + x]
    }
    fn get_coordinate(&self, x: usize, y: usize) -> &Argb {
        &self.data[y * self.width as usize + x]
    }
}

pub trait ImageBackend {
    fn parse_rgb(path: &str, data: &mut Vec<Argb>) -> std::io::Result<()>;
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
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub enum ImageAction {
    Rotate(Angle),
    Translate(PixelCoordinate),
    Transparency(i32), // Will just add or subtract transparency
}
