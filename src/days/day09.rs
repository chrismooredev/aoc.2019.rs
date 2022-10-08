
#[allow(unused_imports)]
use aoch::{AoCDay, DayPart, daystr, run_test, test_runner};

use crate::intcode::{Intcode, RunResult, ICInt};

#[derive(Debug, Clone, Copy)]
pub struct Day09;

impl AoCDay for Day09 {
	type Data<'i> = Intcode;
	type Answer = ICInt;
	fn day(&self) -> u8 { 09 }
	fn parse<'i>(&self, input: &'i str) -> Self::Data<'i> {
		Intcode::parse(input)
	}
	fn part1(&self, _data: &mut Self::Data<'_>) -> Self::Answer {
		_data.reset();
		_data.input.push(1);
		assert_eq!(_data.run(), RunResult::Halted);
		assert_eq!(_data.output.len(), 1);
		_data.output.pop().unwrap()
	}
	fn part2(&self, _data: &mut Self::Data<'_>) -> Self::Answer {
		_data.reset();
		_data.input.push(2);
		assert_eq!(_data.run(), RunResult::Halted);
		assert_eq!(_data.output.len(), 1);
		_data.output.pop().unwrap()
	}
}

#[test]
fn part1() {
	let cases = [
		// (TEST_INPUT, 0),
		(daystr!("09"), 3512778005),
	];
	test_runner::<_, _>(Day09, DayPart::Part1, &cases);
}
#[test]
fn part2() {
	let cases = [
		// (TEST_INPUT, 0),
		(daystr!("09"), 35920),
	];
	test_runner::<_, _>(Day09, DayPart::Part2, &cases);
}
