pub fn init() -> Vec<Box<Player>> {
	// Create the players
	let mut players: Vec<Box<Player>> = Vec::new();
	for i in 1..4 {
		players.push(Box::new(Player {
			cash: 1500,
			loc: 0,
			jail: false,
			snake_eyes: 0
		}))
	}
	return players
}

pub struct Player {
	loc: u8,
	cash: u16,
	jail: bool,
	snake_eyes: u8,
}

impl Player {

	// Roll the dice
	pub fn roll(&mut self) {
		let dice = super::roll_dice();

		// If in jail, try to roll snake eyes
		if self.jail {
			if dice[0] == dice[1] {
				self.travel(dice[0] + dice[1])
			}
		} else { // Otherwise, roll the dice normally
			// If snake eyes are rolled, check whether this automatically sends the player to jail
			if dice[0] == dice[1] {
				if self.snake_eyes < 2 {
					self.snake_eyes += 1
				} else {
					self.snake_eyes = 0;
					self.go_to_jail()
				}
			}
			self.travel(dice[0] + dice[1])
		}
	}

	// Move to a new location
	fn travel(&mut self, distance: u8) {
		self.loc += distance;
		if self.loc > 39 {
			self.loc %= 40;
			self.cash += 200
		}
		if self.loc == 20 {
			self.go_to_jail()
		}
	}

	// Go to jail
	fn go_to_jail(&mut self) {
		self.loc = 10;
		self.jail = true
	}

	// Receive cash
	pub fn income(&mut self, cash: u16) {
		self.cash += cash
	}

	// Pay cash
	pub fn pay(&mut self, cash: u16) {
		self.cash -= cash
	}

	// Is the player in jail
	pub fn is_in_jail(&self) -> bool {
		self.jail
	}

	// Show Total Cash
	pub fn cash(&self) -> u16 {
		self.cash
	}

	// Show Location
	pub fn loc(&self) -> u8 {
		self.loc
	}
}