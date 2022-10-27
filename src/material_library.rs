use crate::{color, diffuse_material, library, material};

impl library::Library<Box<dyn material::Material>> {
    pub fn build_material_library() -> library::Library<Box<dyn material::Material>> {
        let mut l: library::Library<Box<dyn material::Material>> = library::Library::new();
        l.add("default", Box::new(diffuse_material::DiffuseMaterial::new("default")));
        l.add("White", Box::new(diffuse_material::DiffuseMaterial::from_color("White", &color::Color::new(1.0, 1.0, 1.0))));
        l.add("Black", Box::new(diffuse_material::DiffuseMaterial::from_color("Black", &color::Color::new(0.0, 0.0, 0.0))));
        l.add("Red", Box::new(diffuse_material::DiffuseMaterial::from_color("Red", &color::Color::new(1.0, 0.0, 0.0))));
        l.add("Yellow", Box::new(diffuse_material::DiffuseMaterial::from_color("Yellow", &color::Color::new(1.0, 1.0, 0.0))));
        l.add("Green", Box::new(diffuse_material::DiffuseMaterial::from_color("Green", &color::Color::new(0.0, 1.0, 0.0))));
        l.add("Cyan", Box::new(diffuse_material::DiffuseMaterial::from_color("Cyan", &color::Color::new(0.0, 1.0, 1.0))));
        l.add("Blue", Box::new(diffuse_material::DiffuseMaterial::from_color("Blue", &color::Color::new(0.0, 0.0, 1.0))));
        l.add("Magenta", Box::new(diffuse_material::DiffuseMaterial::from_color("Magenta", &color::Color::new(1.0, 0.0, 1.0))));

        l
    }
}
