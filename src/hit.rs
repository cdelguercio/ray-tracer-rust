use crate::vector3;

#[derive(Clone, Copy, Debug)]
pub struct Hit {
    pub position: vector3::Vector3,
    pub normal: vector3::Vector3,
    pub distance: f64,
    pub material_index: usize,
}

impl Hit {
    pub fn new(position: vector3::Vector3, normal: vector3::Vector3, distance: f64, material_index: usize) -> Self {
        Hit {
            position,
            normal,
            distance,
            material_index,
        }
    }
}

impl Default for Hit {
    fn default() -> Self {
        Hit {
            position: vector3::Vector3::default(),
            normal: vector3::Vector3::default(),
            distance: 0.0,
            material_index: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hit_new() {
        let position = vector3::Vector3::new(1.0, 2.0, 3.0);
        let normal = vector3::Vector3::new(4.0, 5.0, 6.0);
        let distance = 7.0;
        let material_index = 8;

        let hit = Hit::new(position, normal, distance, material_index);

        assert_eq!(hit.position, position);
        assert_eq!(hit.normal, normal);
        assert_eq!(hit.distance, distance);
        assert_eq!(hit.material_index, material_index);
    }
}
