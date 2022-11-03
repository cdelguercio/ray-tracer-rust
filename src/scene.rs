use crate::{camera, volume, light, library, material};

pub struct Scene {
    pub camera: camera::Camera,
    pub volumes: Vec<Box<dyn volume::VolumePublicInterface>>,
    pub lights: Vec<Box<dyn light::LightPublicInterface>>,
    pub material_library: library::Library<Box<dyn material::Material>>,
}


unsafe impl Sync for Scene {}

unsafe impl Send for Scene {}
