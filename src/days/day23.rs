
#[allow(unused_imports)]
use aoch::{AoCDay, DayPart, daystr, run_test, test_runner};

#[derive(Debug, Clone, Copy)]
pub struct Day23;

impl AoCDay for Day23 {
	type Data = Vec<u8>;
	type Answer = usize;
	fn day(&self) -> u8 { 23 }
	fn parse(&self, input: &str) -> Self::Data {
		input.split('\n')
			.filter_map(aoch::parsing::trimmed)
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
		(daystr!("23"), 0),
	];
	test_runner::<_, _>(Day23, DayPart::Part1, &cases);
}
#[test]
fn part2() {
	let cases = [
		// (TEST_INPUT, 0),
		(daystr!("23"), 0),
	];
	test_runner::<_, _>(Day23, DayPart::Part2, &cases);
}
