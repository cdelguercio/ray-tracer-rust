use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::{quaternion, transform, vector3};

pub trait ObjectTrait {
    fn object(&self) -> &Object;
}

pub struct Object {
    pub transform: transform::Transform,
    pub m_name: String,

    m_parent: Weak<RefCell<Object>>,
    m_children: Vec<Rc<RefCell<Object>>>, // maybe just use Box if we don't need to share ownership of nodes
}

impl Object {
    pub fn new() -> Self {
        Object {
            transform: transform::Transform::new(),
            m_name: String::new(),
            m_parent: Weak::new(),
            m_children: Vec::new(),
        }
    }

    pub fn position(&self) -> vector3::Vector3 {
        return if self.m_parent.upgrade().is_some() {
            self.m_parent.upgrade().unwrap().borrow().position() + (self.m_parent.upgrade().unwrap().borrow().rotation() * self.transform.position)
        } else {
            self.transform.position
        }
    }

    pub fn rotation(&self) -> quaternion::Quaternion {
        return if self.m_parent.upgrade().is_some() {
            self.m_parent.upgrade().unwrap().borrow().rotation() * self.transform.rotation
        } else {
            self.transform.rotation
        }
    }

    pub fn forward(&self) -> vector3::Vector3 {
        return self.rotation() * vector3::Vector3::new(0.0, 0.0, 1.0)
    }

    fn get_child(&self, name: &str) -> Option<Rc<RefCell<Object>>> {
        for child in self.m_children.iter() {
            if child.borrow().m_name == name {
                return Some(child.clone());
            }
        }

        return None;
    }

    fn add_child(&mut self, child: &Rc<RefCell<Object>>) {
        self.m_children.push(child.clone());
    }

    fn remove_child(&mut self, child: &Rc<RefCell<Object>>) {
        self.m_children.retain(|c| !Rc::ptr_eq(c, child));
    }

    fn set_parent(child: &Rc<RefCell<Object>>, parent: &Rc<RefCell<Object>>) {
        if child.borrow().m_parent.upgrade().is_some() {
            if Rc::ptr_eq(&child.borrow().m_parent.upgrade().unwrap(), parent) {
                return;
            }
            child.borrow().m_parent.upgrade().unwrap().borrow_mut().remove_child(child);
        }

        parent.borrow_mut().add_child(child);
        child.borrow_mut().m_parent = Rc::downgrade(parent);
    }
}
