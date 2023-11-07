use crate::HpiError;

pub trait DataStruct: Sized {
    fn read(data: &[u8], offset: usize) -> Result<Self, HpiError>;
    fn cursor_read(data: &[u8], cursor: &mut usize) -> Result<Self, HpiError>;
    fn write(&self, out_data: &mut [u8], offset: usize) -> Result<(), HpiError>;
}
