use clap::Args;

include!("rgb.rs");
include!("utils.rs");

#[derive(Args, Clone)]
pub struct Options {
    /// The amount of colours to generate
    #[arg(short, long, default_value_t = 1)]
    quantity: usize,

    /// Output the colour value in RGB rather than hex
    #[arg(short, long, default_value_t = false)]
    rgb: bool,
}

/// Generate a random colour and print it to the console
pub fn run(options: &Options) {
    if options.quantity <= 0 {
        utils::error("Quantity must be greater than 0!");
        return;
    }

    for _ in 0..options.quantity {
        let colour = utils::generate_colour();

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
