use crate::vector3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: vector3::Vector3,
    pub direction: vector3::Vector3,
}

impl Ray {
    pub fn new(origin: vector3::Vector3, direction: vector3::Vector3) -> Self {
        Ray {
            origin,
            direction,
        }
    }
}

impl Default for Ray {
    fn default() -> Self {
        Ray {
            origin: vector3::Vector3::default(),
            direction: vector3::Vector3::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let origin = vector3::Vector3::new(1.0, 2.0, 3.0);
        let direction = vector3::Vector3::new(4.0, 5.0, 6.0);

        let ray = Ray::new(origin, direction);

        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
    }
}
