#![feature(portable_simd)]
#![feature(let_else)]

pub mod angle;
pub mod angle_generator;
mod bounds;
pub mod camera;
mod color;
mod diffuse_material;
pub mod hit;
mod image;
mod library;
mod light;
mod light_queue;
mod limits;
mod material;
mod material_library;
mod math;
pub mod mesh;
pub mod mesh_volume;
pub mod quaternion;
mod obj_reader;
mod object;
pub mod parallel_light;
pub mod photon;
mod pixel;
mod pixel_coords;
mod plane;
mod plane_volume;
mod png_writer;
mod pyramid;
pub mod random_generator;
mod ray;
pub mod renderer;
mod transform;
mod tree;
pub mod triangle;
pub mod vector3;
mod volume;
