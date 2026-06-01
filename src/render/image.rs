use crate::render::config::Config;
use crate::util::units::angle::Angle;

use super::config::Pixels;
use super::rgba::Rgba;

// Represents an image as an array of pixels (rgbas)
// Can be rotated
#[derive(Debug)]
pub struct Image <'a> {
    data: Vec<Vec<Rgba>>,
    config: &'a Config // No need to own here
}

impl <'a> Image <'a> {
    pub fn new (config: &'a Config) -> Self {
        Self {
            // This is probably a really bad way to do this
            data: vec![vec![Rgba::default(); config.get_height() as usize]; config.get_width() as usize],
            config: config
        }
    }
    pub fn apply_request (request: &ImageRequest) {

    }
}

// Simmilar to a swerve request in ctre, represents an object that stores rotations and such that are requested for an image
#[derive(Debug)] // New does the same as default so it isn't derived
pub struct ImageRequest {
    actions: Vec<ImageAction>
}

impl ImageRequest {
    pub fn new () -> Self {
        Self {
            actions: Vec::new()
        }
    }
    pub fn from (actions: Vec<ImageAction>) -> Self {
        Self {
            actions: actions
        }
    }
    pub fn add_action (&mut self, action: ImageAction) -> &mut Self {
        self.actions.push(action);
        self
    }
    // I wouldve liked for this to consume itself but then there would allocations for no need sometimes
    pub fn get_actions (&self) -> &Vec<ImageAction> {
        &self.actions
    }
}

#[allow(unused)]
pub type PixelCoordinate = (Pixels, Pixels);  // Tuple of pixels, X and Y respectively

// Enum that represents an action that can be done to an image
#[non_exhaustive]
#[derive(Debug)]
pub enum ImageAction {
    Rotate(Angle),
    Translate(PixelCoordinate),
    Transparency(i32), // Will just add or subtract transparency
}