use crate::{hit, mesh, ray, volume};

struct MeshVolumeStrategy {
    m_mesh: mesh::Mesh,
}

// This was the old way that didn't work:
// impl<T: volume::VolumeProtectedInterface> volume::VolumeStrategy<T> for MeshVolumeStrategy {
impl volume::VolumeStrategy for MeshVolumeStrategy {
    fn cast_transformed_ray(&self, ray: &ray::Ray, cast_buffer: &mut Vec<hit::Hit>) -> Option<hit::Hit> {
        self.m_mesh.cast_ray(ray, cast_buffer)
    }
}

pub struct MeshVolume {
    volume: volume::Volume<MeshVolumeStrategy>,
}

impl MeshVolume {
    pub fn new(material_index: usize, mesh: mesh::Mesh) -> Self {
        MeshVolume {
            volume: volume::Volume::<MeshVolumeStrategy>::new(material_index, MeshVolumeStrategy {
                m_mesh: mesh,
            }),
        }
    }

    fn get_mesh(&self) -> &mesh::Mesh {
        &self.volume.specialization.m_mesh
    }

    fn set_mesh(&mut self, mesh: mesh::Mesh) {
        self.volume.specialization.m_mesh = mesh;
    }
}

impl volume::VolumePublicInterface for MeshVolume {
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
