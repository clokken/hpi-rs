pub struct DataMap {
    sections: Vec<DataSection>,
}

pub struct DataSection {
    pub offset: usize,
    pub length: usize,
    pub name: &'static str,
}

impl DataMap {
    pub fn new() -> Self {
        DataMap { sections: Vec::new() }
    }

    pub fn add(&mut self, offset: usize, length: usize, name: &'static str) {
        self.sections.push(DataSection{ offset, length, name });
    }

    pub fn iter<'a>(&'a self) -> core::slice::Iter<'a, DataSection> {
        self.sections.iter()
    }
}
