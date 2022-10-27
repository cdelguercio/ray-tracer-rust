use std::f64::consts;

use crate::{light, photon, quaternion, random_generator, ray, vector3, color};

use crate::light::LightProtectedInterface;
pub use crate::light::LightPublicInterface;

struct ParallelLightStrategy {
    m_radius: f64,
}

impl light::LightStrategy for ParallelLightStrategy {
    fn emit(&self, base: &light::Light<ParallelLightStrategy>, photon: &mut photon::Photon, photon_brightness: f64, random_generator: &mut random_generator::RandomGenerator) {
        let direction = base.object.forward(); // TODO(cdelguercio) we probably want to change all references to base.X to parameters, so that we aren't recalculating them for every photon; or we find another way to cache the values
        let photon_color = base.m_color * base.m_lumens * photon_brightness;

        let mut offset;
        if self.m_radius > 0.0 {
            offset = vector3::Vector3::random(random_generator, self.m_radius);
            // project onto plane with `direction` normal
            offset = offset - (direction * vector3::Vector3::dot(&offset, &direction));
            // spread out distribution so that it is uniform
            offset = vector3::Vector3::normalized(&offset) * ((offset.norm() / self.m_radius).sqrt() * self.m_radius);
        } else {
            offset = vector3::Vector3::default();
        }

        photon.ray = ray::Ray::new(base.object.position() + offset, direction);
        photon.color = photon_color;
        photon.bounces = 0;
    }
}

pub struct ParallelLight {
    light: light::Light<ParallelLightStrategy>,
}

impl ParallelLight {
    pub fn new() -> ParallelLight {
        ParallelLight {
            light: light::Light::<ParallelLightStrategy>::new(ParallelLightStrategy {
                m_radius: 0.0,
            }),
        }
    }

    pub fn set_radius(&mut self, radius: f64) {
        self.light.specialization.m_radius = radius;
        self.light.m_area = consts::PI * radius * radius;
        self.light.update_parameters();
    }
}

impl light::LightPublicInterface for ParallelLight {
    fn get_color(&self) -> color::Color {
        self.light.get_color()
    }

    fn set_color(&mut self, color: color::Color) {
        self.light.set_color(color)
    }

    fn get_brightness(&self) -> f64 {
        self.light.get_brightness()
    }

    fn set_brightness(&mut self, brightness: f64) {
        self.light.set_brightness(brightness)
    }

    fn emit(&self, photon: &mut photon::Photon, photon_brightness: f64, random_generator: &mut random_generator::RandomGenerator) {
        self.light.emit(photon, photon_brightness, random_generator)
    }

    fn set_position(&mut self, position: vector3::Vector3) {
        self.light.set_position(position)
    }

    fn set_rotation(&mut self, rotation: quaternion::Quaternion) {
        self.light.set_rotation(rotation)
    }
}
