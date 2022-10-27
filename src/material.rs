use crate::{color, photon, random_generator, vector3};

pub trait Material {
    fn get_name(&self) -> String;
    fn color_for_hit(&self, pixel_direction: &vector3::Vector3, photon_hit: &photon::PhotonHit) -> color::Color;
    fn bounce(&self, photon_hit: &photon::PhotonHit, generator: &mut random_generator::RandomGenerator) -> photon::Photon;
}
