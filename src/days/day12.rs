
use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[allow(unused_imports)]
use aoch::{AoCDay, DayPart, daystr, run_test, test_runner};
use itertools::Itertools;
use num::integer::lcm;
use regex::Regex;

#[derive(Debug)]
pub struct MoonSim {
	moons_pos: Vec<[isize; 3]>,
	moons_vel: Vec<[isize; 3]>,
}
impl MoonSim {
	pub fn new(pos: Vec<[isize; 3]>) -> MoonSim {
		MoonSim { 
			moons_vel: vec![[0; 3]; pos.len()],
			moons_pos: pos,
		}
	}

	fn step(&mut self) {
		// apply gravity
		self.moons_pos.iter()
			.enumerate()
			.tuple_combinations()
			.for_each(|((ia, a), (ib, b))| {
				a.iter().zip(b.iter())
					.enumerate()
					.for_each(|(axis, (a, b))| {
						match a.cmp(b) {
							Ordering::Greater => {
								self.moons_vel[ia][axis] -= 1;
								self.moons_vel[ib][axis] += 1;
							},
							Ordering::Less => {
								self.moons_vel[ia][axis] += 1;
								self.moons_vel[ib][axis] -= 1;
							},
							Ordering::Equal => {},
						}
					});
			});
		
		// apply velocity
		self.moons_vel.iter().zip(self.moons_pos.iter_mut())
			.for_each(|(vel, pos)| {
				pos[0] += vel[0];
				pos[1] += vel[1];
				pos[2] += vel[2];
			});
	}

	fn energy(&self) -> usize {
		self.moons_pos.iter().zip(&self.moons_vel)
			.map(|(pos, vel)| {
				let en_pos = pos.iter().copied().map(isize::abs).sum::<isize>() as usize;
				let en_vel = vel.iter().copied().map(isize::abs).sum::<isize>() as usize;
				en_pos * en_vel
			})
			.sum()
	}

	fn hash_axis(&self, axis: usize) -> u64 {
		let mut s = DefaultHasher::new();
		self.moons_pos.iter().zip(&self.moons_vel)
			.enumerate()
			.for_each(|(i, (p, v))| {
				(i, p[axis], v[axis]).hash(&mut s);
			});
		s.finish()
	}

	fn find_repeated_state(&mut self) -> usize {
		let mut states: [HashSet<u64>; 3] = Default::default();
		let mut lcd: [Option<usize>; 3] = Default::default();

		for i in 0.. {
			self.step();
			let mut found = false;
			lcd.iter_mut()
				.enumerate()
				.filter(|(_ax, o)| o.is_none())
				.for_each(|(ax, o)| {
					if ! states[ax].insert(self.hash_axis(ax)) {
						debug_assert!(o.is_none());
						let _ = o.insert(i);
						found = true;
						eprintln!("found repeated state for axis {} at step {}", ax, i);
					}
				});

			if found {
				// check all lcd is Some(_), if so - break
				if lcd.iter().all(Option::is_some) {
					break;
				}
			}
		}

		debug_assert!(lcd.iter().all(Option::is_some));

		let lcm = lcm(lcd[0].unwrap(), lcm(lcd[1].unwrap(), lcd[2].unwrap()));

		lcm
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Day12;

impl AoCDay for Day12 {
	type Data<'i> = Vec<[isize; 3]>;
	type Answer = usize;
	fn day(&self) -> u8 { 12 }
	fn parse<'i>(&self, input: &'i str) -> Self::Data<'i> {
		let line = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();
		input.split('\n')
			.filter_map(aoch::parsing::trimmed)
			.map(|n| {
				let l = line.captures(n).unwrap();
				let mut matches = l.iter();
				let _ = matches.next();
				let x = matches.next().unwrap().unwrap().as_str().parse().unwrap();
				let y = matches.next().unwrap().unwrap().as_str().parse().unwrap();
				let z = matches.next().unwrap().unwrap().as_str().parse().unwrap();
				[x, y, z]
			})
			.collect()
	}
	fn part1(&self, _data: &mut Self::Data<'_>) -> Self::Answer {
		let mut ms = MoonSim::new(_data.clone());
		for _ in 0..1000 {
			ms.step();
		}
		ms.energy()
	}
	fn part2(&self, _data: &mut Self::Data<'_>) -> Self::Answer {
		let mut ms = MoonSim::new(_data.clone());
		ms.find_repeated_state()
	}
}

#[test]
fn part1() {
	let cases = [
		(daystr!("12"), 14780),
	];
	test_runner::<_, _>(Day12, DayPart::Part1, &cases);
}
#[test]
fn part2() {
	let cases = [
		(daystr!("12"), 279751820342592),
	]; 
	test_runner::<_, _>(Day12, DayPart::Part2, &cases);
}
