// Represents an angle
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Angle {
    data: i64
}

impl Angle {
    pub fn from_degrees (degrees: i64) -> Self {
        Self {
            data: degrees
        }
    }
    pub fn as_degrees (&self) -> i64 {
        self.data
    }
    // Add radians
}