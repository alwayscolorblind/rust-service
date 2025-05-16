use clap::{ArgMatches, Command};

use super::CommandError;

const COMMAND: &str = "serve";
const DESCRIPTION: &str = "Start server";

pub fn configure() -> Command {
    Command::new(COMMAND).about(DESCRIPTION)
}

pub fn handle(matches: &ArgMatches) -> Result<(), CommandError> {
    if let Some(matches) = matches.subcommand_matches(COMMAND) {
        serve();
    }

    Ok(())
}

fn create_runtime() ->

fn serve() {
}
