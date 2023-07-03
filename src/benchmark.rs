use core::num;
use std::fs;

use crate::computer::{Bitboard, calc_position_score};
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, Default)]
pub struct CaseResult {
	time: f32,
	correct: bool,
	positions_searched: u64
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Case {
	state: Bitboard,
	score: i8
}

impl From<&str> for Case {
	fn from(value: &str) -> Self {
		Self::new(value)
	}
}

impl Case {
	pub fn new(line: &str) -> Self {
		let mut words = line.split(" ");
		Self {
			state: words.next().unwrap().into(),
			score: words.next().unwrap().parse().unwrap()
		}
	}

	pub fn run(self) -> CaseResult {
		let mut positions_searched = 0;
		let start = Instant::now();
		let calculated_score = calc_position_score(self.state, &mut positions_searched);
		let calculation_time = start.elapsed().as_secs_f32();
		CaseResult {
			time: calculation_time,
			correct: calculated_score == self.score,
			positions_searched
		}
	}
}

pub fn run_tests(length: u8, rigour: u8) {
	let mut test_count = 0;
	let (correct, total_time, total_positions_searched, num_tests) = fs::read_to_string(format!("src/tests/Test_L{}_R{}.txt", length, rigour))
		.unwrap()
		.split("\n")
		.map(|line| Case::new(line).run())
		.inspect(|_| { println!("Running test #{}.", test_count); test_count += 1; })
		.fold((true, 0.0, 0, 0), |acc, result| (
			acc.0 && result.correct,
			acc.1 + result.time,
			acc.2 + result.positions_searched,
			acc.3 + 1
		));
	
	let mean_time = total_time / num_tests as f32;
	let mean_positions_searched = total_positions_searched as f32 / num_tests as f32;
	let kilo_positions_per_seconds = (total_positions_searched as f32 / total_time * 1.0e-3).round();
	println!(
		"correct: {correct}\nmean time: {}\nmean positions searched: {mean_positions_searched}\nK pos/s: {kilo_positions_per_seconds}",
		format!(
			"{} s {} ms {} us",
			mean_time,
			mean_time,
			mean_time
		)
	);
}