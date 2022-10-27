use std::path;

use crate::{mesh, triangle};

pub fn load_meshes(path: &path::Path) -> Vec<mesh::Mesh> {
    let (models, _) =
        tobj::load_obj(
            &path,
            &tobj::LoadOptions::default()
        )
            .expect("Failed to OBJ load file");

    let mut meshes = Vec::<mesh::Mesh>::new();

    for model in models {
        let mut triangles = Vec::<triangle::Triangle>::new();
        // TODO(cdelguercio): implement
        meshes.push(mesh::Mesh::new(&model.name, triangles));
    }

    meshes
}
