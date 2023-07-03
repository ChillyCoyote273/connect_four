#![allow(dead_code)]

use crate::game::Game;
use std::{convert::From, iter::FromIterator, fmt::Display};

const BOTTOM: u64 = 0b1000000100000010000001000000100000010000001;
const WIDTH: u8 = 7;
const HEIGHT: u8 = 6;
const BOARD_HEIGHT: u8 = 7;
const TOTAL_SIZE: u8 = WIDTH * HEIGHT;

#[derive(Default, Debug, Clone, Copy)]
pub struct Bitboard {
    position: u64,
    mask: u64,
}

impl From<Game> for Bitboard {
    fn from(value: Game) -> Self {
        let (position, mask) = value.get_bitboards();
        Self { position, mask }
    }
}

impl FromIterator<char> for Bitboard {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        iter.into_iter()
            .filter_map(|c| c.to_digit(10))
            .filter_map(|d| match d {
                1..=7 => Some(d as u8 - 1),
                _ => None,
            })
            .fold(Self::new(), |acc, x| {
                if acc.can_play(x) {
                    let mut new = acc;
                    new.play(x);
                    new
                } else {
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

impl Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!( 
            f,
            "\n---------------\n{}\n---------------\n{}\n---------------\n{}\n---------------\n{}\n---------------\n{}\n---------------\n{}\n---------------\n",
            format!("|{}|{}|{}|{}|{}|{}|{}|", self.get_tile(0, 5), self.get_tile(1, 5), self.get_tile(2, 5), self.get_tile(3, 5), self.get_tile(4, 5), self.get_tile(5, 5), self.get_tile(6, 5)),
            format!("|{}|{}|{}|{}|{}|{}|{}|", self.get_tile(0, 4), self.get_tile(1, 4), self.get_tile(2, 4), self.get_tile(3, 4), self.get_tile(4, 4), self.get_tile(5, 4), self.get_tile(6, 4)),
            format!("|{}|{}|{}|{}|{}|{}|{}|", self.get_tile(0, 3), self.get_tile(1, 3), self.get_tile(2, 3), self.get_tile(3, 3), self.get_tile(4, 3), self.get_tile(5, 3), self.get_tile(6, 3)),
            format!("|{}|{}|{}|{}|{}|{}|{}|", self.get_tile(0, 2), self.get_tile(1, 2), self.get_tile(2, 2), self.get_tile(3, 2), self.get_tile(4, 2), self.get_tile(5, 2), self.get_tile(6, 2)),
            format!("|{}|{}|{}|{}|{}|{}|{}|", self.get_tile(0, 1), self.get_tile(1, 1), self.get_tile(2, 1), self.get_tile(3, 1), self.get_tile(4, 1), self.get_tile(5, 1), self.get_tile(6, 1)),
            format!("|{}|{}|{}|{}|{}|{}|{}|", self.get_tile(0, 0), self.get_tile(1, 0), self.get_tile(2, 0), self.get_tile(3, 0), self.get_tile(4, 0), self.get_tile(5, 0), self.get_tile(6, 0))
        )
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
        0b100000 << (column * BOARD_HEIGHT)
    }

    fn bottom_mask(column: u8) -> u64 {
        1 << (column * BOARD_HEIGHT)
    }

    pub fn position_mask(column: u8, row: u8) -> u64 {
        1 << row << (column * 7)
    }

    pub fn play(&mut self, column: u8) {
        self.position ^= self.mask;
        self.mask |= self.mask + Self::bottom_mask(column);
    }

    pub fn check_win(&self) -> bool {
        let mut m = self.position & self.position >> BOARD_HEIGHT;
        if m & m >> (2 * BOARD_HEIGHT) != 0 {
            return true;
        }

        m = self.position & self.position >> (BOARD_HEIGHT - 1);
        if m & m >> (2 * BOARD_HEIGHT - 2) != 0 {
            return true;
        }

        m = self.position & self.position >> (BOARD_HEIGHT + 2);
        if m & m >> (2 * BOARD_HEIGHT + 2) != 0 {
            return true;
        }

        m = self.position & self.position >> 1;
        if m & m >> 2 != 0 {
            return true;
        }

        false
    }

    pub fn move_count(&self) -> u8 {
        self.mask.count_ones() as u8
    }

    pub fn is_winning_move(&self, column: u8) -> bool {
        let mut other = *self;
        other.play(column);
        other.check_win()
    }

    fn get_tile(&self, column: u8, row: u8) -> char {
        if self.mask & Self::position_mask(column, row) == 0 {
            return ' ';
        }

        let is_current_piece = self.position & Self::position_mask(column, row) != 0;
        let is_first_player_turn = self.move_count() % 2 == 0;

        if is_current_piece == is_first_player_turn {
            'x'
        }
        else {
            'o'
        }
    }
}


pub fn calc_position_score(state: Bitboard) -> (i8, u64) {
    let mut position_counter = 0;
    (negamax(state, &mut position_counter), position_counter)
}

fn negamax(state: Bitboard, position_counter: &mut u64) -> i8 {
    *position_counter += 1;
    
    if state.move_count() == TOTAL_SIZE {
        return 0;
    }

    for column in 0..WIDTH {
        if state.can_play(column) && state.is_winning_move(column) {
            return (TOTAL_SIZE + 1 - state.move_count()) as i8 / 2;
        }
    }

    let mut best_score = -(TOTAL_SIZE as i8);

    for column in 0..WIDTH {
        if state.can_play(column) {
            let mut next = state;
            next.play(column);
            let score = -negamax(next, position_counter);
            best_score = best_score.max(score);
        }
    }

    best_score
}
