use crate::{camera, color, hit, library, light, material, photon, pixel_coords, random_generator, ray, vector3, volume};

const SELF_HIT_THRESHOLD: f64 = f64::EPSILON;

pub struct Renderer {
    // TODO(cdelguercio): maybe these should be passed into each function and each function should be static
    // TODO(cdelguercio): another useful abstraction would be a "scene" object that contains all of the objects, lights, etc.
    // camera: camera::Camera,
    // material_library: library::Library<Box<dyn material::Material>>,
    // volumes: Vec<Box<dyn volume::VolumePublicInterface>>,
    m_bounce_threshold: u32,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            // camera,
            // material_library: library::Library::build_material_library(), // TODO(cdelguercio): this should be passed in
            // volumes,
            m_bounce_threshold: 1,
        }
    }

    // process_light generates a photon from a particular light source
    // the photon_brightness should be derived from 1.0 / total_number_of_photons from the light source
    // TODO(cdelguercio): maybe we would rather return a photon::Photon instead of passing in a mutable reference
    pub fn process_light(
        &self,
        light: &dyn light::LightPublicInterface,
        photon: &mut photon::Photon,
        photon_brightness: f64,
        random_generator: &mut random_generator::RandomGenerator,
    ) {
        light.emit(photon, photon_brightness, random_generator);
    }

    // process_photon
    pub fn process_photon(
        &self,
        photon: &photon::Photon,
        cast_buffer: &mut Vec<hit::Hit>,
        volume_hit_buffer: &mut Vec::<photon::PhotonHit>,
        volumes: &Vec<Box<dyn volume::VolumePublicInterface>>,
    ) -> Option<photon::PhotonHit> {
        if photon.color.brightness() < f64::EPSILON {
            return None;
        }

        volume_hit_buffer.clear();

        for volume in volumes {
            let hit = volume.cast_ray(&photon.ray, cast_buffer);

            match hit {
                Some(hit) => {
                    volume_hit_buffer.push(photon::PhotonHit {
                        hit,
                        photon: *photon,
                    });
                },
                None => {},
            }
        }

        if volume_hit_buffer.is_empty() {
            return None;
        }

        // return the shortest valid hit from the photon
        return if let Some(hit) = volume_hit_buffer
            .iter()
            .filter(|photon_hit| photon_hit.hit.distance > SELF_HIT_THRESHOLD)
            .reduce(|a, b| return if a.hit.distance < b.hit.distance { a } else { b }) {
            Some(*hit)
        } else {
            None
        };
    }

    // bounce_photon_hit bounces a photon hit, and then optionally generates another photon
    pub fn bounce_photon_hit(
        &self,
        photon_hit: &photon::PhotonHit,
        random_generator: &mut random_generator::RandomGenerator,
        material_library: &library::Library<Box<dyn material::Material>>,
    ) -> Option<photon::Photon> {
        if photon_hit.photon.bounces >= self.m_bounce_threshold {
            return None;
        }

        let material = material_library.fetch_by_index(photon_hit.hit.material_index);

        Some(material.bounce(photon_hit, random_generator))
    }

    // process_hit determines if hit should be fully evaluated or if it should be skipped
    pub fn process_hit(
        &self,
        photon_hit: &photon::PhotonHit,
        cast_buffer: &mut Vec<hit::Hit>,
        camera: &camera::Camera,
        volumes: &Vec<Box<dyn volume::VolumePublicInterface>>,
    ) -> bool {
        let camera_position = camera.position();
        // let camera_normal = camera.forward(); // TODO(cdelguercio): this is in the original C++ code, but is never used

        let coord = camera.coord_for_point(&photon_hit.hit.position);

        // Not within the camera frustum, skip
        let Some(coord) = coord else {
            return false;
        };

        let pixel_direction = camera.pixel_direction(&coord);
        let dot = vector3::Vector3::dot(&pixel_direction, &photon_hit.hit.normal);

        // Not facing the pixel, skip
        if dot >= 0.0 {
            return false;
        }

        let path = camera_position - photon_hit.hit.position;
        let camera_distance = path.norm();

        if camera_distance < SELF_HIT_THRESHOLD {
            return false;
        }

        let ray = ray::Ray::new(photon_hit.hit.position, path / camera_distance);

        let mut closest_hit: Option<hit::Hit> = None;

        // Do any volumes obscure this hit?
        for volume in volumes {
            let hit = volume.cast_ray(&ray, cast_buffer);

            let Some(hit) = hit else {
                continue;
            };

            if hit.distance > SELF_HIT_THRESHOLD && (closest_hit.is_none() || hit.distance < closest_hit.unwrap().distance) {
                closest_hit = Some(hit);
            }
        }

        // If no object was hit, or the closest hit object is behind the camera, the hit is valid
        if closest_hit.is_none() || closest_hit.unwrap().distance > camera_distance {
            return true;
        }

        false
    }

    // process_final_hit generates a color at a particular pixel coordinate from a photon hit
    pub fn process_final_hit(
        &self,
        photon_hit: &photon::PhotonHit,
        camera: &camera::Camera,
        material_library: &library::Library<Box<dyn material::Material>>,
    ) -> Option<(pixel_coords::PixelCoords, color::Color)> {
        let coord_result = camera.coord_for_point(&photon_hit.hit.position);

        return match coord_result {
            Some(coord) => {
                let pixel_direction = camera.pixel_direction(&coord);

                let material = material_library.fetch_by_index(photon_hit.hit.material_index);

                Some((coord, material.color_for_hit(&pixel_direction, photon_hit)))
            },
            None => {
                None
            },
        }
    }
}
