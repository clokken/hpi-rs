use crate::{internals::EntryData, HpiDirectory};

pub enum HpiItem {
    Directory(HpiDirectory),
    Entry(HpiEntry),
}
pub struct HpiEntry {
    // TODO remove pub from all these fields
    pub name: String,
    pub data: EntryData,
    pub path: String, // path of parent, not self
}
