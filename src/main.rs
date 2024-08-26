use clap::Parser;
use retina_cli::cli::{report::report, detect::detect, Commands, Cli};


fn main() {
    let cli = Cli::parse();

    // To check provided values
    if let Some(database) = cli.database.as_deref() {
        println!("Value for name: {}", database);
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Report { database }) => report(database),
        Some(Commands::Detect {
            database,
            email,
            password,
            path,
        }) => detect(database, email, password, path),
        None => {}
    }
}
