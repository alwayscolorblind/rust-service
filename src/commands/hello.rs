use clap::{Arg, ArgMatches, Command};
use lazy_static::lazy_static;

use super::CommandError;

const COMMAND: &str = "hello";
const DESCRIPTION: &str = "Just template command";

lazy_static! {
    static ref WORD_ARG: Arg = Arg::new("word").short('w').default_value("world");
}

pub fn configure() -> Command {
    Command::new(COMMAND)
        .about(DESCRIPTION)
        .arg(WORD_ARG.clone())
}

pub fn handle(matches: &ArgMatches) -> Result<(), CommandError> {
    if let Some(matches) = matches.subcommand_matches(COMMAND) {
        let word = matches
            .get_one::<String>(WORD_ARG.get_id().as_str())
            .map(|s| s.as_str())
            .ok_or(CommandError::ArgumentParseError)?;

        println!("Hello {}!", word);
    }

    Ok(())
}
