use crate::{chunk::Chunk, chunk_type::ChunkType, png::Png, Error, Result};
use chrono::{DateTime, Local};
use log::{info, warn};
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
    info!("new file : {:?}",new_file.to_str());
    let mut out = File::create(&new_file)?;
    out.write_all(&bytes)?;
    Ok(new_file)
}

pub fn decode(file_to_decode: &std::path::PathBuf, chunk_type: String) -> Result<String> {
    let original_png = Png::from_file(file_to_decode)?;
    let chunk_with_msg = original_png.chunk_by_type(&chunk_type);
    if let Some(c) = chunk_with_msg {
        let message = c.data_as_string()?;
        info!("decrypted message : {}",message);
        return Ok(message);
    }
    warn!("no message encoded");
    let e = "no message encoded".to_string();
    Err(Error::msg(e))
}

pub fn remove(
    file_to_clean: &std::path::PathBuf,
    chunk_type: String,
) -> Result<std::path::PathBuf> {
    let mut original_png = Png::from_file(file_to_clean)?;
    let _r = original_png.remove_chunk(&chunk_type);
    let bytes = original_png.as_bytes();
    let new_png = Png::try_from(bytes.as_slice())?;
    let new_file = set_new_file_name("cleaned", &mut file_to_clean.clone()).unwrap();
    info!("new file : {:?}",new_file.to_str());
    let mut out = File::create(&new_file)?;
    out.write_all(&new_png.as_bytes())?;
    Ok(new_file)
}

pub fn print(file_to_print: &std::path::PathBuf) -> Result<()> {
    let original_png = Png::from_file(file_to_print)?;
    let bytes = original_png.as_bytes();
    info!("{:?}",bytes);
    Ok(())
}

fn set_new_file_name(
    prefix: &str,
    new_file: &mut std::path::PathBuf,
) -> Result<std::path::PathBuf> {
    let name = new_file.file_stem().unwrap();
    let ext = new_file.extension().unwrap();
    let now: DateTime<Local> = Local::now();
    let now = format!("{}", now.format("_%H%M%S%d%m%Y_"));
    let new_full_name =
        prefix.to_owned() + &now + name.to_str().unwrap() + "." + ext.to_str().unwrap();
    new_file.set_file_name(new_full_name);
    Ok(new_file.to_path_buf())
}
