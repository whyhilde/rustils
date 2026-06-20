use clap::{Parser, Subcommand};

use crate::commands::unzip_all;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Colors,
    UnzipAll(unzip_all::Args),
}
