use crate::{quaternion, vector3};

pub struct Transform {
    pub position: vector3::Vector3,
    pub rotation: quaternion::Quaternion,
    pub scale: vector3::Vector3,
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            position: vector3::Vector3::new(0.0, 0.0, 0.0),
            rotation: quaternion::Quaternion::new(0.0, 0.0, 0.0, 1.0),
            scale: vector3::Vector3::new(1.0, 1.0, 1.0),
        }
    }

    fn forward(&self) -> vector3::Vector3 {
        self.rotation * vector3::Vector3::new(0.0, 0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transform_forward() {
        let transform = Transform {
            position: vector3::Vector3::new(0.0, 0.0, 0.0),
            rotation: quaternion::Quaternion::new(0.0, 0.0, 0.0, 1.0),
            scale: vector3::Vector3::new(1.0, 1.0, 1.0),
        };

        assert_eq!(transform.forward(), vector3::Vector3::new(0.0, 0.0, 1.0));
    }
}
