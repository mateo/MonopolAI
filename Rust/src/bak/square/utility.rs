pub struct Utility {
	price: u8,
	rent_multiplier: [u8; 2],
	mortgage: u8,
	is_mortgaged: bool,
	owner: i8,
	lands: u8
}

impl Utility {

	// Create a new utility
	pub fn new(rent_multiplier: [u8; 2], price: u8, mortgage: u8) -> Utility {
		Utility {
			// Set the basics
			owner: -1,
			lands: 0,
			is_mortgaged: false,
			// Set variables provided
			price: price,
			rent_multiplier: rent_multiplier,
			mortgage: mortgage
		}
	}

	// Show the rent multipliers
	pub fn rent_multiplier(&self) -> [u8; 2] {
		self.rent_multiplier
	}

	// Is the property mortgaged?
	pub fn is_mortgaged(&self) -> bool {
		self.is_mortgaged
	}

}

impl super::Ownable for Utility {

	// Set the owner
	fn set_owner(&mut self, owner: i8) {
		self.owner = owner
	}

	// Show the owner
	fn owner(&self) -> i8 {
		self.owner
	}

}

impl super::Square for Utility {

	// Show how many times someone has landed on the square
	fn lands(&self) -> u8 {
		self.lands
	}

	// Show Property Type
	fn kind(&self) -> u8 { 2 }

}