use std::fs::read_to_string;
use std::env;
use serde_json::Value;

mod dice;

fn main() {
	// Initialize the JSON datastores
	let root: String = env::var("MONOPOLAI_PATH").expect("Please define the MONOPOLAI_PATH Variable");
	// Players
	let path: String = format!("{}/Rust/src/player.json", root);
	let json = read_to_string(path).expect("Something went wrong reading the JSON File");
	let players: Value = serde_json::from_str(&json).expect("Something went wrong parsing the JSON File");
	//Squares
	let path: String = format!("{}/Rust/src/properties.json", root);
	let json = read_to_string(path).expect("Something went wrong reading the JSON File");
	let squares: Value = serde_json::from_str(&json).expect("Something went wrong parsing the JSON File");

	println!("Rents: {}", squares["1"]["rents"]);
	println!("Location: {}", players["location"]);

	let roll = dice::roll();
	println!("{a}, {b}", a=roll[0], b=roll[1]);
}
