use crate::{color, object, photon, quaternion, random_generator, vector3};

pub trait LightStrategy {
    // fn update_parameters(&mut self); // TODO this one doesn't need to be overridden? just access, so it's in the LightProtectedInterface but not here?
    fn emit(&self, base: &Light<Self>, photon: &mut photon::Photon, photon_brightness: f64, random_generator: &mut random_generator::RandomGenerator);
}

pub trait LightPublicInterface {
    fn get_color(&self) -> color::Color;
    fn set_color(&mut self, color: color::Color);
    fn get_brightness(&self) -> f64;
    fn set_brightness(&mut self, brightness: f64);
    fn emit(&self, photon: &mut photon::Photon, photon_brightness: f64, random_generator: &mut random_generator::RandomGenerator);
    fn set_position(&mut self, position: vector3::Vector3); // TODO(cdelguercio): maybe have Light derive from Object?
    fn set_rotation(&mut self, rotation: quaternion::Quaternion);
}

pub trait LightProtectedInterface {
    fn update_parameters(&mut self);
    fn object(&self) -> &object::Object;
    fn get_lumens(&self) -> f64;
}

pub struct Light<T: ?Sized> {
    pub specialization: Box<T>,
    pub m_color: color::Color,
    m_brightness: f64,
    pub m_area: f64,
    pub m_lumens: f64,
    pub object: object::Object,
}

impl<T: LightStrategy> Light<T> {
    pub fn new(specialization: T) -> Light<T> {
        let mut l = Light {
            specialization: Box::new(specialization),
            m_color: color::Color::new(1.0, 1.0, 1.0),
            m_brightness: 0.0,
            m_area: 0.0,
            m_lumens: 0.0,
            object: object::Object::new(),
        };
        l.update_parameters();

        l
    }
}

impl<T: LightStrategy> LightPublicInterface for Light<T> {
    fn get_color(&self) -> color::Color {
        self.m_color
    }

    fn set_color(&mut self, color: color::Color) {
        self.m_color = color;
        self.update_parameters();
    }

    fn get_brightness(&self) -> f64 {
        self.m_brightness
    }

    fn set_brightness(&mut self, brightness: f64) {
        self.m_brightness = brightness;
        self.update_parameters();
    }

    fn emit(&self, photon: &mut photon::Photon, photon_brightness: f64, random_generator: &mut random_generator::RandomGenerator) {
        self.specialization.emit(self, photon, photon_brightness, random_generator)
    }

    fn set_position(&mut self, position: vector3::Vector3) {
        self.object.transform.position = position;
    }

    fn set_rotation(&mut self, rotation: quaternion::Quaternion) {
        self.object.transform.rotation = rotation;
    }
}

impl<T: LightStrategy> LightProtectedInterface for Light<T> {
    fn update_parameters(&mut self) {
        if self.m_area > 0.0 {
            self.m_lumens = self.m_brightness * self.m_area;
        } else {
            self.m_lumens = self.m_brightness;
        }
    }

    fn object(&self) -> &object::Object {
        &self.object
    }

    fn get_lumens(&self) -> f64 {
        self.m_lumens
    }
}
