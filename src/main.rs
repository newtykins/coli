use owo_colors::{OwoColorize, Rgb};
use palette::LinSrgb;
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

fn random_colour() -> Rgb {
    let mut rng = rand::thread_rng();

    return Rgb(
        rng.gen_range(0..255),
        rng.gen_range(0..255),
        rng.gen_range(0..255),
    );
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
                 red,
                 green,
                 blue,
                 standard: _,
             }| owo_colors::Rgb(red as u8, green as u8, blue as u8),
        )
        .collect();
}

fn main() {
    // ! Random colour
    // let colour = random_colour();

    // println!(
    //     "{} #{:x}{:x}{:x}",
    //     DEBUG_STR.on_color(colour),
    //     colour.0,
    //     colour.1,
    //     colour.2
    // );

    // ! Gradient
    let from = random_colour();
    let to = random_colour();
    let colours = generate_gradient(from, to, 10);

    for i in 0..colours.len() {
        print!("{}", DEBUG_STR.on_color(colours[i]));

        if i == colours.len() - 1 {
            print!("     {} → {}", from.to_hex(), to.to_hex());
        }
    }
}
