pub struct Property {
	houses: u8,
	prices: [u16; 2],
	rents: [u16; 6],
	mortgage: u8,
	is_mortgaged: bool,
	has_hotel: bool,
	owner: usize,
	lands: u8
}

impl Property {
	
	// Create the property
	pub fn new(rents: [u16; 6], prices: [u16; 2], mortgage: u8) -> Property {
		Property {
			// Set the basics
			houses: 0,
			has_hotel: false,
			is_mortgaged: false,
			owner: -1,
			lands: 0,
			// Set variables provided
			prices: prices,
			rents: rents,
			mortgage: mortgage
		}
	}

	// Build a house
	pub fn build_house(&mut self) {
		
		// Check if there is already a hotel
		if self.has_hotel { panic!("Can't do that, the property already has a hotel") }
		if self.is_mortgaged { panic!("Can't do that, the property is mortgaged") }
		
		// If there are less than 4 houses, add one
		if self.houses < 4 {
			self.houses += 1
		} else  { // If there are 4, remove them and build a hotel
			self.houses = 0;
			self.has_hotel = true
		}
	}

	// Show whether the property has a hotel
	pub fn has_hotel(&self) -> bool {
		self.has_hotel
	}

	// Show the number of houses
	pub fn houses(&self) -> u8 {
		self.houses
	}

	// Show the rent
	pub fn rent(&self) -> u16 {
		// If there is a hotel, rent is the maximum
		if self.has_hotel {
			self.rents[5]
		}
		else { // Otherwise, determine the rent based on the number of houses
			self.rents[self.houses]
		}
	}

	// Is the property mortgaged?
	pub fn is_mortgaged(&self) -> bool {
		self.is_mortgaged
	}

}

impl super::Ownable for Property {

	// Set the owner
	fn set_owner(&mut self, owner: i8) {
		self.owner = owner
	}

	// Show the owner
	fn owner(&self) -> i8 {
		self.owner
	}

}

impl super::Square for Property {

	// Show how many times someone has landed on the square
	fn lands(&self) -> u8 {
		self.lands
	}

}