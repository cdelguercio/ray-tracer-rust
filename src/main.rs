use tdi_ray_tracer::{angle, camera, hit, image, library, mesh, mesh_volume, parallel_light,
                     parallel_light::LightPublicInterface, photon, pipeline, pixel, png_writer,
                     quaternion, random_generator, renderer, scene, triangle, vector3};

fn main() {
    let material_library = library::Library::build_material_library();

    let image_width = 100;
    let image_height = 100;
    let camera = camera::Camera::new(
        image_width,
        image_height,
        &angle::Angle::from_degrees(90.0),
    );

    let triangles = vec![
        triangle::Triangle::new(
            vector3::Vector3::new(-1.0, -1.0, 3.0),
            vector3::Vector3::new(-1.0, 1.0, 3.0),
            vector3::Vector3::new(1.0, 0.0, 3.0),
        ),
    ];

    let mesh_volume = Box::new(mesh_volume::MeshVolume::new(material_library.index_for_name("Cyan"), mesh::Mesh::new("test_mesh", triangles)));

    let mut light = parallel_light::ParallelLight::new();
    light.set_radius(1.0);
    light.set_brightness(1000.0);
    light.set_position(vector3::Vector3::new(0.25, 0.25, 1.5));
    light.set_rotation(quaternion::Quaternion::from_roll_pitch_yaw(0.0, 0.0, 0.0));

    let s = scene::Scene {
        camera,
        volumes: vec![mesh_volume],
        lights: vec![Box::new(light)],
        material_library,
    };

    let renderer = renderer::Renderer::new();

    let p = pipeline::Pipeline::new(
        renderer,
    );

    p.render_scene(&s);
}
