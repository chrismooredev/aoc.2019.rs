
#[allow(unused_imports)]
use aoch::{AoCDay, DayPart, daystr, run_test, test_runner};

use crate::intcode::*;

#[derive(Debug, Clone, Copy)]
pub struct Day05;

impl AoCDay for Day05 {
	type Data<'i> = Intcode;
	type Answer = ICInt;
	fn day(&self) -> u8 { 05 }
	fn parse<'i>(&self, input: &'i str) -> Self::Data<'i> {
		Intcode::parse(input)
	}
	fn part1(&self, _data: &mut Self::Data<'_>) -> Self::Answer {
		let mut cpu = _data.clone();
		cpu.input.push(1);
		assert_eq!(cpu.run(), RunResult::Halted);
		cpu.output.pop().unwrap()
	}
	fn part2(&self, _data: &mut Self::Data<'_>) -> Self::Answer {
		let mut cpu = _data.clone();
		cpu.input.push(5);
		assert_eq!(cpu.run(), RunResult::Halted);
		cpu.output.pop().expect("CPU did not output any values")
	}
}

#[test]
fn part1() {
	let cases = [
		(daystr!("05"), 9961446),
	];
	test_runner::<_, _>(Day05, DayPart::Part1, &cases);
}
#[test]
fn part2() {
	let cases = [
		(daystr!("05"), 742621),
	];
	test_runner::<_, _>(Day05, DayPart::Part2, &cases);
}
