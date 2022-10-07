use owo_colors::{OwoColorize, Rgb};

// Extend owo_colors::Rgb to provide colour output methods

trait ColourConversion {
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
