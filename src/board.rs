use types::Move;
use std;

pub struct Board {
	b: [[bool; 16]; 16],
	player_position: [(u32, u32); 2]
}

impl Board {
	pub fn legal_moves(&self, player: u32) -> std::vec::IntoIter<Move> {
		let pos = self.player_position[player as usize];
		let mut moves = vec![];
		if pos.1 > 0  && self.b[pos.0 as usize][(pos.1 - 1) as usize] { moves.push(Move::Up); }
		if pos.1 < 15 && self.b[pos.0 as usize][(pos.1 + 1) as usize] { moves.push(Move::Down); }
		if pos.0 > 0  && self.b[(pos.0 - 1) as usize][pos.1 as usize] { moves.push(Move::Left); }
		if pos.0 < 15 && self.b[(pos.0 + 1) as usize][pos.1 as usize] { moves.push(Move::Right); }
		moves.into_iter()
	}
}

impl<'a> From<&'a str> for Board {
	fn from(text: &'a str) -> Board {
		let mut x = 0;
		let mut y = 0;
		let mut b = [[false; 16]; 16];
		let mut player_position = [(0, 0); 2];
		for c in text.split(',') {
			b[x as usize][y as usize] = c == ".";
			if c == "0" {
				player_position[0] = (x, y);
			} else if c == "1" {
				player_position[1] = (x, y);
			}
			x = (x+1) % 16;
			if x == 0 {
				y += 1;
			}
		}
		Board {
			b: b,
			player_position: player_position
		}
	}
}
