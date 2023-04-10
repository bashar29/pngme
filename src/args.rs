use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct PngMeArgs {
    #[clap(subcommand)]
    pub command_type: PnnmeFunctions,
}

#[derive(Debug, Subcommand)]
pub enum PnnmeFunctions {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

/// pngme encode ./dice.png ruSt "This is a secret message!""
#[derive(Debug, Args)]
pub struct EncodeArgs {
    #[arg(short = 'f', long = "file")]
    pub file_to_encode: std::path::PathBuf,
    #[arg(short = 't', long = "type_chunk")]
    pub type_chunk: String,
    #[arg(short = 'm', long = "secret_message")]
    pub message_to_encode: String,
}

#[derive(Debug, Args)]
pub struct DecodeArgs {
    #[arg(short = 'f', long = "file")]
    pub file_with_message: std::path::PathBuf,
    #[arg(short = 't', long = "type_chunk")]
    pub type_chunk: String,
}

#[derive(Debug, Args)]
pub struct RemoveArgs {
    #[arg(short = 'f', long = "file")]
    pub file_to_remove: std::path::PathBuf,
    #[arg(short = 't', long = "type_chunk")]
    pub type_chunk: String,
}

#[derive(Debug, Args)]
pub struct PrintArgs {
    #[arg(short = 'f', long = "file")]
    pub file_to_print: std::path::PathBuf,
}
