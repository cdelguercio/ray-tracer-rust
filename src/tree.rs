use crate::bounds;
use crate::hit;
use crate::pyramid;
use crate::ray;
use crate::vector3;

pub trait Bounded {
    fn get_bounds(&self) -> bounds::Bounds;
}

pub trait Pivotable {
    fn get_pivot(&self) -> vector3::Vector3;
    fn get_pivot_component(&self, axis: &vector3::Axis) -> f64;
}

pub trait Intersectable {
    fn ray_intersects(&self, ray: &ray::Ray) -> Option<hit::Hit>;
}

pub trait NodeObject: Bounded + Pivotable + Intersectable + Copy {}
impl<T> NodeObject for T where T: Bounded + Pivotable + Intersectable + Copy {}

struct Node<T: NodeObject> {
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,

    contents: Vec<T>,
    axis: vector3::Axis,
    pivot: f64,
    bounds: bounds::Bounds,
    depth: u32,
}

impl<T: NodeObject> Default for Node<T> {
    fn default() -> Self {
        Node {
            left: None,
            right: None,
            contents: Vec::<T>::new(),
            axis: vector3::Axis::X,
            pivot: 0.0,
            bounds: bounds::Bounds::default(),
            depth: 0,
        }
    }
}

impl<T: NodeObject> Node<T> {
    fn count(&self) -> usize {
        1 +
            self.left.as_ref().map_or(0, |l| l.count()) +
            self.right.as_ref().map_or(0, |r| r.count())
    }

    fn depth(&self) -> usize {
        let mut depth: usize = 1;
        depth = self.left.as_ref().map_or(0, |l| l.depth() + 1).max(depth);
        depth = self.right.as_ref().map_or(0, |r| r.depth() + 1).max(depth);

        depth
    }
}

pub struct Tree<T: NodeObject> {
    root: Node<T>,
    m_page_size: usize,
}

impl<T: NodeObject> Tree<T> {
    pub fn new(objects: Vec<T>) -> Tree<T> {
        Tree::new_with_page_size(objects, 1)
    }

    pub fn new_with_page_size(objects: Vec<T>, page_size: usize) -> Tree<T> {
        Tree {
            root: Tree::generate(objects, &vector3::Axis::X, &page_size),
            m_page_size: page_size,
        }
    }

    fn generate(objects: Vec<T>, axis: &vector3::Axis, page_size: &usize) -> Node<T> {
        if objects.len() == 0 {
            return Node::default();
        }

        let mut bounds = objects[0].get_bounds();

        for object in objects.iter() {
            bounds += object.get_bounds();
        }

        return if objects.len() <= *page_size {
            Node {
                left: None,
                right: None,
                contents: objects,
                axis: (*axis).clone(),
                pivot: 0.0,
                bounds,
                depth: 0,
            }
        } else {
            let axis = Tree::<T>::next_non_zero_axis(&bounds, axis);
            let pivot = Tree::<T>::median_pivot_component(&objects, &axis);
            let mut left_objects = Vec::<T>::new();
            let mut middle_objects = Vec::<T>::new();
            let mut right_objects = Vec::<T>::new();
            let left: Option<Box<Node<T>>>;
            let right: Option<Box<Node<T>>>;
            let contents: Vec<T>;

            for object in objects.iter() {
                let pivot_component = object.get_pivot_component(&axis);
                if pivot_component < pivot {
                    left_objects.push(*object);
                } else if pivot_component > pivot {
                    right_objects.push(*object);
                } else {
                    middle_objects.push(*object);
                }
            }

            if left_objects.len() > 0 { // TODO(cdelguercio): replaces these with match statements
                left = Some(Box::new(Tree::<T>::generate(left_objects, &axis, page_size)));
            } else {
                left = None;
            }

            if right_objects.len() > 0 {
                right = Some(Box::new(Tree::<T>::generate(right_objects, &axis, page_size)));
            } else {
                right = None;
            }

            if middle_objects.len() > 0 {
                contents = middle_objects;
            } else {
                contents = Vec::<T>::new();
            }

            Node {
                left,
                right,
                contents,
                axis,
                pivot,
                bounds,
                depth: 0, // TODO(cdelguercio): calculate depth?
            }
        }
    }

    // Find the next non-zero bound, if available
    // TODO(cdelguercio for ignotuscaligo): this implementation differs from the C++ version in that
    // it checks if the current axis is valid LAST instead of FIRST. I'm not sure which one is correct.
    fn next_non_zero_axis(bounds: &bounds::Bounds, axis: &vector3::Axis) -> vector3::Axis {
        let mut valid_axis =(*axis).clone();

        loop {
            valid_axis = valid_axis.next();

            if valid_axis == *axis {
                break;
            }

            let limits = bounds.get_limits(&valid_axis);

            if limits.max - limits.min > f64::EPSILON {
                break;
            }
        }

        valid_axis
    }

