use nannou::prelude::*;
use nannou::color::rgb::Srgb;
use nannou::state::Mouse;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
enum Piece {
	#[default]
	Empty,
	Red,
	Yellow
}

impl Piece {
	fn get_color(&self) -> Srgb<u8> {
		match self {
			Self::Empty => DARKBLUE,
			Self::Red => RED,
			Self::Yellow => YELLOW
		}
	}
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
enum Turn {
	#[default]
	Red,
	Yellow,
	RedWon,
	YellowWon,
	Tie
}

impl Turn {
	fn get_color(&self) -> Srgba<u8> {
		match self {
			Self::Red => {
				let mut color = Srgba::from(RED);
				color.alpha = 64;
				color
			},
			Self::Yellow => {
				let mut color = Srgba::from(YELLOW);
				color.alpha = 255;//96;
				color
			},
			_ => Rgba::from_components((0, 0, 0, 0))
		}
	}

	fn get_piece(&self) -> Piece {
		match self {
			Self::Red => Piece::Red,
			Self::Yellow => Piece::Yellow,
			_ => Piece::Empty
		}
	}

	fn change(&self) -> Self {
		match self {
			Self::Red => Self::Yellow,
			Self::Yellow => Self::Red,
			_ => *self
		}
	}

	fn win(&self) -> Self {
		match self {
			Self::Red => Self::RedWon,
			Self::Yellow => Self::YellowWon,
			_ => *self
		}
	}
}

#[derive(Clone, Debug, Default)]
pub struct Game {
	board: [[Piece; 6]; 7],
	turn: Turn
}

impl Game {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn draw(&self, draw: &Draw, mouse: &Mouse) {
		draw.background().color(BLUE);

		for column in 0..7 {
			for row in 0..6 {
				draw.ellipse()
					.color(self.board[column][row].get_color())
					.radius(50.0)
					.xy(Self::board_to_point(column, row));
			}
		}
		
		let (column, _) = Self::point_to_board(mouse.x, mouse.y);

		let row = self.find_space(column);
		if let Some(row) = row {
			draw.ellipse()
				.color(self.turn.get_color())
				.radius(50.0)
				.xy(Self::board_to_point(column, row));
		}
	}

	fn find_space(&self, column: usize) -> Option<usize> {
		for row in 0..6 {
			if self.board[column][row] == Piece::Empty {
				return Some(row)
			}
		}
		None
	}

	fn board_to_point(column: usize, row: usize) -> Vec2 {
		Vec2::new(column as f32 * 125.0 - 375.0, row as f32 * 125.0 - 312.5)
	}

	fn point_to_board(x: f32, y: f32) -> (usize, usize) {
		(((x / 125.0 + 3.5) as usize).min(6), ((y / 125.0 + 3.0) as usize).min(5))
	}

	pub fn handle_click(&mut self, mouse: &Mouse) {
		let (column, _) = Self::point_to_board(mouse.x, mouse.y);

		self.make_move(column)
	}

	pub fn make_move(&mut self, column: usize) {
		let row = self.find_space(column);
		if let Some(row) = row {
			self.board[column][row] = self.turn.get_piece();

			self.check_game_over(column, row);

			self.turn = self.turn.change()
		}
	}

	fn check_game_over(&mut self, column: usize, row: usize) -> bool {
		if self.board[column][row] == Piece::Empty {
			return false;
		}

		for i in 0..4 {
			if self.check_both_directions(column, row, i) {
				self.turn = self.turn.win();
				return true;
			}
		}

		for column in 0..7 {
			for row in 0..6 {
				if self.board[column][row] == Piece::Empty {
					return false;
				}
			}
		}

		self.turn = Turn::Tie;
		true
	}

	fn check_both_directions(&self, column: usize, row: usize, direction: usize) -> bool {
		let forward = match direction {
			0 => (1, 0),
			1 => (1, 1),
			2 => (0, 1),
			_ => (-1, 1)
		};

		self.count_direction(column as i32, row as i32, forward.0, forward.1, self.board[column][row]) +
			self.count_direction(column as i32, row as i32, -forward.0, -forward.1, self.board[column][row]) >= 5
	}

	fn count_direction(&self, column: i32, row: i32, delta_column: i32, delta_row: i32, color: Piece) -> usize {
		for i in 1.. {
			let (c, r) = (column + delta_column * i, row + delta_row * i);
			if c < 0 || c > 6 || r < 0 || r > 5 || self.board[c as usize][r as usize] != color {
				return i as usize;
			}
		}
		0
	}

	pub fn is_computer_turn(&self) -> bool {
		self.turn == Turn::Yellow
	}

	pub fn get_bitboards(&self) -> (u64, u64) {
		let mut position = 0;
		let mut mask = 0;
		let current_piece = self.turn.get_piece();

		for column in 0..7 {
			for row in 0..6 {
				if self.board[column][row] != Piece::Empty {
					mask |= Self::position_mask(column as u8, row as u8);
					if self.board[column][row] == current_piece {
						position |= Self::position_mask(column as u8, row as u8);
					}
				}
			}
		}

		(position, mask)
	}

	fn position_mask(column: u8, row: u8) -> u64 {
		1 << row << column * 7
	}
}