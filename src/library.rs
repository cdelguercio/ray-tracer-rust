use std::collections::HashMap;

pub struct Library<T> {
    m_contents: Vec<T>,
    m_index_map: HashMap<String, usize>,
}

impl<T> Library<T> {
    pub fn new() -> Library<T> {
        Library {
            m_contents: Vec::new(),
            m_index_map: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: &str, item: T) {
        if self.m_index_map.contains_key(name) {
            panic!("Cannot add item to library, name {} already exists", name);
        }

        self.m_contents.push(item);
        self.m_index_map.insert(name.to_string(), self.m_contents.len() - 1);
    }

    fn index_for_name(&self, name: &str) -> usize {
        if !self.m_index_map.contains_key(name) {
            panic!("Cannot get index for name {}, name does not exist", name);
        }

        *self.m_index_map.get(name).unwrap()
    }

    fn fetch(&self, name: &str) -> &T {
        &self.m_contents[self.index_for_name(name)]
    }

    fn fetch_mut(&mut self, name: &str) -> &mut T {
        let index = self.index_for_name(name);
        &mut self.m_contents[index]
    }

    pub fn fetch_by_index(&self, index: usize) -> &T {
        if index >= self.m_contents.len() {
            panic!("Cannot get item at index {}, library only has {} items", index, self.m_contents.len());
        }

        &self.m_contents[index]
    }

    fn len(&self) -> usize {
        self.m_contents.len()
    }
}
