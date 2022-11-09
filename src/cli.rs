use clap::{Parser, Subcommand};
use std::path::PathBuf;
use crate::log::LogLevel;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: PathBuf,

    /// Set logging level
    #[arg(short, long)]
    pub log_level: Option<LogLevel>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    pub command: CliCommand,
}

#[derive(Subcommand)]
pub enum CliCommand {
    /// Standard Sync
    sync {

    },
    diff
}

pub fn parse_cli() -> Result<Cli, String> {
    return Ok(Cli::parse());
}