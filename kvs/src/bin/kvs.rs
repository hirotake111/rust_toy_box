use std::process::exit;

use clap::{Error, Parser, Subcommand};

#[derive(Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    author = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION"), 
    about = env!("CARGO_PKG_DESCRIPTION"),
)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    // Get a value by given key
    Get { key: String },
    Set { key: String, value: String },
    Rm { key: String },
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    match &cli.command {
        #[allow(unused_variables)]
        Commands::Get { key } => {
            eprintln!("unimplemented");
            exit(1);
        }
        #[allow(unused_variables)]
        Commands::Set { key, value } => {
            eprintln!("unimplemented");
            exit(1);
        }
        #[allow(unused_variables)]
        Commands::Rm { key } => {
            eprintln!("unimplemented");
            exit(1);
        }
    }
}
