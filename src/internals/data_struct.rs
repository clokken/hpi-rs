pub trait DataStruct: Sized {
    fn read(data: &[u8], offset: usize) -> Result<Self, DataStructError>;
    fn write(&self, out_data: &mut [u8], offset: usize) -> Result<(), DataStructError>;
}

#[derive(Debug)]
pub enum DataStructError {
    NotEnoughSize {
        needed: usize,
        available: usize,
    },
    // OutOfBounds, // removed because boundary checks should be the caller's responsibility
    Other(String),
}
