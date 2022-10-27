use crate::{hit, object, ray};

pub trait VolumeStrategy {
    fn cast_transformed_ray(&self, ray: &ray::Ray, cast_buffer: &mut Vec<hit::Hit>) -> Option<hit::Hit>;
}

pub trait VolumePublicInterface {
    fn get_material_index(&self) -> usize;
    fn set_material_index(&mut self, index: usize);
    fn cast_ray(&self, ray: &ray::Ray, cast_buffer: &mut Vec<hit::Hit>) -> Option<hit::Hit>;
}

pub trait VolumeProtectedInterface {
    fn cast_transformed_ray(&self, ray: &ray::Ray, cast_buffer: &mut Vec<hit::Hit>) -> Option<hit::Hit>;
}

pub struct Volume<T> {
    pub specialization: T,
    pub m_material_index: usize,
    pub object: object::Object,
}

impl<T: VolumeStrategy> Volume<T> {
    pub fn new(material_index: usize, specialization: T) -> Volume<T> {
        Volume {
            specialization,
            m_material_index: material_index,
            object: object::Object::new(),
        }
    }

    fn transform_ray(&self, ray: &ray::Ray) -> ray::Ray {
        ray::Ray {
            origin: self.object.rotation().inverse() * (ray.origin - self.object.position()),
            direction: self.object.rotation().inverse() * ray.direction,
        }
    }
}

impl<T: VolumeStrategy> VolumePublicInterface for Volume<T> {
    fn get_material_index(&self) -> usize {
        self.m_material_index
    }

    fn set_material_index(&mut self, index: usize) {
        self.m_material_index = index;
    }

    fn cast_ray(&self, ray: &ray::Ray, cast_buffer: &mut Vec<hit::Hit>) -> Option<hit::Hit> {
        let transformed_ray = self.transform_ray(ray);
        let hit = self.cast_transformed_ray(&transformed_ray, cast_buffer);

        return if let Some(mut hit) = hit {
            hit.position = self.object.position() + (self.object.rotation() * hit.position);
            hit.normal = self.object.rotation() * hit.normal;
            hit.material_index = self.m_material_index;

            Some(hit)
        } else {
            None
        }
    }
}

impl<T: VolumeStrategy> VolumeProtectedInterface for Volume<T> {
    fn cast_transformed_ray(&self, ray: &ray::Ray, cast_buffer: &mut Vec<hit::Hit>) -> Option<hit::Hit> {
        self.specialization.cast_transformed_ray(ray, cast_buffer)
    }
}

