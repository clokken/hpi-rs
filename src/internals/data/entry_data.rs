use crate::internals::{DataStruct, DataStructError, utils};

#[derive(Debug)]
pub struct EntryData {
    pub name_ptr: u32,          // 4
    pub data_start_ptr: u32,    // 4
    pub flat_size: u32,         // 4
    pub compressed_size: u32,   // 4
    pub date: u32,              // 4
    pub checksum: u32,          // 4
}                           // Total: 24 bytes

pub const ENTRY_DATA_SIZE: usize = 24;

impl DataStruct for EntryData {
    fn read(data: &[u8], offset: usize) -> Result<Self, DataStructError> {
        if offset + ENTRY_DATA_SIZE > data.len() {
            return Err(DataStructError::NotEnoughSize {
                needed: ENTRY_DATA_SIZE,
                available: data.len() - offset,
            });
        }

        let mut cursor = offset;

        Ok(EntryData {
            name_ptr: utils::cursor_read_u32(data, &mut cursor)?,
            data_start_ptr: utils::cursor_read_u32(data, &mut cursor)?,
            flat_size: utils::cursor_read_u32(data, &mut cursor)?,
            compressed_size: utils::cursor_read_u32(data, &mut cursor)?,
            date: utils::cursor_read_u32(data, &mut cursor)?,
            checksum: utils::cursor_read_u32(data, &mut cursor)?,
        })
    }

    fn write(&self, out_data: &mut [u8], offset: usize) -> Result<(), DataStructError> {
        if offset + ENTRY_DATA_SIZE > out_data.len() {
            return Err(DataStructError::NotEnoughSize {
                needed: ENTRY_DATA_SIZE,
                available: out_data.len() - offset,
            });
        }

        let mut cursor = offset;
        utils::cursor_write_u32(self.name_ptr, out_data, &mut cursor)?;
        utils::cursor_write_u32(self.data_start_ptr, out_data, &mut cursor)?;
        utils::cursor_write_u32(self.flat_size, out_data, &mut cursor)?;
        utils::cursor_write_u32(self.compressed_size, out_data, &mut cursor)?;
        utils::cursor_write_u32(self.date, out_data, &mut cursor)?;
        utils::cursor_write_u32(self.checksum, out_data, &mut cursor)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_correct_data() {
        let name_ptr_bytes: [u8; 4] = [0x12, 0x34, 0x56, 0x78];
        let data_start_ptr_bytes: [u8; 4] = [0x02, 0x04, 0x08, 0x16];
        let flat_size_bytes: [u8; 4] = [0x11, 0x22, 0x33, 0x44];
        let compressed_size_bytes: [u8; 4] = [0x99, 0x88, 0x77, 0x66];
        let date_bytes: [u8; 4] = [0x87, 0x65, 0x43, 0x21];
        let checksum_bytes: [u8; 4] = [0xAB, 0xCD, 0xEF, 0x12];

        let mut bytes: Vec<u8> = Vec::with_capacity(ENTRY_DATA_SIZE);
        bytes.extend(name_ptr_bytes.iter());
        bytes.extend(data_start_ptr_bytes.iter());
        bytes.extend(flat_size_bytes.iter());
        bytes.extend(compressed_size_bytes.iter());
        bytes.extend(date_bytes.iter());
        bytes.extend(checksum_bytes.iter());

        let data = EntryData::read(&bytes, 0).unwrap();

        assert_eq!(data.name_ptr.to_le_bytes(),         name_ptr_bytes);
        assert_eq!(data.data_start_ptr.to_le_bytes(),   data_start_ptr_bytes);
        assert_eq!(data.flat_size.to_le_bytes(),        flat_size_bytes);
        assert_eq!(data.compressed_size.to_le_bytes(),  compressed_size_bytes);
        assert_eq!(data.date.to_le_bytes(),             date_bytes);
        assert_eq!(data.checksum.to_le_bytes(),         checksum_bytes);
    }

    #[test]
    fn writes_correct_data() {
        let data = EntryData {
            name_ptr: 0x12345678,
            data_start_ptr: 0x87654321,
            flat_size: 0x01020408,
            compressed_size: 0xABCDEF12,
            date: 0x1337C0DE,
            checksum: 13374269,
        };

        let mut bytes: [u8; ENTRY_DATA_SIZE] = [0; ENTRY_DATA_SIZE];

        data.write(&mut bytes, 0).unwrap();

        assert_eq!(bytes[0..4],     data.name_ptr.to_le_bytes());
        assert_eq!(bytes[4..8],     data.data_start_ptr.to_le_bytes());
        assert_eq!(bytes[8..12],    data.flat_size.to_le_bytes());
        assert_eq!(bytes[12..16],   data.compressed_size.to_le_bytes());
        assert_eq!(bytes[16..20],   data.date.to_le_bytes());
        assert_eq!(bytes[20..24],   data.checksum.to_le_bytes());
    }
}
