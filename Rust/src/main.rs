mod game;
use game::Game;
use game::player::Player;

fn main() {
	let G = Game::new();
	let players = game::player::init();
	let squares = game::square::init();
/*
	for i in 1..4 {
		players[i].roll();
		if !players[i].is_in_jail() {
			if squares[usize::from(players[i].loc())] == game::square::property::Property {
				if squares[usize::from(players[i].loc())].owner() > -1 {
					pay_rent(players[i], players[usize::from(squares[players[i].loc()].owner())], squares[players[i].loc()].rent())
				} else {
					// Decide if the player wants to buy the property
				}
			}
		}
	}
*/}

fn pay_rent(payer: &mut Player, payee: &mut Player, amount: u16) {
	payer.pay(amount);
	payee.income(amount)
}

// Isaac please fix this lmao you know how memory works