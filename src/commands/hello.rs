use clap::{Arg, ArgMatches, Command};

use super::CommandError;

const COMMAND: &str = "hello";
const DESCRIPTION: &str = "Just template command";

const WORD_ARG: &str = "word";

pub fn configure() -> Command {
    let word_arg = Arg::new(WORD_ARG).short('w').default_value("world");

    Command::new(COMMAND).about(DESCRIPTION).arg(word_arg)
}

pub fn handle(matches: &ArgMatches) -> Result<(), CommandError> {
    if let Some(matches) = matches.subcommand_matches(COMMAND) {
        let word = matches
            .get_one::<String>("word")
            .map(|s| s.as_str())
            .ok_or(CommandError::ArgumentParseError)?;

        println!("Hello {}!", word);
    }

    Ok(())
}
