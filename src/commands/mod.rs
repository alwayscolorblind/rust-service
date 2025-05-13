use clap::{ArgMatches, Command};
use thiserror::Error;

mod hello;

#[derive(Debug, Error)]
pub enum CommandError {}

pub fn configure(command: Command) -> Command {
    command.subcommand(hello::configure())
}

pub fn handle(matches: &ArgMatches) -> Result<(), CommandError> {
    hello::handle(matches)?;

    Ok(())
}
