#[derive(Debug)]
pub enum HpiError {
    // TODO replace almost all of these with a generic "InvalidInputData" ?
    NotEnoughSpace {
        needed: usize,
        available: usize,
    },
    ChecksumMismatch {
        expected: u32,
        got: u32,
    },
    UnknownMarker(u32),
    UnknownVersion(u32),
    UnknownCompMethod(u8),
    InvalidNamePointer {
        offset: usize,
    },
    Other(&'static str),
}
