use std::ops;

use strum::IntoEnumIterator;

use crate::limits;
use crate::ray;
use crate::vector3;

pub struct Bounds {
    x: limits::Limits,
    y: limits::Limits,
    z: limits::Limits,
}

impl Bounds {
    pub fn new(x: limits::Limits, y: limits::Limits, z: limits::Limits) -> Self {
        Bounds {
            x,
            y,
            z,
        }
    }
    pub fn from_vectors(min: vector3::Vector3, max: vector3::Vector3) -> Self {
        Bounds {
            x: limits::Limits::new(min.get_x(), max.get_x()),
            y: limits::Limits::new(min.get_y(), max.get_y()),
            z: limits::Limits::new(min.get_z(), max.get_z()),
        }
    }

    pub fn from_vector(v: vector3::Vector3) -> Self {
        Bounds::from_vectors(v, v)
    }

    fn extend(&mut self, limits: limits::Limits, axis: vector3::Axis) {
        match axis {
            vector3::Axis::X => {
                self.x += limits;
            },
            vector3::Axis::Y => {
                self.y += limits;
            },
            vector3::Axis::Z => {
                self.z += limits;
            },
        }
    }

    pub fn minimum(&self) -> vector3::Vector3 {
        vector3::Vector3::new(self.x.min, self.y.min, self.z.min)
    }

    pub fn maximum(&self) -> vector3::Vector3 {
        vector3::Vector3::new(self.x.max, self.y.max, self.z.max)
    }

    pub fn get_limits(&self, axis: &vector3::Axis) -> limits::Limits {
        match axis {
            vector3::Axis::X => self.x.clone(),
            vector3::Axis::Y => self.y.clone(),
            vector3::Axis::Z => self.z.clone(),
        }
    }

    fn contains(&self, vector: vector3::Vector3) -> bool {
        self.x.contains(vector.get_x()) &&
            self.y.contains(vector.get_y()) &&
            self.z.contains(vector.get_z())
    }

    fn intersects(&self, other: &Bounds) -> bool {
        self.x.intersects(&other.x) &&
            self.y.intersects(&other.y) &&
            self.z.intersects(&other.z)
    }

    pub fn ray_intersects(&self, ray: &ray::Ray) -> bool {  // TODO(cdelguercio): move to Ray?
        // t is the parameter of the line equation O{x,y,z} + D{x,y,z}*t
        let mut t_min = 0.0;
        let mut t_max = f64::INFINITY;

        for axis in vector3::Axis::iter() {

            let bounds_component = self.get_limits(&axis);
            let origin_component = ray.origin.get_component(&axis);
            let direction_component = ray.direction.get_component(&axis);

            // If the ray is parallel to the axis, it won't intersect the bounds unless it's inside
            if direction_component.abs() < f64::EPSILON {
                if !bounds_component.contains(origin_component) {
                    return false;
                }
                // For all axes if the ray hits the "far" end of a bound before the "near" end of one of
                // the other axes, it won't intersect the bounds
            } else {
                let t1 = (bounds_component.min - origin_component) / direction_component;
                let t2 = (bounds_component.max - origin_component) / direction_component;

                // set the convention that the "near" bounds component is the one closer to the
                // origin of the ray
                let t_near = t1.min(t2);
                let t_far = t1.max(t2);

                if t_near > t_min {
                    t_min = t_near;
                }
                if t_far < t_max {
                    t_max = t_far;
                }

                if t_min > t_max {
                    return false;
                }
            }
        }

        true
    }
}

impl Default for Bounds {
    fn default() -> Self {
        Bounds {
            x: limits::Limits::default(),
            y: limits::Limits::default(),
            z: limits::Limits::default(),
        }
    }
}

impl ops::AddAssign for Bounds {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains() {
        let bounds = Bounds::from_vectors(vector3::Vector3::new(0.0, 0.0, 0.0), vector3::Vector3::new(1.0, 1.0, 1.0));

        assert!(bounds.contains(vector3::Vector3::new(0.0, 0.0, 0.0)));
        assert!(bounds.contains(vector3::Vector3::new(1.0, 1.0, 1.0)));
        assert!(bounds.contains(vector3::Vector3::new(0.5, 0.5, 0.5)));
        assert!(!bounds.contains(vector3::Vector3::new(-1.0, -1.0, -1.0)));
        assert!(!bounds.contains(vector3::Vector3::new(2.0, 2.0, 2.0)));
    }

    #[test]
    fn intersects() {
        let bounds = Bounds::from_vectors(vector3::Vector3::new(0.0, 0.0, 0.0), vector3::Vector3::new(1.0, 1.0, 1.0));

        let other = Bounds::from_vectors(vector3::Vector3::new(0.0, 0.0, 0.0), vector3::Vector3::new(1.0, 1.0, 1.0));
        assert!(bounds.intersects(&other));
        assert!(other.intersects(&bounds));

        let other = Bounds::from_vectors(vector3::Vector3::new(0.5, 0.5, 0.5), vector3::Vector3::new(1.5, 1.5, 1.5));
        assert!(bounds.intersects(&other));
        assert!(other.intersects(&bounds));

        let other = Bounds::from_vectors(vector3::Vector3::new(-0.5, -0.5, -0.5), vector3::Vector3::new(0.5, 0.5, 0.5));
        assert!(bounds.intersects(&other));
        assert!(other.intersects(&bounds));

        let other = Bounds::from_vectors(vector3::Vector3::new(-2.0, -2.0, -2.0), vector3::Vector3::new(-1.0, -1.0, -1.0));
        assert!(!bounds.intersects(&other));
        assert!(!other.intersects(&bounds));

        let other = Bounds::from_vectors(vector3::Vector3::new(2.0, 2.0, 2.0), vector3::Vector3::new(3.0, 3.0, 3.0));
        assert!(!bounds.intersects(&other));
        assert!(!other.intersects(&bounds));
    }

    #[test]
    fn ray_intersects() {
        let ray = ray::Ray::new(
            vector3::Vector3::new(0.0, 0.0, 0.0),
            vector3::Vector3::new(1.0, 0.0, 0.0),
        );

        let bounds = Bounds::from_vectors(
            vector3::Vector3::new(0.0, 0.0, 0.0),
            vector3::Vector3::new(1.0, 1.0, 1.0),
        );

        assert!(bounds.ray_intersects(&ray));
    }
}
