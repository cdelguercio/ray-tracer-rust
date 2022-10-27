use crate::{hit, ray, tree, triangle};

pub struct Mesh {
    pub m_name: String,
    m_tree: tree::Tree<triangle::Triangle>
}

impl Mesh {
    pub fn new(name: &str, triangles: Vec<triangle::Triangle>) -> Self {
        Mesh {
            m_name: String::from(name),
            m_tree: tree::Tree::new(triangles),
        }
    }

    pub fn cast_ray(&self, ray: &ray::Ray, cast_buffer: &mut Vec<hit::Hit>) -> Option<hit::Hit> {
        self.m_tree.cast_ray(ray, cast_buffer)
    }
}
