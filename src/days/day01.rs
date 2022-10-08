
#[allow(unused_imports)]
use aoch::{AoCDay, DayPart, daystr, run_test, test_runner};

#[derive(Debug, Clone, Copy)]
pub struct Day01;

fn calc_fuel(mass: usize) -> usize {
	(mass / 3).saturating_sub(2)
}

fn calc_needed_fuel(mass: usize) -> usize {
	let mut total = 0;
	let mut nmass = mass;
	while nmass > 0 {
		nmass = calc_fuel(nmass);
		total += nmass;
	}
	total
}

impl AoCDay for Day01 {
	type Data<'i> = Vec<usize>;
	type Answer = usize;
	fn day(&self) -> u8 { 01 }
	fn parse<'i>(&self, input: &'i str) -> Self::Data<'i> {
		input.split('\n')
			.filter_map(aoch::parsing::trimmed)
			.map(|n| n.parse::<usize>().unwrap())
			.collect()
	}
	fn part1(&self, _data: &mut Self::Data<'_>) -> Self::Answer {
		_data.iter()
			.copied()
			.map(calc_fuel)
			.sum()
	}
	fn part2(&self, _data: &mut Self::Data<'_>) -> Self::Answer {
		_data.iter()
			.copied()
			.map(calc_needed_fuel)
			.sum()
	}
}

#[test]
fn test_fuel() {
	const FUELS: &[(usize, usize)] = &[
		(12, 2),
		(14, 2),
		(1969, 654),
		(100756, 33583),
	];
	FUELS.iter()
		.enumerate()
		.for_each(|(i, (mass, fuel))| assert_eq!(*fuel, calc_fuel(*mass), "fuel #{} produced bad value", i));
}

#[test]
fn test_total_fuel() {
	const FUELS: &[(usize, usize)] = &[
		(14, 2),
		(1969, 966),
		(100756, 50346),
	];
	FUELS.iter()
		.enumerate()
		.for_each(|(i, (mass, fuel))| assert_eq!(*fuel, calc_needed_fuel(*mass), "fuel #{} produced bad value", i));
}

#[test]
fn part1() {
	let cases = [
		// (TEST_INPUT, 0),
		(daystr!("01"), 3317668),
	];
	test_runner::<_, _>(Day01, DayPart::Part1, &cases);
}
#[test]
fn part2() {
	let cases = [
		// (TEST_INPUT, 0),
		(daystr!("01"), 4973628),
	];
	test_runner::<_, _>(Day01, DayPart::Part2, &cases);
}
