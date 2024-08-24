use clap::{Parser, Subcommand};
use retina_cli::cli::{detect, report};
use std::path::PathBuf;


#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Optional name to operate on
    #[arg(short, long)]
    database: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    // TODO: add flags email, password, image
    Detect {
        #[arg(short, long, required = true)]
        file: String,

        #[arg(short, long, required = true)]
        comment: String,

        #[arg(short, long, required = true)]
        database: String,
    },
    Report {
        #[arg(short, long, required = true)]
        database: String,
    },
}


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
            file,
            comment,
            database,
        }) => detect(file, comment, database),
        None => {}
    }
}
