use clap::Args;
use rand::Rng;

include!("rgb.rs");

#[derive(Args, Clone, Copy)]
pub struct Options {
    #[arg(short, long, default_value_t = 1)]
    quantity: usize,

    #[arg(short, long, default_value_t = false)]
    rgb: bool,
}

/// Generate a random colour
fn generate_colour() -> Rgb {
    let mut rng = rand::thread_rng();

    return Rgb(
        rng.gen_range(0..255),
        rng.gen_range(0..255),
        rng.gen_range(0..255),
    );
}

/// Generate a random colour and print it to the console
pub fn run(options: Options) {
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
            colour.preview(),
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
