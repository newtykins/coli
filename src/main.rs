use clap::{Parser, Subcommand};

mod gradient;
mod random;
mod trending;
mod utils;

// Main CLI definitions
#[derive(Parser)]
#[clap(about, author, version)]
struct Value {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generates a random colour
    Random(random::Options),

    /// Generates a gradient between two colours
    Gradient(gradient::Options),

    /// Displays today's trending palettes on coolors.co
    Trending(trending::Options),
}

fn main() {
    let value = Value::parse();

    match value.command {
        Commands::Random(mut options) => random::run(&mut options),
        Commands::Gradient(mut options) => gradient::run(&mut options),
        Commands::Trending(mut options) => trending::run(&mut options),
    }
}
