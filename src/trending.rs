use headless_chrome::Browser;
use regex::Regex;

include!("rgb.rs");

// todo: find a way to hide the headless chrome window
// todo: allow for more than 48 palettes at once by scrolling the window before parsing
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let browser = Browser::default()?;
    let tab = browser.wait_for_initial_tab()?;
    let re = Regex::new(r"rgb\(([0-9]+),[ ]?([0-9]+),[ ]?([0-9]+)\)").unwrap();

    tab.navigate_to("https://coolors.co/palettes/trending")?;
    let palettes = tab.wait_for_elements(".palette-card_colors")?;

    for palette in &palettes {
        let html = palette.get_content().unwrap_or("".to_string());
        let colours = re.captures_iter(Box::leak(html.into_boxed_str()));
        let mut url = "https://coolors.co/palette/".to_string();

        for colour_values in colours {
            let rgb_values = vec![
                u8::from_str_radix(colour_values.get(1).unwrap().as_str(), 10).unwrap(),
                u8::from_str_radix(colour_values.get(2).unwrap().as_str(), 10).unwrap(),
                u8::from_str_radix(colour_values.get(3).unwrap().as_str(), 10).unwrap(),
            ];

            let colour = Rgb(rgb_values[0], rgb_values[1], rgb_values[2]);

            url += Box::leak(format!("{}-", hex::encode(rgb_values)).into_boxed_str());

            print!("{}", colour.preview());
        }

        url.pop();
        print!("    {}", url.blue().underline().bold());

        // New line for each palette!
        print!("\n");
    }

    println!("{}", palettes.len());

    Ok(())
}
