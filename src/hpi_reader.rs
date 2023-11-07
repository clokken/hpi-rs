use std::fmt;

use crate::hpi_error::HpiError;
use crate::internals::{VersionData, DataStruct, HeaderData, VERSION_DATA_SIZE, HEADER_DATA_SIZE, utils};
use crate::{consts, DataMap, HpiContext, buffer_utils};

pub struct HpiReader;

impl HpiReader {
    pub fn read(data: &[u8]) -> Result<HpiContext, HpiError> {
        let mut data_map = DataMap::new();
        let mut cursor = 0;

        data_map.add(cursor, VERSION_DATA_SIZE, "version");
        let version_data = VersionData::cursor_read(data, &mut cursor)?;

        if version_data.version != consts::HPI_VERSION {
            return Err(HpiError::UnknownVersion(version_data.version));
        }

        if version_data.marker != consts::HPI_MARKER {
            return Err(HpiError::UnknownMarker(version_data.marker));
        }

        data_map.add(cursor, HEADER_DATA_SIZE, "header");
        let header_data = HeaderData::cursor_read(data, &mut cursor)?;

        // TODO data_map.add
        let directory_buffer = utils::try_slice(
            data,
            header_data.dir_block_ptr.try_into().unwrap(),
            header_data.dir_block_len.try_into().unwrap(),
        )?;

        // TODO data_map.add
        let directory_block = buffer_utils::read_chunk_buffer(directory_buffer)?;

        let names_buffer = utils::try_slice(
            data,
            header_data.names_block_ptr.try_into().unwrap(),
            header_data.names_block_len.try_into().unwrap(),
        )?;

        let names_block = buffer_utils::read_chunk_buffer(names_buffer)?;

        let context = HpiContext::build(data, directory_block, names_block)?;

        Ok(context)
    }
}

impl fmt::Display for HpiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "There is an error: {}", self.0)
        // TODO
        write!(f, "{}", self)
    }
}
