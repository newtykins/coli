#[allow(dead_code)]

pub mod utils {
    use rand::Rng;
    include!("rgb.rs");

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

    /// Generate a random colour
    pub fn generate_colour() -> Rgb {
        let mut rng = rand::thread_rng();

        return Rgb(
            rng.gen_range(0..255),
            rng.gen_range(0..255),
            rng.gen_range(0..255),
        );
    }
}
