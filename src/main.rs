use clap::{Parser, Subcommand};

mod random;

// Main CLI definitions
#[derive(Parser)]
#[clap(about, author, version)]
struct Value {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Random(random::Options),
}

fn main() {
    let value = Value::parse();

    match &value.command {
        Commands::Random(options) => random::run(*options),
    }
}
