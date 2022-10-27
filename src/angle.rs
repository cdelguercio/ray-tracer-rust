use std::f64::consts;
use std::ops;

#[derive(Clone, Copy)]
pub struct Angle {
    radians: f64,
}

impl Angle {
    pub fn from_degrees(degrees: f64) -> Self {
        Angle::from_radians(degrees * consts::PI / 180.0)
    }

    pub fn from_radians(radians: f64) -> Self {
        Angle {
            radians
        }
    }

    pub fn get_radians(&self) -> f64 {
        self.radians
    }

    pub fn get_degrees(&self) -> f64 {
        self.radians * 180.0 / consts::PI
    }
}

impl ops::Mul<f64> for Angle {
    type Output = Angle;

    fn mul(self, rhs: f64) -> Angle {
        Angle::from_radians(self.radians * rhs)
    }
}
