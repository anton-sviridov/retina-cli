use clap::{Parser, Subcommand};
use std::path::PathBuf;
use suiwei::models::{Detection, NewDetection};
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
    Detect {
        #[arg(short, long)]
        database: Option<String>,
    },
    Report {
        #[arg(short, long, required=true)]
        file: String,
        
        #[arg(short, long, required=true)]
        comment: String,

        #[arg(short, long, required=true)]
        database: String,
    }
}


fn detect(database: &str) {
    println!("{}", database);
    use self::schema::detections::dsl::*;

    // let connection = &mut establish_connection(database_path);
    let connection = &mut SqliteConnection::establish(database).unwrap();
    let results = detections
        .limit(5)
        .select(Detection::as_select())
        .load(connection)
        .expect("Error loading detections");

    println!("Displaying {} detections", results.len());
    for detection in results {
        println!("{}", detection.image);
        println!("-----------\n");
        println!("{}", detection.description);
    }
}

fn report(file: &str, comment: &str, database: &str) {
    use self::schema::detections;
    let connection = &mut SqliteConnection::establish(database).unwrap();


    let new_memory = NewDetection { image: file, description: comment, date: "" };

    diesel::insert_into(detections::table)
        .values(&new_memory)
        .returning(Detection::as_returning())
        .get_result(connection)
        .expect("Error saving new post");
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
        Some(Commands::Detect { database }) => {
            if let Some(database) = database.as_deref() {
                detect(database)
            }
        },
        Some(Commands::Report { file, comment, database }) => {
            report(file, comment, database)
        },
        None => {},
    }
}