use clap::{ArgMatches, Command};
use thiserror::Error;

mod hello;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("Failed to parse argument")]
    ArgumentParseError,
}

pub fn configure(command: Command) -> Command {
    command.subcommand(hello::configure())
}

pub fn handle(matches: &ArgMatches) -> Result<(), CommandError> {
    hello::handle(matches)?;

    Ok(())
}
