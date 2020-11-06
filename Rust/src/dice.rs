use rand::Rng;

pub fn roll() -> [u8; 2] {
	let mut rng = rand::thread_rng();
	let d1 = rng.gen_range(1,7);
	let d2 = rng.gen_range(1,7);
	return [d1, d2]
}