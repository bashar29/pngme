mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
use args::{PngMeArgs, PnnmeFunctions};
use clap::Parser;
use log::{debug, error, info};

//pub type Error = Box<dyn std::error::Error>;
pub type Error = anyhow::Error;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    env_logger::init();
    let cmd = PngMeArgs::parse();
    info!("arguments : {:?}", cmd);
    match cmd.command_type {
        PnnmeFunctions::Encode(args) => {
            match commands::encode(
                &args.file_to_encode,
                args.type_chunk,
                &args.message_to_encode,
            ) {
                Ok(f) => info!("message encoded in file {}", f.display()),
                Err(e) => error!("message not encoded, error : {}", e),
            }
        }
        PnnmeFunctions::Decode(args) => {
            match commands::decode(&args.file_with_message, args.type_chunk) {
                Ok(s) => info!("decrypted message : {}", s),
                Err(e) => error!(
                    "failed to decode message : {} in file {}",
                    e,
                    &args.file_with_message.display()
                ),
            }
        }
        PnnmeFunctions::Remove(args) => {
            match commands::remove(&args.file_to_remove, args.type_chunk) {
                Ok(f) => info!("file cleaned of the message : {}", f.display()),
                Err(e) => error!(
                    "failed to clean the file {} : {}",
                    &args.file_to_remove.display(),
                    e
                ),
            }
        }
        PnnmeFunctions::Print(args) => match commands::print(&args.file_to_print) {
            Ok(_f) => debug!("file printed"),
            Err(e) => error!(
                "error {} when printing file {}",
                e,
                &args.file_to_print.display()
            ),
        },
    }

    Ok(())
}
