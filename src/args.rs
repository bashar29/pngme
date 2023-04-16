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

/// pngme encode --file ./file.png --type_chunk ruSt --secret_message "This is a secret message!" 
#[derive(Debug, Args)]
pub struct EncodeArgs {
    #[arg(short = 'f', long = "file")]
    pub file_to_encode: std::path::PathBuf,
    #[arg(short = 'c', long = "type_chunk")]
    pub type_chunk: String,
    #[arg(short = 'm', long = "secret_message")]
    pub message_to_encode: String,
}

/// pngme decode -f ./file.png -c ruSt
#[derive(Debug, Args)]
pub struct DecodeArgs {
    #[arg(short = 'f', long = "file")]
    pub file_with_message: std::path::PathBuf,
    #[arg(short = 'c', long = "type_chunk")]
    pub type_chunk: String,
}

/// pngme remove -f ./file.png
#[derive(Debug, Args)]
pub struct RemoveArgs {
    #[arg(short = 'f', long = "file")]
    pub file_to_clean: std::path::PathBuf,
    #[arg(short = 'c', long = "type_chunk")]
    pub type_chunk: String,
}

/// pngme print --file ./file.png
#[derive(Debug, Args)]
pub struct PrintArgs {
    #[arg(short = 'f', long = "file")]
    pub file_to_print: std::path::PathBuf,
}
