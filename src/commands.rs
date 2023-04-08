use crate::args::*;
use crate::chunk_type::ChunkType;
use crate::Error;
use crate::Result;

pub fn encode(
    file_to_encode: std::path::PathBuf,
    chunk_type: ChunkType,
    message: &str,
) -> Result<std::path::PathBuf> {
    todo!()
}

pub fn decode(
    file_to_decode: std::path::PathBuf,
    chunk_type: ChunkType,
) -> Result<String> {
    todo!()
}

pub fn remove(
    file_to_clean: std::path::PathBuf,
    chunk_type: ChunkType,
) -> Result<std::path::PathBuf> {
    todo!()
}

pub fn print(
    file_to_print: std::path::PathBuf,
) -> Result<()> {
    todo!()
}
