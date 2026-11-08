// Represents an angle
#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct Angle {
    data: f64,
}

impl Angle {
    pub fn from_degrees(degrees: f64) -> Self {
        Self { data: degrees * (std::f64::consts::PI / 180_f64) }
    }
    pub fn as_degrees(&self) -> f64 {
        self.data * (180_f64 / std::f64::consts::PI)
    }
    pub fn from_radians(radians: f64) -> Self {
        Self { data: radians }
    }
    pub fn as_radians(&self) -> f64 {
        self.data
    }
}
