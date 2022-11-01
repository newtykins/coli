use clap::Args;
use headless_chrome::{Browser, LaunchOptionsBuilder};
use regex::Regex;

include!("rgb.rs");
include!("utils.rs");

#[derive(Args, Clone)]
pub struct Options {
    /// The amount of palettes to display. Max is 48
    #[arg(short, long, default_value_t = 10)]
    quantity: u8,
}

pub fn run(options: &mut Options) {
    if options.quantity > 48 {
        utils::warn("The max quantity for this subcommand is 48! Reverted to default of 10.");
        options.quantity = 48;
    }

    let browser = Browser::new(
        LaunchOptionsBuilder::default()
            .args(vec![std::ffi::OsStr::new("--disable-headless-mode")])
            .build()
            .unwrap(),
    )
    .unwrap();

    let tab = browser.wait_for_initial_tab().unwrap();
    let re = Regex::new(r"rgb\(([0-9]+),[ ]?([0-9]+),[ ]?([0-9]+)\)").unwrap();

    tab.navigate_to("https://coolors.co/palettes/trending")
        .unwrap();

    let palettes = tab.wait_for_elements(".palette-card_colors").unwrap();

    for i in 0..palettes.len() {
        if usize::from(options.quantity) == i {
            break;
        }

        print!("{}    ", format!("{}.", i + 1).bold());

        let palette = &palettes[i];
        let html = palette.get_content().unwrap();
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
}
