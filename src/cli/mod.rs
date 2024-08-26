pub mod report;
pub mod detect;

use clap::{Parser, Subcommand};
use std::path::PathBuf;


#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Optional name to operate on
    #[arg(short, long)]
    pub database: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,
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
    
        #[arg(short, long, required = true)]
        email: String,
    
        #[arg(short, long, required = true)]
        password: String,
    
        #[arg(short, long, required = true)]
        image: String,
    },
    Report {
        #[arg(short, long, required = true)]
        database: String,
    },
}