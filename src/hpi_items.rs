pub enum HpiItem {
  Directory(HpiDirectory),
  File(HpiFile),
}

pub struct HpiDirectory {
  pub children: i32, // TODO
}

pub struct HpiFile {
  pub size: usize,
}

pub fn foo(item: &HpiItem) {
  match item {
    HpiItem::Directory(dir) => {
      println!("{}", dir.children);
    },
    HpiItem::File(file) => {
      println!("{}", file.size);
    }
  }
}
