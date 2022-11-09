mod config;
mod cli;
mod snapraid_command;
mod log;
mod tests;

use std::error::Error;
use crate::cli::CliCommand;
use crate::snapraid_command::SnapraidCommand;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = cli::parse_cli()?;
    let config = config::load_config(cli.config)?;
    std::env::set_var("RUST_LOG", cli.log_level.unwrap_or(config.log_level).as_ref());
    env_logger::init();

    let cmd = SnapraidCommand {
        binary: config.snapraid_binary,
        conf: config.snapraid_config
    };

    match cli.command {
        CliCommand::diff => {
            let diff = cmd.diff()?;
            println!("Added: {}", diff.added());
        }
        _ => {
            panic!("Unsupported operation");
        }
    }

    std::process::exit(0);
}
