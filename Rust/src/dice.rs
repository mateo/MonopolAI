use rand::distributions::{Distribution, Uniform};

pub fn roll() -> [u8; 2] {
	let mut rng = rand::thread_rng();
	let d1 = Uniform::from(1..7);
	let d2 = Uniform::from(1..7);
	return [d1.sample(&mut rng), d2.sample(&mut rng)]
}