use rand::Rng;

/// Log an error!
pub fn error(message: &str) {
    print!(
        "{} {}",
        "[ERROR]".color(Rgb(255, 0, 0)).bold(),
        message
    );
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
