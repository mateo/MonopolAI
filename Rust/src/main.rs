mod dice;

fn main() {
	let roll = dice::roll();
	println!("{a}, {b}", a=roll[0], b=roll[1]);
}