use clap::{ArgMatches, Command};
use serve::ServeError;
use thiserror::Error;

mod hello;
mod serve;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("Failed to parse argument {arg} in {command}")]
    ArgumentParseError { arg: String, command: String },
    #[error(transparent)]
    ServeError(#[from] ServeError),
}

pub type CommandResult<T> = Result<T, CommandError>;

pub fn configure(command: Command) -> Command {
    command
        .subcommand(hello::configure())
        .subcommand(serve::configure())
}

pub fn handle(matches: &ArgMatches) -> CommandResult<()> {
    hello::handle(matches)?;
    serve::handle(matches)?;

    Ok(())
}
