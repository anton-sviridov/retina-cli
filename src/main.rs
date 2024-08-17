use clap::{Parser, Subcommand};
use std::path::PathBuf;
use random_word::Lang;
use suiwei::models::*;
use diesel::prelude::*;
use suiwei::*;


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
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
enum Commands {
    Decide,
    Remember {
        #[arg(short, long)]
        database: Option<String>,
    }
}

fn decide() {
    let word = random_word::gen(Lang::En);
    print!("{} ", word);
    println!("");
}

fn remember(database_path: &str) {
    println!("{}", database_path);
    use self::schema::memories::dsl::*;

    // let connection = &mut establish_connection(database_path);
    let connection = &mut SqliteConnection::establish(database_path).unwrap();
    let results = memories
        .limit(5)
        .select(Memory::as_select())
        .load(connection)
        .expect("Error loading memories");

    println!("Displaying {} memories", results.len());
    for memory in results {
        println!("{}", memory.image);
        println!("-----------\n");
        println!("{}", memory.description);
    }
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
        Some(Commands::Decide) => {
            decide()
        },
        Some(Commands::Remember { database }) => {
            if let Some(database) = database.as_deref() {
                remember(database)
            }
        },
        None => {},
    }

}