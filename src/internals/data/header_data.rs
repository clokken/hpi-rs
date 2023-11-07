use crate::{internals::{DataStruct, utils}, HpiError};

#[derive(Debug)]
pub struct HeaderData {
    pub dir_block_ptr: u32,     // 4
    pub dir_block_len: u32,     // 4
    pub names_block_ptr: u32,   // 4
    pub names_block_len: u32,   // 4
    pub data_ptr: u32,          // 4 // TODO investigate the meaning of this field
    pub last78: u32,            // 4
}                               // Total: 24 bytes

pub const HEADER_DATA_SIZE: usize = 24;

impl DataStruct for HeaderData {
    fn read(data: &[u8], offset: usize) -> Result<Self, HpiError> {
        if offset + HEADER_DATA_SIZE > data.len() {
            return Err(HpiError::NotEnoughSpace {
                needed: HEADER_DATA_SIZE,
                available: data.len() - offset,
            });
        }

        let mut cursor = offset;

        Ok(HeaderData {
            dir_block_ptr: utils::cursor_read_u32(data, &mut cursor)?,
            dir_block_len: utils::cursor_read_u32(data, &mut cursor)?,
            names_block_ptr: utils::cursor_read_u32(data, &mut cursor)?,
            names_block_len: utils::cursor_read_u32(data, &mut cursor)?,
            data_ptr: utils::cursor_read_u32(data, &mut cursor)?,
            last78: utils::cursor_read_u32(data, &mut cursor)?,
        })
    }

    fn cursor_read(data: &[u8], cursor: &mut usize) -> Result<Self, HpiError> {
        let result = HeaderData::read(data, *cursor);
        *cursor += HEADER_DATA_SIZE;
        result
    }

    fn write(&self, out_data: &mut [u8], offset: usize) -> Result<(), HpiError> {
        if offset + HEADER_DATA_SIZE > out_data.len() {
            return Err(HpiError::NotEnoughSpace {
                needed: HEADER_DATA_SIZE,
                available: out_data.len() - offset,
            });
        }

        let mut cursor = offset;
        utils::cursor_write_u32(self.dir_block_ptr, out_data, &mut cursor)?;
        utils::cursor_write_u32(self.dir_block_len, out_data, &mut cursor)?;
        utils::cursor_write_u32(self.names_block_ptr, out_data, &mut cursor)?;
        utils::cursor_write_u32(self.names_block_len, out_data, &mut cursor)?;
        utils::cursor_write_u32(self.data_ptr, out_data, &mut cursor)?;
        utils::cursor_write_u32(self.last78, out_data, &mut cursor)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_correct_data() {
        let dir_block_ptr_bytes: [u8; 4] = [0x12, 0x34, 0x56, 0x78];
        let dir_block_len_bytes: [u8; 4] = [0x02, 0x04, 0x08, 0x16];
        let names_block_ptr_bytes: [u8; 4] = [0x11, 0x22, 0x33, 0x44];
        let names_block_len_bytes: [u8; 4] = [0x99, 0x88, 0x77, 0x66];
        let data_ptr_bytes: [u8; 4] = [0x87, 0x65, 0x43, 0x21];
        let last78_bytes: [u8; 4] = [0xAB, 0xCD, 0xEF, 0x12];

        let mut bytes: Vec<u8> = Vec::with_capacity(HEADER_DATA_SIZE);
        bytes.extend(dir_block_ptr_bytes.iter());
        bytes.extend(dir_block_len_bytes.iter());
        bytes.extend(names_block_ptr_bytes.iter());
        bytes.extend(names_block_len_bytes.iter());
        bytes.extend(data_ptr_bytes.iter());
        bytes.extend(last78_bytes.iter());

        let data = HeaderData::read(&bytes, 0).unwrap();

        assert_eq!(data.dir_block_ptr.to_le_bytes(),    dir_block_ptr_bytes);
        assert_eq!(data.dir_block_len.to_le_bytes(),    dir_block_len_bytes);
        assert_eq!(data.names_block_ptr.to_le_bytes(),  names_block_ptr_bytes);
        assert_eq!(data.names_block_len.to_le_bytes(),  names_block_len_bytes);
        assert_eq!(data.data_ptr.to_le_bytes(),         data_ptr_bytes);
        assert_eq!(data.last78.to_le_bytes(),           last78_bytes);
    }

    #[test]
    fn writes_correct_data() {
        let data = HeaderData {
            dir_block_ptr: 0x12345678,
            dir_block_len: 0x87654321,
            names_block_ptr: 0x01020408,
            names_block_len: 0xABCDEF12,
            data_ptr: 0x1337C0DE,
            last78: 0x13374269,
        };

        let mut bytes: [u8; HEADER_DATA_SIZE] = [0; HEADER_DATA_SIZE];

        data.write(&mut bytes, 0).unwrap();

        assert_eq!(bytes[0..4],     data.dir_block_ptr.to_le_bytes());
        assert_eq!(bytes[4..8],     data.dir_block_len.to_le_bytes());
        assert_eq!(bytes[8..12],    data.names_block_ptr.to_le_bytes());
        assert_eq!(bytes[12..16],   data.names_block_len.to_le_bytes());
        assert_eq!(bytes[16..20],   data.data_ptr.to_le_bytes());
        assert_eq!(bytes[20..24],   data.last78.to_le_bytes());
    }
}
