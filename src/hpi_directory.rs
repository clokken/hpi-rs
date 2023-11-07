use crate::{HpiItem, internals::DirectoryData};

pub struct HpiDirectory {
    // TODO remove pub from all these fields
    pub children: Vec<HpiItem>,
    pub name: String,
    pub data: DirectoryData,
    pub path: String, // path of parent, not self
}

impl HpiDirectory {
    pub fn debug_print(&self) {
        if self.name != "" {
            println!("{}/{}", self.path, self.name);
        } else {
            println!("/");
        }

        for child in &self.children {
            if let HpiItem::Entry(entry) = child {
                println!("{}/{}/{}", self.path, self.name, entry.name);
            }
        }

        for child in &self.children {
            if let HpiItem::Directory(dir) = child {
                dir.debug_print();
            }
        }
    }

    pub fn iter(&self, recursive: bool) -> HpiDirectoryIterator {
        HpiDirectoryIterator::new(&self.children, recursive)
    }
}

pub struct HpiDirectoryIterator<'a> {
    children: &'a Vec<HpiItem>,
    recursive: bool,
    item_cursor: usize,
    sub_dir_cursor: usize,
    sub_dir_iter: Option<Box<HpiDirectoryIterator<'a>>>,
}

impl<'a> HpiDirectoryIterator<'a> {
    fn new(children: &'a Vec<HpiItem>, recursive: bool) -> Self {
        HpiDirectoryIterator {
            children,
            recursive,
            item_cursor: 0,
            sub_dir_cursor: 0,
            sub_dir_iter: None,
        }
    }
}

impl<'a> Iterator for HpiDirectoryIterator<'a> {
    type Item = &'a HpiItem;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(sub_dir_iter) = self.sub_dir_iter.as_mut() {
            match sub_dir_iter.next() {
                Some(next) => {
                    return Some(next);
                },
                None => {
                    self.sub_dir_iter = None;
                },
            }
        }

        if self.item_cursor < self.children.len() {
            let next_item = &self.children[self.item_cursor];
            self.item_cursor += 1;
            return Some(next_item);
        }

        if !self.recursive {
            return None;
        }

        while self.sub_dir_cursor < self.children.len() {
            let next_item = &self.children[self.sub_dir_cursor];
            self.sub_dir_cursor += 1;

            if let HpiItem::Directory(sub_dir) = next_item {
                let mut sub_iter = HpiDirectoryIterator::new(&sub_dir.children, true);

                let sub_dir_next = sub_iter.next();
                self.sub_dir_iter = Some(Box::new(sub_iter));

                if sub_dir_next.is_some() {
                    return sub_dir_next;
                }

                break;
            }
        }

        None
    }
}
