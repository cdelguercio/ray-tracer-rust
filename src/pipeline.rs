use std::path;
use std::thread;

use crossbeam;
use kanal;

use crate::{renderer, photon, scene, pixel, png_writer, hit, image, random_generator};

pub struct Pipeline {
    renderer: renderer::Renderer,
}

impl Pipeline {
    pub fn new(renderer: renderer::Renderer) -> Self {
        Pipeline {
            renderer,
        }
    }

    pub fn render_scene(&self, scene: &scene::Scene) {
        let (photon_sender, photon_receiver): (kanal::Sender<photon::Photon>, kanal::Receiver<photon::Photon>) = kanal::unbounded();
        let (hit_sender, hit_receiver): (kanal::Sender<photon::PhotonHit>, kanal::Receiver<photon::PhotonHit>) = kanal::unbounded();
        let (final_hit_sender, final_hit_receiver): (kanal::Sender<photon::PhotonHit>, kanal::Receiver<photon::PhotonHit>) = kanal::unbounded();
        let photon_receiver_checker = photon_receiver.clone();
        let hit_receiver_checker = hit_receiver.clone();
        let final_hit_receiver_checker = final_hit_receiver.clone();

        let mut image = image::Image::new(scene.camera.width(), scene.camera.height());

        let result = crossbeam::scope(|s| {
            let process_lights_handle = s.spawn(|_| {
                let mut rg = random_generator::RandomGenerator::new();
                for _ in 0..10000 {


                    let mut photon = photon::Photon::default();

                    println!("photon = {:?}", photon);

                    for light in &scene.lights {
                        self.renderer.process_light(light.as_ref(), &mut photon, 1.0, &mut rg);
                    }

                    photon_sender.send(photon).expect("Photon Send Failed");
                }
            });

            s.spawn(|_| {
                let mut cast_buffer = Vec::<hit::Hit>::new();

                let mut volume_hit_buffer = Vec::<photon::PhotonHit>::new();

                for photon in photon_receiver {
                    let photon_hit = self.renderer.process_photon(&photon, &mut cast_buffer, &mut volume_hit_buffer, &scene.volumes);

                    println!("photon_hit = {:?}", photon_hit);

                    let Some(photon_hit) = photon_hit else {
                        continue;
                    };

                    hit_sender.send(photon_hit).expect("Photon Hit Send Failed");
                }
            });

            s.spawn(|_| {
                let mut rg = random_generator::RandomGenerator::new();

                let mut cast_buffer = Vec::<hit::Hit>::new();

                for photon_hit in hit_receiver {
                    let photon = self.renderer.bounce_photon_hit(&photon_hit, &mut rg, &scene.material_library);
                    if let Some(photon) = photon {
                        photon_sender.send(photon).expect("Photon Send Failed");
                    };

                    let photon_hit_valid = self.renderer.process_hit(&photon_hit, &mut cast_buffer, &scene.camera, &scene.volumes);
                    if photon_hit_valid {
                        final_hit_sender.send(photon_hit).expect("Final Photon Hit Send Failed");
                    };
                }
            });

            s.spawn(|_| {
                for photon_hit in final_hit_receiver {
                    let result = self.renderer.process_final_hit(&photon_hit, &scene.camera, &scene.material_library);

                    if let Some((pc, c)) = result {
                        image.set_pixel(pc.x, pc.y, pixel::Pixel::from_color(&c));
                    }
                }
            });

            process_lights_handle.join().expect("Process Lights Failed");

            s.spawn(|_| {
                loop {
                    if photon_receiver_checker.is_empty() && hit_receiver_checker.is_empty() && final_hit_receiver_checker.is_empty() {
                        photon_receiver_checker.close();
                        hit_receiver_checker.close();
                        final_hit_receiver_checker.close();
                        break;
                    }
                    thread::sleep(std::time::Duration::from_millis(1000));
                }
            });
        });

        result.unwrap();

        let png_w = png_writer::PngWriter::new(scene.camera.width() as u32, scene.camera.height() as u32, path::Path::new("test.png"));

        png_w.write(&image);
    }
}