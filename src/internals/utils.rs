use crate::HpiError;

pub fn read_u32(data: &[u8], offset: usize) -> Result<u32, HpiError> {
    if offset + 4 > data.len() {
        return Err(HpiError::NotEnoughSpace { needed: 4, available: data.len() - offset });
    }

    Ok(u32::from_le_bytes([data[offset + 0], data[offset + 1], data[offset + 2], data[offset + 3]]))
}

pub fn cursor_read_u32(data: &[u8], cursor: &mut usize) -> Result<u32, HpiError> {
    let result = read_u32(data, *cursor)?;
    *cursor += 4;
    Ok(result)
}

pub fn write_u32(input: u32, out_data: &mut [u8], offset: usize) -> Result<(), HpiError> {
    if offset + 4 > out_data.len() {
        return Err(HpiError::NotEnoughSpace { needed: 4, available: out_data.len() - offset });
    }

    out_data[offset..offset + 4].copy_from_slice(&input.to_le_bytes());

    Ok(())
}

pub fn cursor_write_u32(input: u32, out_data: &mut [u8], cursor: &mut usize)
    -> Result<(), HpiError>
{
    write_u32(input, out_data, *cursor)?;
    *cursor += 4;
    Ok(())
}

pub fn try_slice(data: &[u8], offset: usize, length: usize) -> Result<&[u8], HpiError> {
    if offset + length > data.len() {
        return Err(HpiError::NotEnoughSpace {
            needed: length,
            available: data.len() - offset,
        });
    }

    Ok(&data[offset..offset + length])
}
