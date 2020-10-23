use rand::Rng;

pub mod player;
pub mod square;

fn roll_dice() -> [u8; 2] {
	let mut rng = rand::thread_rng();
	let d1 = rng.gen_range(1,6);
	let d2 = rng.gen_range(1,6);
	return [d1, d2]
}

pub struct Game {
	houses: u8,
	hotels: u8
}

impl Game {

	// Create the Game
	pub fn new() -> Game {
		Game {
			houses: 32,
			hotels: 12
		}
	}

	// Show how many available hotels there are
	pub fn hotels(&self) -> u8 {
		self.hotels
	}

	// Show how many available houses there are
	pub fn houses(&self) -> u8 {
		self.houses
	}

	// Remove houses from the pool
	pub fn place_houses(&mut self, n: u8) {
		self.houses -= n
	}

	// Remove hotels from the pool
	pub fn place_hotels(&mut self, n: u8) {
		self.hotels -= n
	}

	// Add houses to the pool
	pub fn demo_house(&mut self, n: u8) {
		self.houses += n
	}

	// Add hotels to the pool
	pub fn demo_hotel(&mut self, n: u8) {
		self.hotels += n
	}
}