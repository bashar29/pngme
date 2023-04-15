use crate::{chunk::Chunk, chunk_type::ChunkType, png::Png, Result};
use chrono::{DateTime, Local};
use std::fs::File;
use std::io::Write;
use std::str::FromStr;

pub fn encode(
    file_to_encode: &std::path::PathBuf,
    chunk_type: String,
    message: &str,
) -> Result<std::path::PathBuf> {
    let mut original_png = Png::from_file(file_to_encode)?;
    let new_chunk_type: ChunkType = ChunkType::from_str(&chunk_type)?;
    let new_chunk: Chunk = Chunk::new(new_chunk_type, message.as_bytes().to_vec());
    original_png.append_chunk(new_chunk);
    let bytes: Vec<u8> = original_png.as_bytes();
    let new_file = set_new_file_name("encoded", &mut file_to_encode.clone()).unwrap();
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

fn set_new_file_name<'a>(
    prefix: &str,
    new_file: &'a mut std::path::PathBuf,
) -> Result<std::path::PathBuf> {
    let name = new_file.file_stem().unwrap();
    let ext = new_file.extension().unwrap();
    let now: DateTime<Local> = Local::now();
    let now = format!("{}", now.format("_%H%M%S%d%m%Y_"));
    let new_full_name = prefix.to_owned() + &now + name.to_str().unwrap() + "." + ext.to_str().unwrap();
    new_file.set_file_name(new_full_name);
    Ok(new_file.to_path_buf())
}
