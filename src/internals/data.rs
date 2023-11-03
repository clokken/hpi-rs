use packed_struct::prelude::*;

#[derive(Debug, PackedStruct)]
#[packed_struct(endian="lsb")]
pub struct VersionData {
  pub marker: u32,
  pub version: u32,
}

#[derive(Debug, PackedStruct)]
#[packed_struct(endian="lsb")]
pub struct ChunkData {
  pub marker: u32,
  pub unknown1: u8,
  pub compression_method: u8,
  pub is_encrypted: u8,
  pub compressed_size: u32,
  pub flat_size: u32,
  pub checksum: u32,
}

#[derive(Debug, PackedStruct)]
#[packed_struct(endian="lsb")]
pub struct HeaderData {
  dir_block_ptr: u32,
  dir_block_len: u32,
  names_block_ptr: u32,
  names_block_len: u32,
  data: u32,
  last78: u32,
}

#[derive(Debug, PackedStruct)]
#[packed_struct(endian="lsb")]
pub struct DirectoryData {
  name_ptr: u32,
  first_subdir_ptr: u32,
  subdir_count: u32,
  first_file_ptr: u32,
  file_count: u32,
}

#[derive(Debug, PackedStruct)]
#[packed_struct(endian="lsb")]
pub struct EntryData {
  name_ptr: u32,
  data_start_ptr: u32,
  flat_size: u32,
  compressed_size: u32,
  date: u32,
  checksum: u32,
}
