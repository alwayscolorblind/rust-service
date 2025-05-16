use clap::{ArgMatches, Command};
use thiserror::Error;

mod hello;
mod serve;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("Failed to parse argument")]
    ArgumentParseError,
}

pub fn configure(command: Command) -> Command {
    command
        .subcommand(hello::configure())
        .subcommand(serve::configure())
}

pub fn handle(matches: &ArgMatches) -> Result<(), CommandError> {
    hello::handle(matches)?;
    serve::handle(matches)?;

    Ok(())
}
