
use std::collections::BTreeMap;

#[allow(unused_imports)]
use aoch::{AoCDay, DayPart, daystr, run_test, test_runner};

use crate::intcode::{Intcode, RunResult, ICInt};
use crate::rendering;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Color {
	#[default]
	Black = 0,
	White = 1,
}

#[derive(Debug)]
enum Direction {
	Up,
	Right,
	Down,
	Left,
}
impl Direction {
	fn turn_left(&self) -> Direction {
		match self {
			Direction::Up => Direction::Left,
			Direction::Left => Direction::Down,
			Direction::Down => Direction::Right,
			Direction::Right => Direction::Up,
		}
	}
	fn turn_right(&self) -> Direction {
		match self {
			Direction::Up => Direction::Right,
			Direction::Left => Direction::Up,
			Direction::Down => Direction::Left,
			Direction::Right => Direction::Down,
		}
	}
	fn advance(&self, pos: &mut (isize, isize)) {
		match self {
			Direction::Up => pos.1 += 1,
			Direction::Down => pos.1 -= 1,
			Direction::Left => pos.0 -= 1,
			Direction::Right => pos.0 += 1,
		}
	}
}

#[derive(Debug)]
pub struct Mapper {
	prog: Intcode,
	pos: (isize, isize),
	dir: Direction,
	map: BTreeMap<(isize, isize), Color>
}
impl Mapper {
	pub fn new(prog: Intcode) -> Mapper {
		Mapper {
			prog,
			pos: (0, 0),
			dir: Direction::Up,
			map: BTreeMap::new(),
		}
	}
	fn run(&mut self) {
		loop {
			match self.prog.run() {
				bad @ (RunResult::RepeatedState | RunResult::InvalidInstruction(_)) => panic!("{:?}", bad),
				RunResult::Halted => {
					self.process_output();
					break;
				},
				RunResult::Starved => {
					self.process_output();
					self.prog.input.push(
						*self.map.get(&self.pos)
							.unwrap_or(&Color::Black)
							as ICInt
					);

				}
			}
		}
	}
	fn process_output(&mut self) {
		assert!(self.prog.output.len() % 2 == 0, "program did not output instruction pairs");
		for [col, dir] in self.prog.output.array_windows().copied() {
			let tile_color = self.map.entry(self.pos).or_default();

			match col {
				0 => *tile_color = Color::Black,
				1 => *tile_color = Color::White,
				_ => panic!("unknown color code: {:?}", col),
			}
			match dir {
				0 => self.dir = self.dir.turn_left(),
				1 => self.dir = self.dir.turn_right(),
				_ => panic!("unknown direction code: {:?}", dir),
			}

			self.dir.advance(&mut self.pos);
		}

		self.prog.output.clear();
	}
	fn reset(&mut self, starting_color: Color) {
		self.prog.reset();
		self.map.clear();
		self.pos = (0, 0);
		self.dir = Direction::Up;

		self.map.insert((0, 0), starting_color);
	}
	fn render(&self, unset: char, set: char, newline: bool) -> String {
		let (xmin, xmax, ymin, ymax) = {
			let mut coords = self.map.iter()
				.filter(|&(_, c)| *c == Color::White)
				.map(|(k, _)| *k);
			let first = coords.next().unwrap();
			let (mut xmin, mut xmax) = (first.0, first.0);
			let (mut ymin, mut ymax) = (first.1, first.1);
			for (x, y) in coords {
				if x < xmin { xmin = x; } else if x > xmax { xmax = x; }
				if y < ymin { ymin = y; } else if y > ymax { ymax = y; }
			}
			(xmin, xmax + 1, ymin, ymax)
		};

		let width = xmax.abs_diff(xmin);
		let height = ymax.abs_diff(ymin);

		if rendering::DEBUG_OUTPUT {
			eprintln!("X: min={}, max={}, len={}", xmin, xmax, width);
			eprintln!("Y: min={}, max={}, len={}", ymin, ymax, height);
		}
		
		let mut canvas = String::with_capacity((width+1) * height);

		for y in (ymin..=ymax).rev() {
			for x in xmin..=xmax {
				let color = self.map.get(&(x, y)).copied().unwrap_or_default();
				canvas.push(match color {
					Color::Black => unset,
					Color::White => set,
				});
			}
			if newline { canvas.push('\n'); }
		}

		canvas
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Day11;

impl AoCDay for Day11 {
	type Data<'i> = Mapper;
	type Answer = String;
	fn day(&self) -> u8 { 11 }
	fn parse<'i>(&self, input: &'i str) -> Self::Data<'i> {
		Mapper::new(Intcode::parse(input))
	}
	fn part1(&self, _data: &mut Self::Data<'_>) -> Self::Answer {
		_data.reset(Color::Black);
		_data.run();
		_data.map.len().to_string()
	}
	fn part2(&self, _data: &mut Self::Data<'_>) -> Self::Answer {
		_data.reset(Color::White);
		_data.run();
		eprintln!("{}", _data.render(' ', '#', true));
		rendering::parse(_data.render('0', '1', false))
	}
}

#[test]
fn part1() {
	let cases = [
		// (TEST_INPUT, 0),
		(daystr!("11"), "1709"),
	];
	test_runner::<_, _>(Day11, DayPart::Part1, &cases);
}
#[test]
fn part2() {
	let cases = [
		// (TEST_INPUT, 0),
		(daystr!("11"), "PGUEHCJH"),
	];
	test_runner::<_, _>(Day11, DayPart::Part2, &cases);
}
