use owo_colors::OwoColorize;
use rand::Rng;

fn generate_rgb_pair() -> (u8, u8, u8) {
    let mut rng = rand::thread_rng();

    return (
        rng.gen_range(0..255),
        rng.gen_range(0..255),
        rng.gen_range(0..255),
    );
}

fn main() {
    let (r, g, b) = generate_rgb_pair();
    let colour = owo_colors::Rgb(r, g, b);
    let debug_str = "     ";

    println!("{} #{:x}{:x}{:x}", debug_str.on_color(colour), r, g, b);
}
