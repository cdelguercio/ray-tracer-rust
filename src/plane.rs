use crate::hit;
use crate::ray;
use crate::vector3;

pub struct Plane {
    pub normal: vector3::Vector3,
    pub dot: f64,
}

impl Plane {
    pub fn new(a: vector3::Vector3, b: vector3::Vector3, c: vector3::Vector3) -> Self {
        let normal = vector3::Vector3::cross(&(b - a), &(c - a)).normalize();
        let dot = vector3::Vector3::dot(&normal, &a);
        Plane {
            normal,
            dot,
        }
    }

    fn point_above_plane(&self, point: &vector3::Vector3) -> bool {
        vector3::Vector3::dot(&self.normal, point) > self.dot
    }

    fn ray_intersects(&self, ray: &ray::Ray) -> Option<hit::Hit> {
        let dot = vector3::Vector3::dot(&ray.direction, &self.normal);

        if dot.abs() < f64::EPSILON {
            return None;
        }

        let t = (self.dot - vector3::Vector3::dot(&self.normal, &ray.origin)) / dot;

        if t < 0.0 {
            return None;
        }

        let position = ray.origin + ray.direction * t;
        Some(
            hit::Hit::new(
                position,
                self.normal,
                (position - ray.origin).norm(),
                0,
            )
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::f64::consts;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn plane_new() {
        let a = vector3::Vector3::new(2.0, 3.0, 4.0);
        let b = vector3::Vector3::new(5.0, 6.0, 7.0);
        let c = vector3::Vector3::new(1.0, 8.0, 9.0);

        let plane = Plane::new(a, b, c);

        assert_approx_eq!(plane.normal.get_x(), 0.0, 1e-6f64);
        assert_approx_eq!(plane.normal.get_y(), -consts::FRAC_1_SQRT_2, 1e-6f64);
        assert_approx_eq!(plane.normal.get_z(), consts::FRAC_1_SQRT_2, 1e-6f64);
        assert_approx_eq!(plane.dot, consts::FRAC_1_SQRT_2, 1e-6f64);
    }

    #[test]
    fn plane_point_above_plane() {
        let a = vector3::Vector3::new(2.0, 3.0, 4.0);
        let b = vector3::Vector3::new(5.0, 6.0, 7.0);
        let c = vector3::Vector3::new(1.0, 8.0, 9.0);

        let plane = Plane::new(a, b, c);

        let point_above = vector3::Vector3::new(0.0, 0.0, 10.0);
        let point_below = vector3::Vector3::new(0.0, 20.0, 10.0);

        assert_eq!(plane.point_above_plane(&point_above), true);
        assert_eq!(plane.point_above_plane(&point_below), false);
    }

    #[test]
    fn ray_intersects() {
        // intersection from the correct side
        let ray1 = ray::Ray::new(
            vector3::Vector3::new(0.25, 0.25, -1.0),
            vector3::Vector3::new(0.0, 0.0, 1.0),
        );

        // TODO(cdelguercio): is this correct?
        // intersection from the wrong side still intersects because this is a plane and not a triangle
        let ray2 = ray::Ray::new(
            vector3::Vector3::new(0.25, 0.25, 1.0),
            vector3::Vector3::new(0.0, 0.0, -1.0),
        );

        // near miss still intersects because this is a plane and not a triangle
        let near_miss_ray = ray::Ray::new(
            vector3::Vector3::new(0.51, 0.51, -1.0),
            vector3::Vector3::new(0.0, 0.0, 1.0),
        );

        let plane = Plane::new(
            vector3::Vector3::new(0.0, 0.0, 0.0),
            vector3::Vector3::new(1.0, 0.0, 0.0),
            vector3::Vector3::new(0.0, 1.0, 0.0),
        );

        assert!(plane.ray_intersects(&ray1).is_some());
        assert!(plane.ray_intersects(&ray2).is_some());
        assert!(plane.ray_intersects(&near_miss_ray).is_some());
    }
}
