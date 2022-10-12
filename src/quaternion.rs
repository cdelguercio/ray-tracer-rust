use std::ops;

use crate::vector3;

#[derive(Copy, Clone)]
pub struct Quaternion {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Quaternion {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Quaternion {
        Quaternion { x, y, z, w }
    }

    fn from_roll_pitch_yaw(roll: f64, pitch: f64, yaw: f64) -> Quaternion {
        let (sin_roll, cos_roll) = (roll / 2.0).sin_cos();
        let (sin_pitch, cos_pitch) = (pitch / 2.0).sin_cos();
        let (sin_yaw, cos_yaw) = (yaw / 2.0).sin_cos();

        Quaternion {
            x: cos_yaw * sin_pitch * cos_roll + sin_yaw * cos_pitch * sin_roll,
            y: cos_yaw * cos_pitch * sin_roll - sin_yaw * sin_pitch * cos_roll,
            z: sin_yaw * cos_pitch * cos_roll - cos_yaw * sin_pitch * sin_roll,
            w: cos_yaw * cos_pitch * cos_roll + sin_yaw * sin_pitch * sin_roll,
        }
    }

    fn from_axis_angle(axis: vector3::Vector3, angle: f64) -> Quaternion {
        let (sin, cos) = (angle / 2.0).sin_cos();

        Quaternion {
            x: axis.get_x() * sin,
            y: axis.get_y() * sin,
            z: axis.get_z() * sin,
            w: cos,
        }
    }

    fn conjugate(&self) -> Quaternion {
        Quaternion::new(-self.x, -self.y, -self.z, self.w)
    }

    fn inverse(&self) -> Quaternion {
        self.conjugate() * (1.0 / self.norm_squared())
    }

    fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    fn norm_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }
}

impl Default for Quaternion {
    fn default() -> Self {
        Quaternion {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }
}

impl ops::Mul<f64> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: f64) -> Quaternion {
        Quaternion {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs
        }
    }
}

impl ops::Mul<vector3::Vector3> for Quaternion { // TODO replace with Vector3
    type Output = vector3::Vector3;

    fn mul(self, rhs: vector3::Vector3) -> vector3::Vector3 {
        let v = Quaternion::new(rhs.get_x(), rhs.get_y(), rhs.get_z(), 0.0);
        let v = self * v * self.conjugate();

        vector3::Vector3::new(v.x, v.y, v.z)
    }
}

impl ops::Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Quaternion) -> Quaternion {
        Quaternion {
            x: self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            y: self.w * rhs.y + self.y * rhs.w + self.z * rhs.x - self.x * rhs.z,
            z: self.w * rhs.z + self.z * rhs.w + self.x * rhs.y - self.y * rhs.x,
            w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::f64::consts;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn from_roll_pitch_yaw() {
        let (roll, pitch, yaw) = (0.0, 0.0, consts::PI / 2.0);
        let q = Quaternion::from_roll_pitch_yaw(roll, pitch, yaw);
        assert_approx_eq!(q.x, 0.0, 1e-6f64);
        assert_approx_eq!(q.y, 0.0, 1e-6f64);
        assert_approx_eq!(q.z, consts::FRAC_1_SQRT_2, 1e-6f64);
        assert_approx_eq!(q.w, consts::FRAC_1_SQRT_2, 1e-6f64);
    }

    #[test]
    fn from_axis_angle() {
        let axis = vector3::Vector3::new(1.0, 0.0, 0.0);
        let angle = consts::PI / 2.0;
        let q = Quaternion::from_axis_angle(axis, angle);
        assert_approx_eq!(q.x, consts::FRAC_1_SQRT_2, 1e-6f64);
        assert_approx_eq!(q.y, 0.0, 1e-6f64);
        assert_approx_eq!(q.z, 0.0, 1e-6f64);
        assert_approx_eq!(q.w, consts::FRAC_1_SQRT_2, 1e-6f64);
    }

    #[test]
    fn conjugate() {
        let q = Quaternion {x: 2.0, y: 3.0, z: 4.0, w: 1.0};
        let q_conj = q.conjugate();
        assert_approx_eq!(q_conj.x, -2.0, 1e-6f64);
        assert_approx_eq!(q_conj.y, -3.0, 1e-6f64);
        assert_approx_eq!(q_conj.z, -4.0, 1e-6f64);
        assert_approx_eq!(q_conj.w, 1.0, 1e-6f64);
    }

    #[test]
    fn inverse() {
        let q = Quaternion {x: 1.0, y: 2.0, z: 3.0, w: 4.0};
        let q_inv = q.inverse();
        assert_approx_eq!(q_inv.x, -1.0 / 30.0, 1e-6f64);
        assert_approx_eq!(q_inv.y, -2.0 / 30.0, 1e-6f64);
        assert_approx_eq!(q_inv.z, -3.0 / 30.0, 1e-6f64);
        assert_approx_eq!(q_inv.w, 4.0 / 30.0, 1e-6f64);
    }

    #[test]
    fn norm() {
        let q = Quaternion {x: 1.0, y: 2.0, z: 3.0, w: 4.0};
        assert_approx_eq!(q.norm(), 5.4772, 1e-3f64);
    }

    #[test]
    fn norm_squared() {
        let q = Quaternion {x: 1.0, y: 2.0, z: 3.0, w: 4.0};
        assert_approx_eq!(q.norm_squared(), f64::powi(5.4772, 2), 1e-3f64);
    }

    #[test]
    fn mul_quaternion_quaternion() {
        let q1 = Quaternion {x: 2.0, y: 3.0, z: 4.0, w: 1.0};
        let q2 = Quaternion {x: 3.0, y: 4.0, z: 5.0, w: 2.0};
        let q_product = q1 * q2;
        assert_approx_eq!(q_product.x, 6.0, 1e-6f64);
        assert_approx_eq!(q_product.y, 12.0, 1e-6f64);
        assert_approx_eq!(q_product.z, 12.0, 1e-6f64);
        assert_approx_eq!(q_product.w, -36.0, 1e-6f64);
    }

    #[test]
    fn mul_quaternion_f64() {
        let q1 = Quaternion {x: 2.0, y: 3.0, z: 4.0, w: 1.0};
        let s = 3.0;
        let q_product = q1 * s;
        assert_approx_eq!(q_product.x, 6.0, 1e-6f64);
        assert_approx_eq!(q_product.y, 9.0, 1e-6f64);
        assert_approx_eq!(q_product.z, 12.0, 1e-6f64);
        assert_approx_eq!(q_product.w, 3.0, 1e-6f64);
    }
}
