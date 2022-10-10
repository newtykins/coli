use clap::Args;
use palette::LinSrgb;

include!("rgb.rs");
include!("utils.rs");

#[derive(Args, Clone)]
pub struct Options {
    #[arg(short, long)]
    from: Option<String>,

    #[arg(short, long)]
    to: Option<String>,

    #[arg(short, long, default_value_t = 10)]
    steps: usize,
}

fn generate_gradient(from: Rgb, to: Rgb, len: usize) -> Vec<Rgb> {
    let (r1, g1, b1) = from.to_rgb();
    let (r2, g2, b2) = to.to_rgb();

    let gradient = palette::Gradient::new(vec![
        LinSrgb::new(r1 as f32, g1 as f32, b1 as f32),
        LinSrgb::new(r2 as f32, g2 as f32, b2 as f32),
    ]);

    return gradient
        .take(len)
        .map(
            |palette::rgb::Rgb {
                 red: r,
                 green: g,
                 blue: b,
                 standard: _,
             }| owo_colors::Rgb(r as u8, g as u8, b as u8),
        )
        .collect();
}

fn parse_hex(hex_string: String) -> owo_colors::Rgb {
    let split = hex_string.split('#').nth_back(0).unwrap_or("").to_string();

    if split.len() != 6 {
        return generate_colour();
    }

    let colour = hex::decode(split).map(|bytes| owo_colors::Rgb(bytes[0], bytes[1], bytes[2]));

    match colour {
        Ok(out) => return out,
        Err(_) => return generate_colour(),
    }
}

pub fn run(options: Options) {
    if options.steps <= 0 {
        error("Step count must be greater than 0!");
        return;
    }

    let from;
    let to;

    match options.from {
        Some(hex) => from = parse_hex(hex),
        None => from = generate_colour(),
    }

    match options.to {
        Some(hex) => to = parse_hex(hex),
        None => to = generate_colour(),
    }

    let gradient = generate_gradient(from, to, options.steps);

    for color in gradient {
        print!("{}", color.preview());
    }

    print!("    {} -> {}", from.to_hex().bold(), to.to_hex().bold());
}
