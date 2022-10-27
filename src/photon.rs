use crate::bounds;
use crate::color;
use crate::hit;
use crate::ray;
use crate::vector3;

#[derive(Clone, Copy, Debug)]
pub struct Photon {
    pub ray: ray::Ray,
    pub color: color::Color,
    pub bounces: u32,
}

impl Default for Photon {
    fn default() -> Self {
        Photon {
            ray: ray::Ray::default(),
            color: color::Color::default(),
            bounces: 0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PhotonHit {
    pub hit: hit::Hit,
    pub photon: Photon,
}

impl PhotonHit {
    fn get_bounds(&self) -> bounds::Bounds {
        bounds::Bounds::from_vector(self.hit.position)
    }

    fn get_pivot(&self) -> vector3::Vector3 {
        self.hit.position
    }
}
