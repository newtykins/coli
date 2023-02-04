use rand::seq::SliceRandom;
use std::{collections::HashMap};
use coli::{ColourConversion, error};
use owo_colors::OwoColorize;

/// Get recent palettes made on https://colorhunt.co/
pub fn run() {
	// Fetch the data from ColorHunt
    let response = reqwest::blocking::Client::new()
        .post("https://colorhunt.co/php/feed.php")
        .form(&[("sort", "new")])
        .send()
		.unwrap();

	if response.status() == 404 {
		error("Sorry, but there was an error while interacting with ColorHunt!");
		return;
	}

	// Select a random five recent palettes
	let data = response.json::<Vec<HashMap<String, String>>>().unwrap();
	let slice: Vec<_> = data.choose_multiple(&mut rand::thread_rng(), 5).collect();

	for palette in slice {
		// Split the code into characters
		let split_code = palette
			.get("code")
			.unwrap()
			.chars()
			.collect::<Vec<char>>();
			
		// Collect groups of 6
		let hexes = split_code
			.chunks(6)
			.map(|c| c.iter().collect::<String>());

		// Make a list of strings from these new found hexes
		let hex_strings = hexes.clone().map(|s| "#".to_string() + &s).collect::<Vec<String>>();

		// Turn them into colours!
		let colours = hexes
			.map(|s| hex::decode(s).unwrap())
			.map(|rgb| owo_colors::Rgb(rgb[0], rgb[1], rgb[2]))
			.collect::<Vec<owo_colors::Rgb>>();

		// Display the colours
		for colour in colours {
			print!("{}", colour.preview());
		}

		// And display the hex codes for each colour
		print!(" {}, {}, {}, {}\n", hex_strings[0].bold(), hex_strings[1].bold(), hex_strings[2].bold(), hex_strings[3].bold());
	}
}
