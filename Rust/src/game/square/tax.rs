pub struct Tax {
	amount: u8,
	lands: u8
}

impl Tax {

	// Create the Tax
	pub fn new(amount: u8) -> Tax {
		Tax {
			lands: 0,
			amount: amount
		}
	}

	// Show the amount
	pub fn amount(&self) -> u8 {
		self.amount
	}
}

impl super::Square for Tax {

	// Show how many times someone has landed on the square
	fn lands(&self) -> u8 {
		self.lands
	}

}