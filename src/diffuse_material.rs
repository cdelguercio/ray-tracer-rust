use std::f64::consts;

use crate::{angle_generator, color, material, photon, random_generator, ray, vector3};

pub struct DiffuseMaterial {
    name: String,
    m_color: color::Color,
    m_angle_generator: angle_generator::AngleGenerator,
}

impl DiffuseMaterial {
    pub fn new(name: &str) -> DiffuseMaterial {
        DiffuseMaterial {
            name: name.to_string(),
            m_color: color::Color::new(1.0, 1.0, 1.0),
            m_angle_generator: angle_generator::AngleGenerator::default(),
        }
    }

    pub fn from_color(name: &str, color: &color::Color) -> DiffuseMaterial {
        DiffuseMaterial {
            name: name.to_string(),
            m_color: *color,
            m_angle_generator: angle_generator::AngleGenerator::default(),
        }
    }
}

impl material::Material for DiffuseMaterial {
    fn get_name(&self) -> String {
        return self.name.clone();
    }

    fn color_for_hit(&self, pixel_direction: &vector3::Vector3, photon_hit: &photon::PhotonHit) -> color::Color {
        let reflection = vector3::Vector3::reflected(&photon_hit.photon.ray.direction, &photon_hit.hit.normal);
        let reflection_dot = vector3::Vector3::dot(&-pixel_direction, &reflection);
        let brightness = ((reflection_dot + 1.0) / 2.0).max(0.0);

        self.m_color * photon_hit.photon.color * brightness
    }

    fn bounce(&self, photon_hit: &photon::PhotonHit, random_generator: &mut random_generator::RandomGenerator) -> photon::Photon {
        let reflection = vector3::Vector3::reflected(&photon_hit.photon.ray.direction, &photon_hit.hit.normal);

        let mut brightness = 1.0 / consts::PI * 2.0;

        let offset_reflection = self.m_angle_generator.generate_offset_vector(&reflection, random_generator);

        let normal_dot = vector3::Vector3::dot(&offset_reflection, &photon_hit.hit.normal);
        if normal_dot <= 0.0 {
            brightness = 0.0;
        }

        photon::Photon{
            bounces: photon_hit.photon.bounces + 1,
            color: self.m_color * photon_hit.photon.color * brightness,
            ray: ray::Ray{
                origin: photon_hit.hit.position,
                direction: offset_reflection,
            },
        }
    }
}
