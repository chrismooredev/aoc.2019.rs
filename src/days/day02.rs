
#[allow(unused_imports)]
use aoch::{AoCDay, DayPart, daystr, run_test, test_runner};

#[derive(Debug, Clone, Copy)]
pub struct Day02;

type ICInt = i32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Intcode {
	pc: isize,
	stepped: usize,
	ram: Vec<ICInt>,
	original: Vec<ICInt>,
}
impl Intcode {
	fn new(ram: Vec<ICInt>) -> Intcode {
		Intcode {
			pc: 0,
			stepped: 0,
			original: ram.clone(),
			ram
		}
	}
	fn reset(&mut self) {
		self.ram = self.original.clone();
		self.pc = 0;
		self.stepped = 0;
	}
}

impl Intcode {
	fn step(&mut self) {
		let pc = self.pc as usize;
		match self.ram[pc] { // add [a] [b] [out]
			99 => { // hlt
				return;
			},
			1 => {
				let in_a = self.ram[self.ram[pc+1] as usize];
				let in_b = self.ram[self.ram[pc+2] as usize];
				let out_ptr = self.ram[pc+3] as usize;
				self.ram[out_ptr] = in_a + in_b;
				self.pc += 4;
			},
			2 => { // mul [a] [b] [out]
				let in_a = self.ram[self.ram[pc+1] as usize];
				let in_b = self.ram[self.ram[pc+2] as usize];
				let out_ptr = self.ram[pc+3] as usize;
				self.ram[out_ptr] = in_a * in_b;
				self.pc += 4;
			},
			_ => panic!("unexpected opcode enountered @ PC={} after {} steps: {}", self.pc, self.stepped, self.ram[pc]),
		}
		self.stepped += 1;
	}
	fn run(&mut self) {
		while self.ram[self.pc as usize] != 99 { // while not halted
			self.step()
		}
	}
}

impl AoCDay for Day02 {
    type Data = Intcode;
    type Answer = ICInt;
    fn day(&self) -> u8 { 02 }
    fn parse(&self, input: &str) -> Self::Data {
        let v: Vec<ICInt> = input.split(',')
			.filter_map(aoch::parsing::trimmed)
            .map(|n| n.parse::<_>().unwrap())
            .collect();
		Intcode::new(v)
    }
    fn part1(&self, _data: &mut Self::Data) -> Self::Answer {
		if _data.ram.len() > 50 {
			// working on the 'real input'
			_data.ram[1] = 12;
			_data.ram[2] = 2;
		}
		_data.run();
		_data.ram[0]
    }
    fn part2(&self, _data: &mut Self::Data) -> Self::Answer {
		for noun in 0..=99 {
			for verb in 0..=99 {
				_data.reset();
				_data.ram[1] = noun;
				_data.ram[2] = verb;
				_data.run();
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
		ic.run();
		(ic.pc, ic.stepped, ic.ram)
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
