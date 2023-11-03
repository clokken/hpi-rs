pub struct DataMap {
  sections: Vec<DataSection>,
}

pub struct DataSection {
  pub offset: usize,
  pub length: usize,
  pub name: String,
}

impl DataMap {
  pub fn iter<'a>(&'a self) -> core::slice::Iter<'a, DataSection> {
    self.sections.iter()
  }
}
