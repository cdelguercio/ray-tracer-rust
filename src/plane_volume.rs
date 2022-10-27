use crate::{hit, plane, ray, vector3, volume};

pub struct PlaneVolumeStrategy {
    m_plane: plane::Plane,
}

// This was the old way that didn't work:
// impl<T: volume::VolumeProtectedInterface> volume::VolumeStrategy<T> for PlaneVolumeStrategy {
impl volume::VolumeStrategy for PlaneVolumeStrategy {
    fn cast_transformed_ray(&self, ray: &ray::Ray, cast_buffer: &mut Vec<hit::Hit>) -> Option<hit::Hit> {
        None // TODO(cdelguercio): Is this usable like this or not?
    }
}

pub struct PlaneVolume {
    volume: volume::Volume<PlaneVolumeStrategy>
}

impl PlaneVolume {
    pub fn new(material_index: usize) -> Self {
        PlaneVolume {
            volume: volume::Volume::<PlaneVolumeStrategy>::new(material_index, PlaneVolumeStrategy {
                m_plane: plane::Plane::new(vector3::Vector3::default(), vector3::Vector3::new(0.0, 0.0, 1.0), vector3::Vector3::new(1.0, 0.0, 0.0)),
            }),
        }
    }
}

impl volume::VolumePublicInterface for PlaneVolume {
    fn get_material_index(&self) -> usize {
        self.volume.get_material_index()
    }

    fn set_material_index(&mut self, index: usize) {
        self.volume.set_material_index(index);
    }

    fn cast_ray(&self, ray: &ray::Ray, cast_buffer: &mut Vec<hit::Hit>) -> Option<hit::Hit> {
        self.volume.cast_ray(ray, cast_buffer)
    }
}
