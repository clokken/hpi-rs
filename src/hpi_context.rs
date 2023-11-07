use crate::{HpiDirectory, internals::{DirectoryData, DataStruct, DIRECTORY_DATA_SIZE, EntryData, ENTRY_DATA_SIZE, ChunkData}, hpi_error::HpiError, HpiItem, HpiEntry, buffer_utils};

pub struct HpiContext<'a> {
    full_data: &'a [u8],
    directories_buffer: Vec<u8>,
    names_buffer: Vec<u8>,
    pub root: HpiDirectory,
}

impl<'a> HpiContext<'a> {
    pub fn build(full_data: &'a [u8], directories_buffer: Vec<u8>, names_buffer: Vec<u8>)
        -> Result<Self, HpiError>
    {
        let dir_data = DirectoryData::read(&directories_buffer, 0)?;

        let root_children = Self::build_dir_tree(
            &directories_buffer,
            &names_buffer,
            &dir_data,
            "",
        )?;

        let root = HpiDirectory {
            name: String::from(""),
            path: String::from("/"),
            data: dir_data,
            children: root_children,
        };

        Ok(Self {
            full_data,
            directories_buffer,
            names_buffer,
            root,
        })
    }

    fn build_dir_tree(
        directories_buffer: &Vec<u8>,
        names_buffer: &Vec<u8>,
        dir_data: &DirectoryData,
        path: &str,
    ) -> Result<Vec<HpiItem>, HpiError> {
        let mut children = Vec::with_capacity(
            dir_data.subdir_count as usize +
            dir_data.file_count as usize
        );

        for index in 0..dir_data.subdir_count as usize {
            let subdir_offset = dir_data.first_subdir_ptr as usize + (index * DIRECTORY_DATA_SIZE);
            let subdir_data = DirectoryData::read(&directories_buffer, subdir_offset)?;
            let subdir_name = Self::read_name(&names_buffer, subdir_data.name_ptr)?;

            let subdir_children = Self::build_dir_tree(
                directories_buffer,
                names_buffer,
                &subdir_data,
                &format!("{}/{}", path, subdir_name),
            )?;

            let subdir = HpiDirectory {
                name: subdir_name,
                data: subdir_data,
                path: path.to_string(),
                children: subdir_children,
            };

            // nextItem.setStructOrigin(nextOffset);
            // nextItem.setParent(parent);

            children.push(HpiItem::Directory(subdir));
        }

        for index in 0..dir_data.file_count as usize {
            let entry_offset = dir_data.first_file_ptr as usize + (index * ENTRY_DATA_SIZE);
            let entry_data = EntryData::read(&directories_buffer, entry_offset)?;
            let entry_name = Self::read_name(&names_buffer, entry_data.name_ptr)?;

            // nextItem.setStructOrigin(nextOffset);
            // nextItem.setParent(parent);

            let next_entry = HpiEntry {
                name: entry_name,
                data: entry_data,
                path: String::from(path),
            };

            children.push(HpiItem::Entry(next_entry));
        }

        Ok(children)
    }

    fn read_name(names_buffer: &[u8], name_ptr: u32) -> Result<String, HpiError> {
        let offset = name_ptr as usize;
        let mut length = 0;

        for index in 0.. {
            let next = names_buffer[offset + index];

            if next == 0 {
                length = index;
                break;
            }

            if length > 255 { // 255 is completely arbitrary here :)
                return Err(HpiError::InvalidNamePointer { offset });
            }
        }

        let name_buf = &names_buffer[offset..offset + length];
        let name = std::str::from_utf8(name_buf).expect("TODO handle this");
        Ok(String::from(name))
    }

    pub fn extract_file(&self, entry: &HpiEntry) -> Result<Vec<u8>, HpiError> {
        if entry.data.compressed_size == 0 {
            let offset = entry.data.data_start_ptr as usize;
            let length = entry.data.flat_size as usize;
            let slice = &self.full_data[offset..offset + length];
            return Ok(slice.to_vec());
        }

        let flat_size = entry.data.flat_size as usize;
        let mut result = Vec::with_capacity(flat_size);
        let mut offset = entry.data.data_start_ptr as usize;
        let mut checksum = 0;

        while result.len() < flat_size {
            let chunk_data = ChunkData::cursor_read(self.full_data, &mut offset)?;
            let sqsh_buffer = &self.full_data[offset..];

            let next_size = buffer_utils::decompress_buffer(sqsh_buffer, &chunk_data, &mut result)?;

            // TODO:
            /*if (flatBuffer.length !== chunk.getField('flatSize'))
                throw new Error(`Error decompressing chunk. Expected: ${chunk.getField('flatSize')} bytes; Got: ${flatBuffer.length} bytes`);*/

            // checksum = BufferUtils.calcChecksum(checksum, buffer, flatBuffer.length);
            checksum = buffer_utils::calc_checksum(checksum, &result);
        }

        if checksum != entry.data.checksum {
            // return Err(HpiError::ChecksumMismatch { expected: entry.data.checksum, got: checksum });
        }

        Ok(result)
    }
}
