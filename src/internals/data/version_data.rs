use crate::{internals::{DataStruct, utils}, HpiError};

#[derive(Debug)]
pub struct VersionData {
    pub marker: u32,    // 4
    pub version: u32,   // 4
}                       // Total: 8 bytes

pub const VERSION_DATA_SIZE: usize = 8;

impl DataStruct for VersionData {
    fn read(data: &[u8], offset: usize) -> Result<Self, HpiError> {
        if offset + VERSION_DATA_SIZE > data.len() {
            return Err(HpiError::NotEnoughSpace {
                needed: VERSION_DATA_SIZE,
                available: data.len() - offset,
            });
        }

        let mut cursor = offset;
        let marker = utils::cursor_read_u32(data, &mut cursor)?;
        let version = utils::cursor_read_u32(data, &mut cursor)?;

        Ok(VersionData { marker, version })
    }

    fn cursor_read(data: &[u8], cursor: &mut usize) -> Result<Self, HpiError> {
        let result = VersionData::read(data, *cursor);
        *cursor += VERSION_DATA_SIZE;
        result
    }

    fn write(&self, out_data: &mut [u8], offset: usize) -> Result<(), HpiError> {
        if offset + VERSION_DATA_SIZE > out_data.len() {
            return Err(HpiError::NotEnoughSpace {
                needed: VERSION_DATA_SIZE,
                available: out_data.len() - offset,
            });
        }

        let mut cursor = offset;
        utils::cursor_write_u32(self.marker, out_data, &mut cursor)?;
        utils::cursor_write_u32(self.version, out_data, &mut cursor)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_correct_data() {
        let marker_bytes: [u8; 4] = [0x12, 0x34, 0x56, 0x78];
        let version_bytes: [u8; 4] = [0x02, 0x04, 0x08, 0x16];

        let mut bytes: Vec<u8> = Vec::with_capacity(VERSION_DATA_SIZE);
        bytes.extend(marker_bytes.iter());
        bytes.extend(version_bytes.iter());

        let data = VersionData::read(&bytes, 0).unwrap();

        assert_eq!(data.marker.to_le_bytes(),   marker_bytes);
        assert_eq!(data.version.to_le_bytes(),  version_bytes);
    }

    #[test]
    fn writes_correct_data() {
        let data = VersionData {
            marker: 0x12345678,
            version: 0x87654321,
        };

        let mut bytes: [u8; VERSION_DATA_SIZE] = [0; VERSION_DATA_SIZE];

        data.write(&mut bytes, 0).unwrap();

        assert_eq!(bytes[0..4],     data.marker.to_le_bytes());
        assert_eq!(bytes[4..8],     data.version.to_le_bytes());
    }
}
