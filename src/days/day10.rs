
#[allow(unused_imports)]
use aoch::{AoCDay, DayPart, daystr, run_test, test_runner};

#[derive(Debug, Clone, Copy)]
pub struct Day10;

impl AoCDay for Day10 {
	type Data<'i> = Vec<u8>;
	type Answer = usize;
	fn day(&self) -> u8 { 10 }
	fn parse<'i>(&self, input: &'i str) -> Self::Data<'i> {
		input.split('\n')
			.filter_map(aoch::parsing::trimmed)
			.map(|n| n.parse::<u8>().unwrap())
			.collect()
	}
	fn part1(&self, _data: &mut Self::Data<'_>) -> Self::Answer {
		todo!("Day {} Part 1", Self::day(&Self));
	}
	fn part2(&self, _data: &mut Self::Data<'_>) -> Self::Answer {
		todo!("Day {} Part 2", Self::day(&Self));
	}
}

#[test]
fn part1() {
	let cases = [
		// (TEST_INPUT, 0),
		(daystr!("10"), 0),
	];
	test_runner::<_, _>(Day10, DayPart::Part1, &cases);
}
#[test]
fn part2() {
	let cases = [
		// (TEST_INPUT, 0),
		(daystr!("10"), 0),
	];
	test_runner::<_, _>(Day10, DayPart::Part2, &cases);
}
