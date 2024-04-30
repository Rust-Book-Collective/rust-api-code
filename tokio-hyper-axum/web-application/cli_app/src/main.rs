use clap::{Arg, Command};
use cli_app::{commands, settings};

fn main() -> anyhow::Result<()> {
    let mut command = Command::new("Sample CLI application").arg(
        Arg::new("config")
            .short('c')
            .long("config")
            .help("Configuration file location"),
    );

    command = commands::configure(command);

    let matches = command.get_matches();

    let config_location = matches
        .get_one("config")
        .map(|s: &String| Some(s.as_str()))
        .unwrap_or(None);

    let settings = settings::Settings::new(config_location, "APP")?;

    commands::handle(&matches, &settings)?;

    Ok(())
}
