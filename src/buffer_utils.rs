use std::io::Read;

use flate2::read::ZlibDecoder;

use crate::{internals::{ChunkData, CHUNK_DATA_SIZE, DataStruct}, consts, HpiError};

pub fn read_chunk_buffer(data: &[u8]) -> Result<Vec<u8>, HpiError> {
    let chunk_data = ChunkData::read(data, 0)?;

    if chunk_data.marker == consts::SQSH_MARKER {
        // is compressed?
        let compressed_data = &data[CHUNK_DATA_SIZE..];
        let mut decompressed_block = Vec::with_capacity(chunk_data.flat_size as usize);

        decompress_buffer(compressed_data, &chunk_data, &mut decompressed_block)?;

        // TODO compare decompressed_block.len() against chunk_data.flat_size?

        return Ok(decompressed_block);
    }

    println!("Is NOT compressed!");

    Ok(data.to_vec())
}

pub fn decompress_buffer<'a, 'b>(
    data: &'a [u8],
    chunk_data: &'b ChunkData,
    mut output: &mut Vec<u8>,
) -> Result<usize, HpiError> {
    let mut checksum: u32 = 0;
    let mut decrypted_data: Vec<u8> = vec![];

    if chunk_data.is_encrypted == 1 {
        decrypted_data = data.to_vec();
    }

    for x in 0..chunk_data.compressed_size {
        let index = x as usize;
        checksum += data[index] as u32;

        if chunk_data.is_encrypted == 1 {
            let x = x as u8;
            // decrypted_data[index] = (decrypted_data[index] - x) ^ x;
            decrypted_data[index] = decrypted_data[index].wrapping_sub(x);
            decrypted_data[index] = decrypted_data[index] ^ x;
        }
    }

    if chunk_data.checksum != checksum {
        return Err(HpiError::ChecksumMismatch {
            expected: chunk_data.checksum,
            got: checksum,
        });
    }

    if chunk_data.compression_method == 1 {
        return Err(HpiError::Other("LZ77 compression not yet supported :("));
    }

    if chunk_data.compression_method != 2 {
        return Err(HpiError::UnknownCompMethod(chunk_data.compression_method));
    }

    let mut decoder = ZlibDecoder::new(if chunk_data.is_encrypted == 1 {
        &decrypted_data
    } else {
        data
    });

    let written = decoder.read_to_end(&mut output).map_err(|err| {
        eprintln!("zlib decode error: {}", err);
        HpiError::Other("Failed to decode with zlib")
    })?;

    Ok(written)
}

pub fn calc_checksum(prev_chucksum: u32, buffer: &[u8]) -> u32 {
    let mut bytes = [
        (prev_chucksum & 0xFF000000) >> 24,
        (prev_chucksum & 0x00FF0000) >> 16,
        (prev_chucksum & 0x0000FF00) >> 8,
        (prev_chucksum & 0x000000FF),
    ];

    for count in 0..buffer.len() as u32 {
        let c = buffer[count as usize];

        bytes[0] += c as u32;
        bytes[1] ^= c as u32;
        bytes[2] += (c as u32) ^ (count & 0x000000FF);
        bytes[3] ^= (c as u32) ^ (count & 0x000000FF);
    }

    return (bytes[3] << 24) | (bytes[2] << 16) | (bytes[1] << 8) | bytes[0];
}
