use crate::game::Game;
use std::{convert::From, iter::FromIterator};

const BOTTOM: u64 = 0b1000000100000010000001000000100000010000001;
const WIDTH: u8 = 7;
const HEIGHT: u8 = 6;
const BOARD_HEIGHT: u8 = 7;

#[derive(Default, Debug, Clone, Copy)]
#[repr(align(0x10))]
pub struct Bitboard {
	position: u64,
	mask: u64
}

impl From<Game> for Bitboard {
	fn from(value: Game) -> Self {
		let (position, mask) = value.get_bitboards();
		Self {
			position,
			mask
		}
	}
}

impl FromIterator<char> for Bitboard {
	fn from_iter<T: IntoIterator<Item=char>>(iter: T) -> Self {
		iter.into_iter()
			.filter_map(|c| c.to_digit(10))
			.filter_map(|d| match d {
				1..=7 => Some(d as u8 - 1),
				_ => None
			})
			.fold(Self::new(), |acc, x| {
				if acc.can_play(x) {
					let mut new = acc;
					new.play(x);
					new
				}
				else {
					acc
				}
			})
	}
}

impl From<&str> for Bitboard {
	fn from(value: &str) -> Self {
		value.chars().collect()
	}
}

impl Bitboard {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn get_unique_repr(&self) -> u64 {
		self.position + self.mask + BOTTOM
	}

	pub fn can_play(&self, column: u8) -> bool {
		self.mask & Self::top_mask(column) == 0
	}

	fn top_mask(column: u8) -> u64 {
		0b100000 << column * BOARD_HEIGHT
	}

	fn bottom_mask(column: u8) -> u64 {
		1 << column * BOARD_HEIGHT
	}

	pub fn play(&mut self, column: u8) {
		self.position ^= self.mask;
		self.mask |= self.mask + Self::bottom_mask(column);
	}

	pub fn alignment(&self) -> bool {
		let mut m = self.position & self.position >> BOARD_HEIGHT;
		if m & m >> 2 * BOARD_HEIGHT != 0 {
			return true;
		}

		m = self.position & self.position >> BOARD_HEIGHT - 1;
		if m & m >> 2 * BOARD_HEIGHT - 2 != 0 {
			return true;
		}

		m = self.position & self.position >> BOARD_HEIGHT + 2;
		if m & m >> 2 * BOARD_HEIGHT + 2 != 0 {
			return true;
		}

		m = self.position & self.position >> 1;
		if m & m >> 2 != 0 {
			return true;
		}

		false
	}
}

pub fn calc_position_score(state: Bitboard, position_counter: &mut u64) -> i8 {
	*position_counter += 1;
	0
}