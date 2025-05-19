use clap::{ArgMatches, Command};
use std::io;

use super::CommandError;

const COMMAND: &str = "serve";
const DESCRIPTION: &str = "Start server";

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Fail to create Tokio runtime")]
    TokioRuntimeError(#[from] io::Error),
}

pub fn configure() -> Command {
    Command::new(COMMAND).about(DESCRIPTION)
}

pub fn handle(matches: &ArgMatches) -> Result<(), CommandError> {
    if let Some(matches) = matches.subcommand_matches(COMMAND) {
        serve()?;
    }

    Ok(())
}

fn create_runtime() -> Result<tokio::runtime::Runtime, io::Error> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
}

fn serve() -> Result<(), Error> {
    let runtime = create_runtime()?;

    runtime.block_on(async move {});

    Ok(())
}
