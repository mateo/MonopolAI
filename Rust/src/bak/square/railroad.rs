pub struct Railroad {
	price: u8,
	rent: [u8; 4],
	mortgage: u8,
	is_mortgaged: bool,
	owner: i8,
	lands: u8
}

impl Railroad {

	// Create a new utility
	pub fn new(rent: [u8; 4], price: u8, mortgage: u8) -> Railroad {
		Railroad {
			// Set the basics
			owner: -1,
			lands: 0,
			is_mortgaged: false,
			// Set variables provided
			price: price,
			rent: rent,
			mortgage: mortgage
		}
	}

	// Show the rent multipliers
	pub fn rent(&self) -> [u8; 4] {
		self.rent
	}

	// Is the property mortgaged?
	pub fn is_mortgaged(&self) -> bool {
		self.is_mortgaged
	}
	
}

impl super::Ownable for Railroad {

	// Set the owner
	fn set_owner(&mut self, owner: i8) {
		self.owner = owner
	}

	// Show the owner
	fn owner(&self) -> i8 {
		self.owner
	}

}

impl super::Square for Railroad {

	// Show how many times someone has landed on the square
	fn lands(&self) -> u8 {
		self.lands
	}

	// Show Property Type
	fn kind(&self) -> u8 { 3 }

}