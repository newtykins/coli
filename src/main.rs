use clap::{Args, Parser, Subcommand};
use owo_colors::{OwoColorize, Rgb};
use rand::Rng;

const DEBUG_STR: &str = "    ";

trait ColourConversion {
    fn to_rgb(&self) -> (u8, u8, u8);
    fn to_hex(&self) -> String;
}

impl ColourConversion for Rgb {
    fn to_rgb(&self) -> (u8, u8, u8) {
        (self.0, self.1, self.2)
    }

    fn to_hex(&self) -> String {
        format!("#{:x}{:x}{:x}", self.0, self.1, self.2)
    }
}

fn generate_colour() -> Rgb {
    let mut rng = rand::thread_rng();

    return Rgb(
        rng.gen_range(0..255),
        rng.gen_range(0..255),
        rng.gen_range(0..255),
    );
}

#[derive(Parser)]
#[clap(about, author, version)]
struct Value {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Random(RandomOptions),
}

#[derive(Args)]
struct RandomOptions {
    #[arg(short, long, default_value_t = 1)]
    quantity: usize,

    #[arg(short, long, default_value_t = false)]
    rgb: bool,
}

fn main() {
    let value = Value::parse();

    match &value.command {
        Commands::Random(options) => {
            if options.quantity <= 0 {
                print!(
                    "{} {}",
                    "[ERROR]".color(Rgb(255, 0, 0)).bold(),
                    "Quantity must be greater than 0!"
                );

                return;
            }

            for _ in 0..options.quantity {
                let colour = generate_colour();

                println!(
                    "{}  {}",
                    DEBUG_STR.on_color(colour),
                    if options.rgb {
                        format!(
                            "R {}, G {}, B {}",
                            colour.0.bold(),
                            colour.1.bold(),
                            colour.2.bold()
                        )
                    } else {
                        colour.to_hex().bold().to_string()
                    }
                );
            }
        }
    }
}
