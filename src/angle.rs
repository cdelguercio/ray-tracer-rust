use std::ops;

#[derive(Clone, Copy)]
pub struct Angle {
    radians: f64,
}

impl Angle {
    pub fn new(radians: f64) -> Angle {
        Angle { radians }
    }

    pub fn from_degrees(degrees: f64) -> Angle {
        Angle::new(degrees * std::f64::consts::PI / 180.0)
    }

    pub fn from_radians(radians: f64) -> Angle {
        Angle::new(radians)
    }

    pub fn get_radians(&self) -> f64 {
        self.radians
    }

    pub fn get_degrees(&self) -> f64 {
        self.radians * 180.0 / std::f64::consts::PI
    }
}

impl ops::Mul<f64> for Angle {
    type Output = Angle;

    fn mul(self, rhs: f64) -> Angle {
        Angle::new(self.radians * rhs)
    }
}