    fn median_pivot_component(objects: &Vec<T>, axis: &vector3::Axis) -> f64 {
        let mut values = Vec::new();
        for object in objects {
            values.push(object.get_pivot_component(axis));
        }
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        values[values.len() / 2]
    }

    pub fn cast_ray(&self, ray: &ray::Ray, cast_buffer: &mut Vec<hit::Hit>) -> Option<hit::Hit> {
        cast_buffer.clear();

        Self::cast_ray_into_node(ray, &self.root, cast_buffer);

        let min_distance = f64::INFINITY;
        let mut result: Option<hit::Hit> = None;

        for hit in cast_buffer {
            if hit.distance < min_distance {
                result = Some(hit.clone());
            }
        }

        result
    }

    pub fn fetch_within_pyramid(&self, pyramid: &pyramid::Pyramid) -> Vec<T> {
        let mut objects: Vec<T> = Vec::new();

        Self::fetch_within_pyramid_from_node(pyramid, &self.root, &mut objects);

        objects
    }

    fn cast_ray_into_node(ray: &ray::Ray, node: &Node<T>, hits: &mut Vec<hit::Hit>) {
        if node.bounds.ray_intersects(ray) {
            for content in &node.contents {
                if let Some(hit) = content.ray_intersects(ray) {
                    hits.push(hit);
                }
            }

            if let Some(left) = &node.left {
                Self::cast_ray_into_node(ray, left, hits);
            }

            if let Some(right) = &node.right {
                Self::cast_ray_into_node(ray, right, hits);
            }
        }
    }

