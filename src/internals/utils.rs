use crate::internals::DataStructError;

pub fn read_u32(data: &[u8], offset: usize) -> Result<u32, DataStructError> {
    if offset + 4 > data.len() {
        return Err(DataStructError::NotEnoughSize { needed: 4, available: data.len() - offset });
    }

    Ok(u32::from_le_bytes([data[offset + 0], data[offset + 1], data[offset + 2], data[offset + 3]]))
}

pub fn cursor_read_u32(data: &[u8], cursor: &mut usize) -> Result<u32, DataStructError> {
    let result = read_u32(data, *cursor)?;
    *cursor += 4;
    Ok(result)
}

pub fn write_u32(input: u32, out_data: &mut [u8], offset: usize) -> Result<(), DataStructError> {
    if offset + 4 > out_data.len() {
        return Err(DataStructError::NotEnoughSize { needed: 4, available: out_data.len() - offset });
    }

    out_data[offset..offset + 4].copy_from_slice(&input.to_le_bytes());

    Ok(())
}

pub fn cursor_write_u32(input: u32, out_data: &mut [u8], cursor: &mut usize)
    -> Result<(), DataStructError>
{
    write_u32(input, out_data, *cursor)?;
    *cursor += 4;
    Ok(())
}
