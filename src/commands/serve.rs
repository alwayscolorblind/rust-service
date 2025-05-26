use std::{
    net::{Ipv4Addr, SocketAddrV4},
    str::FromStr,
};

use axum::Router;
use clap::{Arg, ArgMatches, Command};
use thiserror::Error;

use super::{CommandError, CommandResult};

const COMMAND: &str = "serve";
const DESCRIPTION: &str = "Start server";

#[derive(Debug, Error)]
pub enum ServeError {
    #[error("Failed to create tokio runtime")]
    RuntimeIOError(#[from] std::io::Error),
    #[error("Failed to parse ip {0}")]
    Ipv4FromStrError(String),
}

type ServeResult<T> = Result<T, ServeError>;

pub fn configure() -> Command {
    let port_arg = Arg::new("port")
        .short('p')
        .default_value("8080")
        .value_parser(clap::value_parser!(u16));

    Command::new(COMMAND).about(DESCRIPTION).arg(port_arg)
}

pub fn handle(matches: &ArgMatches) -> CommandResult<()> {
    if let Some(matches) = matches.subcommand_matches(COMMAND) {
        let port = *matches
            .get_one::<u16>("port")
            .ok_or(CommandError::ArgumentParseError {
                arg: "port".to_string(),
                command: COMMAND.to_string(),
            })?;

        serve(port)?;
    }

    Ok(())
}

fn create_runtime() -> ServeResult<tokio::runtime::Runtime> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(ServeError::RuntimeIOError)
}

fn create_sockaddr(s: &str, port: u16) -> ServeResult<SocketAddrV4> {
    let ip = Ipv4Addr::from_str(s).map_err(|_| ServeError::Ipv4FromStrError(s.to_string()))?;

    Ok(SocketAddrV4::new(ip, port))
}

fn serve(port: u16) -> ServeResult<()> {
    use tokio::net::TcpListener;

    let addr = create_sockaddr("127.0.0.1", port)?;

    create_runtime()?.block_on(async move {
        let make_service = Router::new();

        let listener = TcpListener::bind(addr).await.unwrap();

        axum::serve(listener, make_service).await.unwrap();
    });

    Ok(())
}
