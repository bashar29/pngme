mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
use anyhow::bail;
use args::{DecodeArgs, EncodeArgs, PnnmeFunctions, PrintArgs, RemoveArgs};
use log::{debug, error, info, warn};

//pub type Error = Box<dyn std::error::Error>;
pub type Error = anyhow::Error;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    env_logger::init();

    Ok(())
}
