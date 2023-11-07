use crate::{internals::{DataStruct, utils}, HpiError};

#[derive(Debug)]
pub struct DirectoryData {
    pub name_ptr: u32,          // 4
    pub first_subdir_ptr: u32,  // 4
    pub subdir_count: u32,      // 4
    pub first_file_ptr: u32,    // 4
    pub file_count: u32,        // 4
}                               // Total: 20 bytes

pub const DIRECTORY_DATA_SIZE: usize = 20;

impl DataStruct for DirectoryData {
    fn read(data: &[u8], offset: usize) -> Result<Self, HpiError> {
        if offset + DIRECTORY_DATA_SIZE > data.len() {
            return Err(HpiError::NotEnoughSpace {
                needed: DIRECTORY_DATA_SIZE,
                available: data.len() - offset,
            });
        }

        let mut cursor = offset;

        Ok(DirectoryData {
            name_ptr: utils::cursor_read_u32(data, &mut cursor)?,
            first_subdir_ptr: utils::cursor_read_u32(data, &mut cursor)?,
            subdir_count: utils::cursor_read_u32(data, &mut cursor)?,
            first_file_ptr: utils::cursor_read_u32(data, &mut cursor)?,
            file_count: utils::cursor_read_u32(data, &mut cursor)?,
        })
    }

    fn cursor_read(data: &[u8], cursor: &mut usize) -> Result<Self, HpiError> {
        let result = DirectoryData::read(data, *cursor);
        *cursor += DIRECTORY_DATA_SIZE;
        result
    }

    fn write(&self, out_data: &mut [u8], offset: usize) -> Result<(), HpiError> {
        if offset + DIRECTORY_DATA_SIZE > out_data.len() {
            return Err(HpiError::NotEnoughSpace {
                needed: DIRECTORY_DATA_SIZE,
                available: out_data.len() - offset,
            });
        }

        let mut cursor = offset;
        utils::cursor_write_u32(self.name_ptr, out_data, &mut cursor)?;
        utils::cursor_write_u32(self.first_subdir_ptr, out_data, &mut cursor)?;
        utils::cursor_write_u32(self.subdir_count, out_data, &mut cursor)?;
        utils::cursor_write_u32(self.first_file_ptr, out_data, &mut cursor)?;
        utils::cursor_write_u32(self.file_count, out_data, &mut cursor)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_correct_data() {
        let name_ptr_bytes: [u8; 4] = [0x12, 0x34, 0x56, 0x78];
        let first_subdir_ptr_bytes: [u8; 4] = [0x02, 0x04, 0x08, 0x16];
        let subdir_count_bytes: [u8; 4] = [0x87, 0x65, 0x43, 0x21];
        let first_file_ptr_bytes: [u8; 4] = [0x11, 0x22, 0x33, 0x44];
        let file_count_bytes: [u8; 4] = [0x99, 0x88, 0x77, 0x66];

        let mut bytes: Vec<u8> = Vec::with_capacity(DIRECTORY_DATA_SIZE);
        bytes.extend(name_ptr_bytes.iter());
        bytes.extend(first_subdir_ptr_bytes.iter());
        bytes.extend(subdir_count_bytes.iter());
        bytes.extend(first_file_ptr_bytes.iter());
        bytes.extend(file_count_bytes.iter());

        let data = DirectoryData::read(&bytes, 0).unwrap();

        assert_eq!(data.name_ptr.to_le_bytes(),         name_ptr_bytes);
        assert_eq!(data.first_subdir_ptr.to_le_bytes(), first_subdir_ptr_bytes);
        assert_eq!(data.subdir_count.to_le_bytes(),     subdir_count_bytes);
        assert_eq!(data.first_file_ptr.to_le_bytes(),   first_file_ptr_bytes);
        assert_eq!(data.file_count.to_le_bytes(),       file_count_bytes);
    }

    #[test]
    fn writes_correct_data() {
        let data = DirectoryData {
            name_ptr: 0x12345678,
            first_subdir_ptr: 0x87654321,
            subdir_count: 0x01020408,
            first_file_ptr: 0xABCDEF12,
            file_count: 0x1337C0DE,
        };

        let mut bytes: [u8; DIRECTORY_DATA_SIZE] = [0; DIRECTORY_DATA_SIZE];

        data.write(&mut bytes, 0).unwrap();

        assert_eq!(bytes[0..4],     data.name_ptr.to_le_bytes());
        assert_eq!(bytes[4..8],     data.first_subdir_ptr.to_le_bytes());
        assert_eq!(bytes[8..12],    data.subdir_count.to_le_bytes());
        assert_eq!(bytes[12..16],   data.first_file_ptr.to_le_bytes());
        assert_eq!(bytes[16..20],   data.file_count.to_le_bytes());
    }
}
