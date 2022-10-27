use crate::bounds;
use crate::hit;
use crate::limits;
use crate::ray;
use crate::tree;
use crate::vector3;

#[derive(Copy, Clone)]
pub struct Triangle {
    pub a: vector3::Vector3,
    pub b: vector3::Vector3,
    pub c: vector3::Vector3,

    center: vector3::Vector3,
    normal: vector3::Vector3,

    a_normal: vector3::Vector3,
    b_normal: vector3::Vector3,
    c_normal: vector3::Vector3,
}

impl Triangle {
    pub fn new(a: vector3::Vector3, b: vector3::Vector3, c: vector3::Vector3) -> Self {
        let center = (a + b + c) / 3.0;
        let normal = vector3::Vector3::cross(&(b - a), &(c - a)).normalize();

        let a_normal = vector3::Vector3::cross(&(b - a), &(center - a)).normalize();
        let b_normal = vector3::Vector3::cross(&(c - b), &(center - b)).normalize();
        let c_normal = vector3::Vector3::cross(&(a - c), &(center - c)).normalize();

        Triangle {
            a,
            b,
            c,

            center,
            normal,

            a_normal,
            b_normal,
            c_normal,
        }
    }

    pub fn get_position(&self, coords: vector3::Vector3) -> vector3::Vector3 {
        self.a * coords.get_x() + self.b * coords.get_y() + self.c * coords.get_z()
    }

    pub fn get_normal(&self, coords: vector3::Vector3) -> vector3::Vector3 {
        self.a_normal * coords.get_x() + self.b_normal * coords.get_y() + self.c_normal * coords.get_z()
    }

    fn get_limits(&self, axis: &vector3::Axis) -> limits::Limits {
        limits::Limits::new(
            self.a.get_component(axis).min(self.b.get_component(axis)).min(self.c.get_component(axis)),
            self.a.get_component(axis).max(self.b.get_component(axis)).max(self.c.get_component(axis)),
        )
    }
}

impl tree::Pivotable for Triangle {
    fn get_pivot(&self) -> vector3::Vector3 {
        self.center
    }

    fn get_pivot_component(&self, axis: &vector3::Axis) -> f64 {
        self.center.get_component(axis)
    }
}

impl tree::Bounded for Triangle {
    fn get_bounds(&self) -> bounds::Bounds {
        bounds::Bounds::new(self.get_limits(&vector3::Axis::X),
                            self.get_limits(&vector3::Axis::Y),
                            self.get_limits(&vector3::Axis::Z))
    }
}

impl tree::Intersectable for Triangle {
    fn ray_intersects(&self, ray: &ray::Ray) -> Option<hit::Hit> {
        let ab = self.b - self.a;
        let ac = self.c - self.a;
        let qp = -ray.direction;

        let n = vector3::Vector3::cross(&ab, &ac);

        let d = vector3::Vector3::dot(&qp, &n);

        if d <= 0.0 {
            return None;
        }

        let ap = ray.origin - self.a;
        let t = vector3::Vector3::dot(&ap, &n);

        if t < 0.0 {
            return None;
        }

        let e = vector3::Vector3::cross(&qp, &ap);
        let mut v = vector3::Vector3::dot(&ac, &e);

        if v < 0.0 || v > d {
            return None;
        }

        let mut w = -vector3::Vector3::dot(&ab, &e);
        if w < 0.0 || v + w > d {
            return None;
        }

        let ood = 1.0 / d;
        v *= ood;
        w *= ood;
        let u = 1.0 - v - w;

        let coords = vector3::Vector3::new(u, v, w);

        let position = self.get_position(coords);
        Some(
            hit::Hit::new(
                position,
                self.get_normal(coords).normalize(),
                (position - ray.origin).norm(),
                0,
            )
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tree::Intersectable;

    use std::f64::consts;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn triangle_new() {
        let a = vector3::Vector3::new(2.0, 3.0, 4.0);
        let b = vector3::Vector3::new(5.0, 6.0, 7.0);
        let c = vector3::Vector3::new(1.0, 8.0, 9.0);

        let triangle = Triangle::new(a, b, c);

        assert_eq!(triangle.a, a);
        assert_eq!(triangle.b, b);
        assert_eq!(triangle.c, c);

        assert_approx_eq!(triangle.center.get_x(), 2.6666666666666665, 1e-6f64);
        assert_approx_eq!(triangle.center.get_y(), 5.666666666666667, 1e-6f64);
        assert_approx_eq!(triangle.center.get_z(), 6.666666666666667, 1e-6f64);

        assert_approx_eq!(triangle.normal.get_x(), 0.0, 1e-6f64);
        assert_approx_eq!(triangle.normal.get_y(), -consts::FRAC_1_SQRT_2, 1e-6f64);
        assert_approx_eq!(triangle.normal.get_z(), consts::FRAC_1_SQRT_2, 1e-6f64);

        assert_approx_eq!(triangle.a_normal.get_x(), 0.0, 1e-6f64);
        assert_approx_eq!(triangle.a_normal.get_y(), -consts::FRAC_1_SQRT_2, 1e-6f64);
        assert_approx_eq!(triangle.a_normal.get_z(), consts::FRAC_1_SQRT_2, 1e-6f64);

        assert_approx_eq!(triangle.b_normal.get_x(), 0.0, 1e-6f64);
        assert_approx_eq!(triangle.b_normal.get_y(), -consts::FRAC_1_SQRT_2, 1e-6f64);
        assert_approx_eq!(triangle.b_normal.get_z(), consts::FRAC_1_SQRT_2, 1e-6f64);

        assert_approx_eq!(triangle.c_normal.get_x(), 0.0, 1e-6f64);
        assert_approx_eq!(triangle.c_normal.get_y(), -consts::FRAC_1_SQRT_2, 1e-6f64);
        assert_approx_eq!(triangle.c_normal.get_z(), consts::FRAC_1_SQRT_2, 1e-6f64);
    }

    #[test]
    fn ray_intersects() {
        // intersection from the correct side
        let ray1 = ray::Ray::new(
            vector3::Vector3::new(0.25, 0.25, -1.0),
            vector3::Vector3::new(0.0, 0.0, 1.0),
        );

        // intersection from the wrong side
        let ray2 = ray::Ray::new(
            vector3::Vector3::new(0.25, 0.25, 1.0),
            vector3::Vector3::new(0.0, 0.0, -1.0),
        );

        let near_miss_ray = ray::Ray::new(
            vector3::Vector3::new(0.51, 0.51, -1.0),
            vector3::Vector3::new(0.0, 0.0, 1.0),
        );

        let triangle = Triangle::new(
            vector3::Vector3::new(0.0, 0.0, 0.0),
            vector3::Vector3::new(0.0, 1.0, 0.0),
            vector3::Vector3::new(1.0, 0.0, 0.0),
        );

        assert!(triangle.ray_intersects(&ray1).is_some());
        assert!(triangle.ray_intersects(&ray2).is_none());
        assert!(triangle.ray_intersects(&near_miss_ray).is_none());
    }
}
