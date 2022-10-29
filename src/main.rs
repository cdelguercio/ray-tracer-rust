use std::path;
use tdi_ray_tracer::{angle, camera, hit, image, mesh, mesh_volume, parallel_light, parallel_light::LightPublicInterface, photon, png_writer, quaternion, random_generator, renderer, triangle, vector3};

fn main() {
    let image_width = 100;
    let image_height = 100;

    let mut rg = random_generator::RandomGenerator::new();

    let camera = camera::Camera::new(
        image_width,
        image_height,
        &angle::Angle::from_degrees(90.0),
    );

    let triangles = vec![
        triangle::Triangle::new(
            vector3::Vector3::new(-1.0, -1.0, 1.0),
            vector3::Vector3::new(-1.0, 1.0, 1.0),
            vector3::Vector3::new(1.0, 0.0, 1.0),
        ),
    ];

    let mesh_volume = Box::new(mesh_volume::MeshVolume::new(0, mesh::Mesh::new("test_mesh", triangles)));

    let mut light = parallel_light::ParallelLight::new();
    light.set_radius(3.0);
    light.set_position(vector3::Vector3::new(0.25, 0.25, 0.5));
    light.set_rotation(quaternion::Quaternion::from_roll_pitch_yaw(0.0, 0.0, 0.0));

    let mut photon = photon::Photon::default();

    let renderer = renderer::Renderer::new(
        camera,
        vec![mesh_volume],
    );

    let mut cast_buffer = Vec::<hit::Hit>::new();

    let mut volume_hit_buffer = Vec::<photon::PhotonHit>::new();

    let mut image = image::Image::new(image_width, image_height);

    println!("photon = \n{:?}\n", photon);

    renderer.process_light(&light, &mut photon, 1.0, &mut rg);

    println!("photon = \n{:?}\n", photon);

    let photon_hit = renderer.process_photon(&mut photon, &mut cast_buffer, &mut volume_hit_buffer);

    println!("photon_hit = \n{:?}\n", photon_hit);

    // let photon_hit = photon::PhotonHit::new();
    //
    // println!("photon_hit = \n{:?}\n", photon_hit);
    //
    // renderer.bounce_photon_hit(photon_hit, &mut rg);
    //
    // renderer.process_hit(photon_hit, cast_buffer);
    //
    // let result = renderer.process_final_hit(photon_hit);
    //
    // if let Some((pc, c)) = result {
    //     image.set_pixel(pc.x, pc.y, c);
    // }

    let png_w = png_writer::PngWriter::new(image_width as u32, image_height as u32, path::Path::new("test.png"));

    png_w.write(&image);
}
