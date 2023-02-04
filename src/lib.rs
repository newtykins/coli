use owo_colors::{OwoColorize, Rgb};
use rand::Rng;

// Extend owo_colors::Rgb to provide colour output methods
pub trait ColourConversion {
    fn to_rgb(&self) -> (u8, u8, u8);
    fn to_hex(&self) -> String;
    fn preview(&self) -> String;
}

impl ColourConversion for Rgb {
    fn to_rgb(&self) -> (u8, u8, u8) {
        (self.0, self.1, self.2)
    }

    fn to_hex(&self) -> String {
        format!("#{:x}{:x}{:x}", self.0, self.1, self.2)
    }

    fn preview(&self) -> String {
        format!("{}", "    ".on_color(*self))
    }
}

// Utils
fn log(label: &str, colour: Rgb, message: &str) {
    println!(
        "{} {}",
        format!("[{}]", label).color(colour).bold(),
        message
    )
}

/// Log a warning!
pub fn warn(message: &str) {
    log("WARN", Rgb(255, 191, 0), message);
}

/// Log an error!
pub fn error(message: &str) {
    log("ERROR", Rgb(255, 0, 0), message);
}

/// Generate a random colour
pub fn generate_colour() -> Rgb {
    let mut rng = rand::thread_rng();

    return Rgb(
        rng.gen_range(0..255),
        rng.gen_range(0..255),
        rng.gen_range(0..255),
    );
}
