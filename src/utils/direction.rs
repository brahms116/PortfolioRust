#[derive(Hash, PartialEq, Eq, Clone)]
pub enum Direction {
	N,
	NE,
	E,
	SE,
	S,
	SW,
	W,
	NW,
}

impl Direction {
	pub fn to_i32(&self) -> i32 {
		match self {
			Direction::N => 0,
			Direction::NE => 1,
			Direction::E => 2,
			Direction::SE => 3,
			Direction::S => 4,
			Direction::SW => 5,
			Direction::W => 6,
			Direction::NW => 7,
		}
	}
	pub fn from_i32(num: i32) -> Direction {
		let num = num % 8;
		match num {
			0 => Direction::N,
			1 => Direction::NE,
			2 => Direction::E,
			3 => Direction::SE,
			4 => Direction::S,
			5 => Direction::SW,
			6 => Direction::W,
			7 => Direction::NW,
			_ => panic!("Direction not valid"),
		}
	}
}
