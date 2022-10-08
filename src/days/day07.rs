
use itertools::Itertools;

#[allow(unused_imports)]
use aoch::{AoCDay, DayPart, daystr, run_test, test_runner};

use crate::intcode::{Intcode, ICInt, RunResult};

#[derive(Debug, Clone, Copy)]
pub struct Day07;

fn run_amps(program: Intcode, initial: ICInt, phases: &[ICInt]) -> ICInt {
	// let mut next_input = initial;
	let mut cpus: Vec<_> = phases.iter()
		.map(|p| {
			let mut ic = program.clone();
			ic.input.push(*p);
			ic
		})
		.collect();

	let cpu_count = cpus.len();
	assert!(cpu_count > 1);

	cpus[0].input.push(initial);
	while ! cpus[cpu_count-1].is_halted() {
		// eprintln!("looping main... {:?}", cpus);
		for i in 0..cpu_count {
			let prev_ind = if i == 0 { cpu_count-1 } else { i-1 };
			while cpus[prev_ind].output.len() > 0 {
				let f = cpus[prev_ind].output.remove(0);
				// eprintln!("[{} -> {}] signal={}", prev_ind, i, f);
				cpus[i].input.push(f);
			}

			let result = cpus[i].run();
			// eprintln!("[{}] RunResult::{:?} -- (inputs={:?}, outputs={:?})", i, result, cpus[i].input, cpus[i].output);
			match result {
				RunResult::Halted | RunResult::Starved => {},
				_ => panic!("execution error: {:?}", result),
			}
			// while cpus[i].output.len() > 0 {
			// 	let f = cpus[i].output.remove(0);
			// 	let next_ind = (i+1) % cpu_count;
			// 	eprintln!("[{} -> {}] signal={}", i, next_ind, f);
			// 	cpus[next_ind].input.push(f);
			// }
		}
	}

	// at end of execution, each CPUs otuput is sent to the next one
	// cpus[0].input.pop().unwrap();
	cpus[cpu_count - 1].output.pop().expect("last CPU halted without output")
	// for (i, p) in phases.iter().enumerate() {
	// 	println!("running phase {} with setting {} and input {}", i, p, next_input);
	// 	program.reset();
	// 	program.input = vec![*p, next_input];
	// 	program.run();
	// 	assert_eq!(program.output.len(), 1);
	// 	next_input = program.output.pop().unwrap();
	// }
	// println!("finished with output {}", next_input);
	// next_input
}

impl AoCDay for Day07 {
	type Data<'i> = Intcode;
	type Answer = ICInt;
	fn day(&self) -> u8 { 07 }
	fn parse<'i>(&self, input: &'i str) -> Self::Data<'i> {
		Intcode::parse(input)
	}
	fn part1(&self, _data: &mut Self::Data<'_>) -> Self::Answer {
		const PHASES: &[ICInt] = &[0,1,2,3,4];
		PHASES.iter().copied()
			.permutations(5)
			.map(|p| {
				let out = run_amps(_data.clone(), 0, p.as_slice());
				(p, out)
			})
			.max_by_key(|(p, out)| *out)
			.map(|(p, out)| out)
			.unwrap()
	}
	fn part2(&self, _data: &mut Self::Data<'_>) -> Self::Answer {
		const PHASES: &[ICInt] = &[5,6,7,8,9];
		PHASES.iter().copied()
			.permutations(5)
			.map(|p| {
				let out = run_amps(_data.clone(), 0, p.as_slice());
				(p, out)
			})
			.max_by_key(|(p, out)| *out)
			.map(|(p, out)| out)
			.unwrap()
	}
}

#[test]
fn amp_phases() {
	let cases: &[((&[ICInt], &[ICInt]), ICInt)] = &[
		((&[4,3,2,1,0], &[3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0]), 43210),
		((&[0,1,2,3,4], &[3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,
			23,1,24,23,23,4,23,99,0,0]), 54321),
		((&[1,0,4,3,2], &[3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,
			33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0]), 65210),
	];
	run_test(|(phases, prog)| {
		run_amps(Intcode::new(prog.to_vec()), 0, phases)
	}, &cases);
}

#[test]
fn part1() {
	let cases = [
		// (TEST_INPUT, 0),
		(daystr!("07"), 67023),
	];
	test_runner::<_, _>(Day07, DayPart::Part1, &cases);
}
#[test]
fn part2() {
	let cases = [
		// (TEST_INPUT, 0),
		(daystr!("07"), 7818398),
	];
	test_runner::<_, _>(Day07, DayPart::Part2, &cases);
}
