use std::str::FromStr;
use crate::{Result, png::Png, chunk_type::{ChunkType, self}, chunk::Chunk};
use std::fs::File;
use std::io::Write;

pub fn encode(
    file_to_encode: &std::path::PathBuf,
    chunk_type: String,
    message: &str,
) -> Result<std::path::PathBuf> {
    let mut original_png = Png::from_file(file_to_encode)?;
    let new_chunk_type: ChunkType = ChunkType::from_str(&chunk_type)?;
    let new_chunk: Chunk = Chunk::new(new_chunk_type,message.as_bytes().to_vec());
    original_png.append_chunk(new_chunk);
    let bytes: Vec<u8> = original_png.as_bytes();
    let mut new_file = file_to_encode.clone();
    let name = new_file.file_name().unwrap();
    let name = "encoded_".to_owned() + name.to_str().unwrap();
    new_file.set_file_name(name);
    let mut out = File::create(&new_file)?;
    out.write_all(&bytes)?;
    Ok(new_file)
}

pub fn decode(file_to_decode: &std::path::PathBuf, chunk_type: String) -> Result<String> {
    todo!()
}

pub fn remove(
    file_to_clean: &std::path::PathBuf,
    chunk_type: String,
) -> Result<std::path::PathBuf> {
    todo!()
}

pub fn print(file_to_print: &std::path::PathBuf) -> Result<()> {
    todo!()
}
