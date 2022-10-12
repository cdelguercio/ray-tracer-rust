use std::ops;

use core_simd;
use strum_macros::EnumIter;

use crate::math;

#[derive(Copy, Clone, EnumIter, PartialEq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub fn next(&self) -> Axis {
        match self {
            Axis::X => Axis::Y,
            Axis::Y => Axis::Z,
            Axis::Z => Axis::X,
        }
    }
}

#[repr(align(64))] // TODO(cdelguercio): Not sure if this makes simd faster
#[derive(Copy, Clone, Debug)]
pub struct Vector3 {
    data: core_simd::f64x4,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 {
            data: core_simd::f64x4::from_array([x, y, z, 0.0]),
        }
    }
    pub fn new_simd(data: core_simd::f64x4) -> Vector3 {
        Vector3 { data }
    }

    pub fn get_x(&self) -> f64 {
        self.data[0]
    }

    pub fn get_y(&self) -> f64 {
        self.data[1]
    }

    pub fn get_z(&self) -> f64 {
        self.data[2]
    }

    pub fn get_component(&self, axis: &Axis) -> f64 {
        match axis {
            Axis::X => self.get_x(),
            Axis::Y => self.get_y(),
            Axis::Z => self.get_z(),
        }
    }

    pub fn norm_squared(&self) -> f64 {
        self.data[0] * self.data[0] + self.data[1] * self.data[1] + self.data[2] * self.data[2]
    }

    pub fn norm(&self) -> f64 {
        self.norm_squared().sqrt()
    }

    pub fn normalize(&self) -> Vector3 {
        let norm = self.norm();
        Vector3::new(
            self.data[0] / norm,
            self.data[1] / norm,
            self.data[2] / norm,
        )
    }

    fn normalized_simd(data: core_simd::f64x4) -> core_simd::f64x4 { // TODO(cdelguercio): why are these static?
        let dot = Vector3::dot_simd(data, data);

        // approximate inverse square root
        let inv_root = math::quake_rsqrt(dot as f32) as f64;

        data * core_simd::f64x4::from_array([inv_root, inv_root, inv_root, inv_root]) // TODO(cdelguercio): Unsure if this is correct

        // slow inverse square root
        // let root = dot.sqrt();
        //
        // data / root
    }

    pub fn normalized(a: &Vector3) -> Vector3 {
        Vector3::new_simd(Vector3::normalized_simd(a.data))
    }

    pub fn normalized_sub(a: &Vector3, b: &Vector3) -> Vector3 {
        Vector3::new_simd(Vector3::normalized_simd(a.data - b.data))
    }

    pub fn dot_simd(lhs: core_simd::f64x4, rhs: core_simd::f64x4) -> f64 {
        let dot = lhs * rhs;
        dot[0] + dot[1] + dot[2]
    }

    pub fn dot(lhs: &Vector3, rhs: &Vector3) -> f64 {
        lhs.data[0] * rhs.data[0] + lhs.data[1] * rhs.data[1] + lhs.data[2] * rhs.data[2]
    }

    pub fn cross(lhs: &Vector3, rhs: &Vector3) -> Vector3 {
        Vector3::new(
            lhs.data[1] * rhs.data[2] - lhs.data[2] * rhs.data[1],
            lhs.data[2] * rhs.data[0] - lhs.data[0] * rhs.data[2],
            lhs.data[0] * rhs.data[1] - lhs.data[1] * rhs.data[0],
        )
    }
}

impl Default for Vector3 {
    fn default() -> Self {
        Vector3 {
            data: core_simd::f64x4::from_array([0.0, 0.0, 0.0, 0.0]),
        }
    }
}

impl ops::Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3::new_simd(self.data + rhs.data)
    }
}

impl ops::Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Vector3 {
        Vector3::new_simd(self.data - rhs.data)
    }
}

impl ops::Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3::new_simd(-self.data)
    }
}

impl ops::Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Vector3 {
        Vector3::new(
            self.data[0] * rhs,
            self.data[1] * rhs,
            self.data[2] * rhs,
        )
    }
}

impl ops::Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Vector3 {
        Vector3::new(
            self.data[0] / rhs,
            self.data[1] / rhs,
            self.data[2] / rhs,
        )
    }
}

impl PartialEq for Vector3 {
    fn eq(&self, other: &Vector3) -> bool {
        self.data == other.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_simd() {
        let v = Vector3::new_simd(core_simd::f64x4::from_array([1.0, 2.0, 3.0, 0.0]));
        assert_eq!(v.data[0], 1.0);
        assert_eq!(v.data[1], 2.0);
        assert_eq!(v.data[2], 3.0);
        assert_eq!(v.data[3], 0.0);
    }

    #[test]
    fn dot_simd() {
        let v1 = Vector3::new_simd(core_simd::f64x4::from_array([1.0, 2.0, 3.0, 0.0]));
        let v2 = Vector3::new_simd(core_simd::f64x4::from_array([4.0, 5.0, 6.0, 0.0]));
        let dot = Vector3::dot_simd(v1.data, v2.data);
        assert_eq!(dot, 32.0);
    }
}
