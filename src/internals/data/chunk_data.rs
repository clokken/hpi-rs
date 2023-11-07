use crate::{internals::{DataStruct, utils}, HpiError};

#[derive(Debug)]
pub struct ChunkData {
    pub marker: u32,            // 4
    pub unknown1: u8,           // 1
    pub compression_method: u8, // 1
    pub is_encrypted: u8,       // 1
    pub compressed_size: u32,   // 4
    pub flat_size: u32,         // 4
    pub checksum: u32,          // 4
}                               // Total: 19 bytes

pub const CHUNK_DATA_SIZE: usize = 19;

impl DataStruct for ChunkData {
    fn read(data: &[u8], offset: usize) -> Result<Self, HpiError> {
        if offset + CHUNK_DATA_SIZE > data.len() {
            return Err(HpiError::NotEnoughSpace {
                needed: CHUNK_DATA_SIZE,
                available: data.len() - offset,
            });
        }

        let mut cursor = offset;
        let marker = utils::cursor_read_u32(data, &mut cursor)?;
        let unknown1 = data[cursor];
        cursor += 1;
        let compression_method = data[cursor];
        cursor += 1;
        let is_encrypted = data[cursor];
        cursor += 1;
        let compressed_size = utils::cursor_read_u32(data, &mut cursor)?;
        let flat_size = utils::cursor_read_u32(data, &mut cursor)?;
        let checksum = utils::cursor_read_u32(data, &mut cursor)?;

        Ok(ChunkData {
            marker,
            unknown1,
            compression_method,
            is_encrypted,
            compressed_size,
            flat_size,
            checksum,
        })
    }

    fn cursor_read(data: &[u8], cursor: &mut usize) -> Result<Self, HpiError> {
        let result = ChunkData::read(data, *cursor);
        *cursor += CHUNK_DATA_SIZE;
        result
    }

    fn write(&self, out_data: &mut [u8], offset: usize) -> Result<(), HpiError> {
        if offset + CHUNK_DATA_SIZE > out_data.len() {
            return Err(HpiError::NotEnoughSpace {
                needed: CHUNK_DATA_SIZE,
                available: out_data.len() - offset,
            });
        }

        let mut cursor = offset;
        utils::cursor_write_u32(self.marker, out_data, &mut cursor)?;
        out_data[cursor] = self.unknown1;
        cursor += 1;
        out_data[cursor] = self.compression_method;
        cursor += 1;
        out_data[cursor] = self.is_encrypted;
        cursor += 1;
        utils::cursor_write_u32(self.compressed_size, out_data, &mut cursor)?;
        utils::cursor_write_u32(self.flat_size, out_data, &mut cursor)?;
        utils::cursor_write_u32(self.checksum, out_data, &mut cursor)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_correct_data() {
        let marker_bytes: [u8; 4] = [0x12, 0x34, 0x56, 0x78];
        let unknown1_byte: u8 = 0x02;
        let compression_method_byte: u8 = 0x03;
        let is_encrypted_byte: u8 = 0x04;
        let compressed_size_bytes: [u8; 4] = [0x87, 0x65, 0x43, 0x21];
        let flat_size_bytes: [u8; 4] = [0x11, 0x22, 0x33, 0x44];
        let checksum_bytes: [u8; 4] = [0x99, 0x88, 0x77, 0x66];

        let mut bytes: Vec<u8> = Vec::with_capacity(CHUNK_DATA_SIZE);
        bytes.extend(marker_bytes.iter());
        bytes.push(unknown1_byte);
        bytes.push(compression_method_byte);
        bytes.push(is_encrypted_byte);
        bytes.extend(compressed_size_bytes.iter());
        bytes.extend(flat_size_bytes.iter());
        bytes.extend(checksum_bytes.iter());

        let data = ChunkData::read(&bytes, 0).unwrap();

        assert_eq!(data.marker.to_le_bytes(),           marker_bytes);
        assert_eq!(data.unknown1,                       unknown1_byte);
        assert_eq!(data.compression_method,             compression_method_byte);
        assert_eq!(data.is_encrypted,                   is_encrypted_byte);
        assert_eq!(data.compressed_size.to_le_bytes(),  compressed_size_bytes);
        assert_eq!(data.flat_size.to_le_bytes(),        flat_size_bytes);
        assert_eq!(data.checksum.to_le_bytes(),         checksum_bytes);
    }

    #[test]
    fn writes_correct_data() {
        let data = ChunkData {
            marker:             0x12345678,
            unknown1:           0x12,
            compression_method: 0x34,
            is_encrypted:       0x56,
            compressed_size:    0x87654321,
            flat_size:          0x11223344,
            checksum:           0x99887766,
        };

        let mut bytes: [u8; CHUNK_DATA_SIZE] = [0; CHUNK_DATA_SIZE];

        data.write(&mut bytes, 0).unwrap();

        assert_eq!(bytes[0..4],     data.marker.to_le_bytes());
        assert_eq!(bytes[4],        data.unknown1);
        assert_eq!(bytes[5],        data.compression_method);
        assert_eq!(bytes[6],        data.is_encrypted);
        assert_eq!(bytes[7..11],    data.compressed_size.to_le_bytes());
        assert_eq!(bytes[11..15],   data.flat_size.to_le_bytes());
        assert_eq!(bytes[15..19],   data.checksum.to_le_bytes());
    }
}
