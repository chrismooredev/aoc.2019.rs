
#[allow(unused_imports)]
use aoch::{AoCDay, DayPart, daystr, run_test, test_runner};

#[derive(Debug, Clone, Copy)]
pub struct Day28;

impl AoCDay for Day28 {
	type Data = Vec<u8>;
	type Answer = usize;
	fn day(&self) -> u8 { 28 }
	fn parse(&self, input: &str) -> Self::Data {
		input.trim().split(',')
			.map(|n| n.parse::<u8>().unwrap())
			.collect()
	}
	fn part1(&self, _data: &mut Self::Data) -> Self::Answer {
		todo!("Day {} Part 1", Self::day(&Self));
	}
	fn part2(&self, _data: &mut Self::Data) -> Self::Answer {
		todo!("Day {} Part 2", Self::day(&Self));
	}
}

#[test]
fn part1() {
	let cases = [
		// (TEST_INPUT, 0),
		(daystr!("28"), 0),
	];
	test_runner::<_, _>(Day28, DayPart::Part1, &cases);
}
#[test]
fn part2() {
	let cases = [
		// (TEST_INPUT, 0),
		(daystr!("28"), 0),
	];
	test_runner::<_, _>(Day28, DayPart::Part2, &cases);
}
