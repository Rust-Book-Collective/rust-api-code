use crate::settings::Settings;
use crate::state::ApplicationState;
use clap::{value_parser, Arg, ArgMatches, Command};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

pub const COMMAND_NAME: &str = "serve";

pub fn configure() -> Command {
    Command::new(COMMAND_NAME).about("Start HTTP server").arg(
        Arg::new("port")
            .short('p')
            .long("port")
            .value_name("PORT")
            .help("TCP port to listen on")
            .default_value("8080")
            .value_parser(value_parser!(u16)),
    )
}

pub fn handle(matches: &ArgMatches, settings: &Settings) -> anyhow::Result<()> {
    let port: u16 = *matches.get_one("port").unwrap_or(&8080);

    start_tokio(port, settings)?;

    Ok(())
}

fn start_tokio(port: u16, settings: &Settings) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            let state = Arc::new(ApplicationState::new(settings)?);
            let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);
            let routes = crate::api::configure(state);

            let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
            axum::serve(listener, routes.into_make_service()).await?;

            Ok::<(), anyhow::Error>(())
        })?;

    std::process::exit(0);
}