    fn fetch_within_pyramid_from_node(pyramid: &pyramid::Pyramid, node: &Node<T>, objects: &mut Vec<T>) {
        if pyramid.intersects_bounds(&node.bounds) {
            for content in &node.contents {
                if pyramid.contains_point(&content.get_pivot()) {
                    objects.push(*content);
                }
            }

            if let Some(left) = &node.left {
                Self::fetch_within_pyramid_from_node(pyramid, left, objects);
            }

            if let Some(right) = &node.right {
                Self::fetch_within_pyramid_from_node(pyramid, right, objects);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{angle, limits, quaternion, triangle};

    #[test]
    fn cast_ray() {
        let mut hits: Vec<hit::Hit> = Vec::new();
        let ray = ray::Ray::new(
            vector3::Vector3::new(0.0, 0.0, 0.0),
            vector3::Vector3::new(1.0, 1.0, 1.0)
        );
        let node = Node {
            left: None,
            right: None,
            contents: vec![triangle::Triangle::new(
                vector3::Vector3::new(0.0, 0.0, 0.0),
                vector3::Vector3::new(0.0, 0.0, 1.0),
                vector3::Vector3::new(0.0, 0.0, 2.0),
            )],
            axis: vector3::Axis::X,
            pivot: 0.0,
            depth: 0,
            bounds: bounds::Bounds::new(
                limits::Limits::new(0.0, 1.0),
                limits::Limits::new(0.0, 1.0),
                limits::Limits::new(0.0, 1.0),
            ),
        };

        let tree = Tree {
            root: node,
            m_page_size: 1,
        };

        tree.cast_ray(&ray, &mut hits);

        assert_eq!(hits.len(), 1);
    }

    #[test]
    fn cast_ray_into_node() {
        let mut hits: Vec<hit::Hit> = Vec::new();
        let ray = ray::Ray::new(
            vector3::Vector3::new(0.0, 0.0, 0.0),
            vector3::Vector3::new(1.0, 1.0, 1.0)
        );
        let node = Node {
            left: None,
            right: None,
            contents: vec![triangle::Triangle::new(
                vector3::Vector3::new(0.0, 0.0, 0.0),
                vector3::Vector3::new(0.0, 0.0, 1.0),
                vector3::Vector3::new(0.0, 0.0, 2.0),
            )],
            axis: vector3::Axis::X,
            pivot: 0.0,
            depth: 0,
            bounds: bounds::Bounds::new(
                limits::Limits::new(0.0, 1.0),
                limits::Limits::new(0.0, 1.0),
                limits::Limits::new(0.0, 1.0),
            ),
        };

        let tree = Tree {
            root: node,
            m_page_size: 1,
        };

        Tree::cast_ray_into_node(&ray, &tree.root, &mut hits);

        assert_eq!(hits.len(), 1);
    }

    #[test]
    fn fetch_within_pyramid() {
        let pyramid = pyramid::Pyramid::new(
            vector3::Vector3::new(0.0, 0.0, 0.0),
            quaternion::Quaternion::new(0.0, 0.0, 0.0, 1.0),
            angle::Angle::from_radians(0.0),
            angle::Angle::from_radians(0.0),
        );
        let node = Node {
            left: None,
            right: None,
            contents: vec![triangle::Triangle::new(
                vector3::Vector3::new(0.0, 0.0, 0.0),
                vector3::Vector3::new(0.0, 0.0, 1.0),
                vector3::Vector3::new(0.0, 0.0, 2.0),
            )],
            axis: vector3::Axis::X,
            pivot: 0.0,
            depth: 0,
            bounds: bounds::Bounds::new(
                limits::Limits::new(0.0, 1.0),
                limits::Limits::new(0.0, 1.0),
                limits::Limits::new(0.0, 1.0),
            ),
        };

        let tree = Tree {
            root: node,
            m_page_size: 1,
        };

        let objects = tree.fetch_within_pyramid(&pyramid);

        assert_eq!(objects.len(), 1);
    }

    #[test]
    fn fetch_within_pyramid_from_node() {
        let mut objects: Vec<triangle::Triangle> = Vec::new();
        let pyramid = pyramid::Pyramid::new(
            vector3::Vector3::new(0.0, 0.0, 0.0),
            quaternion::Quaternion::new(0.0, 0.0, 0.0, 1.0),
            angle::Angle::from_radians(0.0),
            angle::Angle::from_radians(0.0),
        );
        let node = Node {
            left: None,
            right: None,
            contents: vec![triangle::Triangle::new(
                vector3::Vector3::new(0.0, 0.0, 0.0),
                vector3::Vector3::new(0.0, 0.0, 1.0),
                vector3::Vector3::new(0.0, 0.0, 2.0),
            )],
            axis: vector3::Axis::X,
            pivot: 0.0,
            depth: 0,
            bounds: bounds::Bounds::new(
                limits::Limits::new(0.0, 1.0),
                limits::Limits::new(0.0, 1.0),
                limits::Limits::new(0.0, 1.0),
            ),
        };

        let tree = Tree {
            root: node,
            m_page_size: 1,
        };

        Tree::fetch_within_pyramid_from_node(&pyramid, &tree.root, &mut objects);

        assert_eq!(objects.len(), 1);
    }

    #[test]
    #[should_panic]
    fn median_pivot_component_with_empty_objects() {
        let axis = vector3::Axis::X;
        let objects: Vec<triangle::Triangle> = vec![];

        Tree::median_pivot_component(&objects, &axis);
    }

    #[test]
    fn median_pivot_component() {
        let axis = vector3::Axis::X;
        let objects: Vec<triangle::Triangle> = vec![
            triangle::Triangle::new(
                vector3::Vector3::new(0.0, 0.0, 0.0),
                vector3::Vector3::new(1.0, 1.0, 0.0),
                vector3::Vector3::new(1.0, 1.0, 1.0),
            ),
            triangle::Triangle::new(
                vector3::Vector3::new(0.0, 0.0, 0.0),
                vector3::Vector3::new(2.0, 2.0, 0.0),
                vector3::Vector3::new(1.0, 1.0, 2.0),
            ),
            triangle::Triangle::new(
                vector3::Vector3::new(0.0, 0.0, 0.0),
                vector3::Vector3::new(5.0, 5.0, 0.0),
                vector3::Vector3::new(5.0, 5.0, 5.0),
            ),
        ];

        assert_eq!(Tree::median_pivot_component(&objects, &axis), 1.0);
    }

    #[test]
    fn generate() {
        let objects: Vec<triangle::Triangle> = vec![
            triangle::Triangle::new(
                vector3::Vector3::new(0.0, 0.0, 0.0),
                vector3::Vector3::new(1.0, 1.0, 0.0),
                vector3::Vector3::new(1.0, 1.0, 1.0),
            ),
            triangle::Triangle::new(
                vector3::Vector3::new(0.0, 0.0, 0.0),
                vector3::Vector3::new(2.0, 2.0, 0.0),
                vector3::Vector3::new(1.0, 1.0, 2.0),
            ),
            triangle::Triangle::new(
                vector3::Vector3::new(0.0, 0.0, 0.0),
                vector3::Vector3::new(5.0, 5.0, 0.0),
                vector3::Vector3::new(5.0, 5.0, 5.0),
            ),
        ];

        let tree = Tree::<triangle::Triangle>::new_with_page_size(objects, 1);

        assert_eq!(tree.root.contents.len(), 1);
    }
}
