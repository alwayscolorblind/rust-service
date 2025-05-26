use clap::Arg;
use commands::CommandError;
use thiserror::Error;

mod commands;
mod settings;

const CONFIG_ARG: &str = "config";

#[derive(Debug, Error)]
enum MainError {
    #[error("Command error")]
    CommandError(#[from] CommandError),
}

fn main() -> Result<(), CommandError> {
    dotenv::dotenv().ok();

    let config_arg = Arg::new("config")
        .long(CONFIG_ARG)
        .short('c')
        .default_value("conifg.json");

    // TODO: move to commands ?
    let mut command = clap::Command::new("App").arg(config_arg);

    command = commands::configure(command);

    let matches = command.get_matches();

    let res = commands::handle(&matches);

    if let Err(e) = res {
        eprintln!("[Error]: {e:?}: {e}")
    }

    Ok(())
}
