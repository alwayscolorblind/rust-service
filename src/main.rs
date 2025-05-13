use clap::Arg;
use commands::CommandError;
use lazy_static::lazy_static;
use thiserror::Error;

mod commands;
mod settings;

// TODO: memory issues here?
// arg creates static refs so.... its probably better to drop them after use
lazy_static! {
    static ref CONFIG_ARG: Arg = Arg::new("config")
        .long("config")
        .short('c')
        .default_value("conifg.json");
}

#[derive(Debug, Error)]
enum MainError {
    #[error("Command error")]
    CommandError(#[from] CommandError),
}

fn main() -> Result<(), CommandError> {
    dotenv::dotenv().ok();

    // TODO: move to commands ?
    let mut command = clap::Command::new("App").arg(CONFIG_ARG.clone());

    command = commands::configure(command);

    let matches = command.get_matches();

    commands::handle(&matches)?;

    Ok(())
}
