use std::fs::read_to_string;
use std::env;

use serde_json::Value;

pub trait Square {
	fn lands(&self) -> u8;
	fn kind(&self) -> u8;
	// Kind Listings:
	// 0: Property
	// 1: Tax
	// 2: Utility
	// 3: Railroad
	// 4: 
}

pub trait Ownable {
	fn owner(&self) -> i8;
	fn set_owner(&mut self, owner: i8);
}

pub mod property;
pub mod tax;
pub mod railroad;
pub mod utility;

use property::Property;
use tax::Tax;
use railroad::Railroad;
use utility::Utility;

pub fn init() -> Vec<Box<dyn Square>> {

	// Read the JSON property configuration file
	let root: String = env::var("MONOPOLAI_PATH").expect("Please define the MONOPOLAI_PATH Variable");
	let path: String = format!("{}/Rust/src/game/square/properties.json", root);
	let json = read_to_string(path).expect("Something went wrong reading the JSON File");

	let prop: Value = serde_json::from_str(&json).expect("Something went wrong parsing the JSON File");

	println!("{}", prop["Brown"]["Mediterranean Avenue"]["rents"]);

	// Create the Vector
	let mut squares: Vec<Box<dyn Square>> = vec![

		// Mediterranean Avenue
		Box::new(Property::new(prop["Brown"]["Mediterranean Avenue"]["rents"].as_array().unwrap(), [60, 50], 30)),

		// Community Chest

		// Baltic Avenue

		// Income Tax
		Box::new(Tax::new(200)),

		// Reading Railroad
	//	squares.push(Box::new(Railroad::new([25, 50, 100, 200], 200, 100)));

		// Oriental Avenue

		// Chance

		// Vermont Avenue

		// Connecticut Avenue

		// Jail

		// St Charles Place

		// Electric Company
	//	squares.push(Box::new(Utility::new([4, 10], 150, 75)));

		// States Avenue

		// Virginia Avenue

		// Pennsylvania Railroad
	//	squares.push(Box::new(Railroad::new([25, 50, 100, 200], 200, 100)));

		// St James Place

		// Community Chest

		// Tennesse Avenue

		// New York Avenue

		// Free Parking

		// Kentucky Avenue

		// Chance

		// Indiana Avenue

		// Illinois Avenue

		// B&O Railroad
	//	squares.push(Box::new(Railroad::new([25, 50, 100, 200], 200, 100)));

		// Atlantic Avenue

		// Ventnor Avenue

		// Water Works
	//	squares.push(Box::new(Utility::new([4, 10], 150, 75)));

		// Marvin Gardens

		// Go To Jail

		// Pacific Avenue

		// North Carolina Avenue

		// Community Chest

		// Pennsylvania Avenue

		// Short Line
	//	squares.push(Box::new(Railroad::new([25, 50, 100, 200], 200, 100)));

		// Chance

		// Park Place

		// Luxury Tax
	//	squares.push(Box::new(Tax::new(100)));

		// Boardwalk

	];

	return squares
	
}