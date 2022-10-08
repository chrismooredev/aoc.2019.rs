
use crate::intcode::*;

#[allow(unused_imports)]
use aoch::{AoCDay, DayPart, daystr, run_test, test_runner};

#[derive(Debug, Clone, Copy)]
pub struct Day02;


impl AoCDay for Day02 {
    type Data<'i> = Intcode;
    type Answer = ICInt;
    fn day(&self) -> u8 { 02 }
    fn parse<'i>(&self, input: &'i str) -> Self::Data<'i> {
        Intcode::parse(input)
    }
    fn part1(&self, _data: &mut Self::Data<'_>) -> Self::Answer {
		if _data.ram.len() > 50 {
			// working on the 'real input'
			_data.ram[1] = 12;
			_data.ram[2] = 2;
		}
		assert_eq!(_data.run(), RunResult::Halted);
		_data.ram[0]
    }
    fn part2(&self, _data: &mut Self::Data<'_>) -> Self::Answer {
		for noun in 0..=99 {
			for verb in 0..=99 {
				_data.reset();
				_data.ram[1] = noun;
				_data.ram[2] = verb;
				assert_eq!(_data.run(), RunResult::Halted);
				if _data.ram[0] == 19690720 {
					return 100 * noun + verb;
				}
			}
		}
		panic!("no noun/verb combo found that evaluates to 19690720");
    }
}

#[test]
fn few_steps() {
	let cases: [(&[ICInt], _); 4] = [
		(
			&[1,0,0,0,99],
			(4, 1, vec![2,0,0,0,99]),
		),
		(
			&[2,3,0,3,99],
			(4, 1, vec![2,3,0,6,99]),
		),
		(
			&[2,4,4,5,99,0],
			(4, 1, vec![2,4,4,5,99,9801]),
		),
		(
			&[1,1,1,4,99,5,6,0,99],
			(8, 2, vec![30,1,1,4,2,5,6,0,99]),
		)
	];

	run_test(|ram| {
		let mut ic = Intcode::new(ram.to_vec());
		assert_eq!(ic.run(), RunResult::Halted);
		(ic.pc(), ic.stepped(), ic.ram)
	}, &cases);
}

#[test]
fn part1() {
	let cases = [
		("1,1,1,4,99,5,6,0,99", 30),
		("1,9,10,3,2,3,11,0,99,30,40,50", 3500),
		(daystr!("02"), 11590668),
	];
	test_runner::<_, _>(Day02, DayPart::Part1, &cases);
}
#[test]
fn part2() {
	let cases = [
		// (TEST_INPUT, 0),
		(daystr!("02"), 2254),
	];
	test_runner::<_, _>(Day02, DayPart::Part2, &cases);
}
