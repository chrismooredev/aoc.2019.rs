
use std::collections::HashSet;
use std::ops::Range;
use std::str::FromStr;
use itertools::Itertools;
use std::slice::Iter;
use std::iter::Copied;

#[allow(unused_imports)]
use aoch::{AoCDay, DayPart, daystr, run_test, test_runner};
use num::ToPrimitive;
use smallvec::SmallVec;

#[derive(Debug)]
pub struct AsteroidField<'a> {
	raw: &'a str,
	rows: Vec<&'a [u8]>,
	width: usize,
	height: usize,
	slopes_cache: Vec<(usize, usize)>,
}
impl<'a> AsteroidField<'a> {
	pub fn parse(s: &'a str) -> Self {
		let lines: Vec<&'a [u8]> = s.lines()
			.filter_map(aoch::parsing::trimmed)
			.map(str::as_bytes)
			.collect();

		assert!(lines.len() > 0);
		let width = lines[0].len();
		for (i, l) in lines[1..].iter().enumerate() {
			assert_eq!(l.len(), width, "line {} does not match expected length {}", i+1, width);
		}

		AsteroidField {
			width, height: lines.len(),
			slopes_cache: Self::slopes_for_size(width, lines.len()),
			raw: s, rows: lines,
		}
	}
	fn find_best(&self) -> Option<(usize, usize)> {
		let mut best = None;
		for (x, y) in self.asteroids() {
			let count = self.visible_from(x, y).count();
			eprintln!("asteriod ({:>2},{:>2}) found {} visible others", x, y, count);
			match best {
				None => { best = Some((x, y, count)); },
				Some((_, _, oc)) if count > oc  => { best = Some((x, y, count)); },
				_ => continue,
			}
		}
		dbg!(best);
		best.map(|(x, y, _c)| (x, y))
	}
	pub fn exists(&self, x: usize, y: usize) -> Option<bool> {
		let raw = self.rows.get(y).map(|r| r.get(x)).flatten();
		// if let Some(c) = raw {
		// 	eprintln!("[DIM=({:>2},{:>2})][REQ=({:>2},{:>2})] found={} (row={:?})", self.width, self.height, x, y, *c as char, self.rows.get(y));
		// } else {
		// 	eprintln!("[DIM=({:>2},{:>2})][REQ=({:>2},{:>2})] found={} (row={:?})", self.width, self.height, x, y, "<OOB>", self.rows.get(y));
		// }
		match raw? {
			b'.' => Some(false),
			b'#' => Some(true),
			c => panic!("bad char: {:?}", c),
		}
	}
	pub fn exists_signed(&self, x: isize, y: isize) -> Option<bool> {
		self.exists(x.to_usize()?, y.to_usize()?)
	}

	fn asteroids(&self) -> AsteroidPositions<'_> {
		AsteroidPositions { field: self, last: None }
	}
	pub fn visible_from(&self, x: usize, y: usize) -> impl Iterator<Item = (usize,usize)> + '_ {
		// let (x, y) = (x, y);
		self.slopes_cache.iter().copied()
			.flat_map(SlopeRotator2::new)
			// .flatten()
			.filter_map(move |sl| {
				let found = self.find_visible((x, y), sl);
				eprintln!("({:>2},{:>2}) * i*({:>2},{:>2}) -> {:?}", x, y, sl.0, sl.1, found);
				found
			})
			// .filter_map(move |sl| self.find_visible((x, y), sl))
	}
	// fn find_visible(&self, around: (usize, usize), slope: (isize, isize)) -> impl Iterator<Item = (usize, usize)> {
	// 	LineOfSightIter { field: self, ox: x, oy: y }
	// }

	fn render_counts(&self) -> String {
		let mut out = String::with_capacity((self.width + 1) * self.height);
		for (y, row) in self.rows.iter().enumerate() {
			for (x, c) in row.iter().enumerate() {
				match *c {
					b'.' => out.push('.'),
					b'#' => {
						let cnt = self.slopes_cache.iter().copied()
							.map(SlopeRotator::new)
							.flatten()
							.filter_map(move |sl| self.find_visible((x,y), sl))
							.count();
						let s = cnt.to_string();
						if s.len() > 1 {
							out.push_str(&format!("({})", s));
						} else {
							out.push_str(&s);
						}
					},
					c => panic!("unknown char: {:?}", c),
				};
			}
			out.push('\n');
		}
		out
	}

	fn find_visible(&self, around: (usize, usize), slope: (isize, isize)) -> Option<(usize, usize)> {
		let (ax, ay) = (around.0 as isize, around.1 as isize);
		let (sx, sy) = slope;

		if sx == 0 && sy == 0 {
			// eprintln!("\tvisiting ({:>2},{:>2}): {:?}", around.0, around.1, self.exists(around.0, around.1));
			eprintln!("({:>2},{:>2}) + i*({:>2},{:>2}) => ({:>2},{:>2}) = {:?}", ax, ay, sx, sy, ax, ay, self.exists_signed(ax, ay));
			return self.exists_signed(ax, ay)?.then(|| around);
		}
		let (mut cx, mut cy) = (sx, sy);
		for i in 1.. {
			let (x, y) = (ax + cx, ay + cy);
			eprintln!("({:>2},{:>2}) + {}*({:>2},{:>2}) => ({:>2},{:>2}) = {:?}", ax, ay, i, sx, sy, x, y, self.exists_signed(x, y));
			// eprintln!("\tvisiting ({:>2},{:>2}): {:?}", x, y, self.exists_signed(x, y));
			if self.exists_signed(x, y)? {
				return Some((x as usize, y as usize));
			}
			(cx, cy) = (cx + sx, cy + sy);
		}
		unreachable!()
	}

// ** BAD POINT ROTATION LOGIC?

	// precache slopes to amortize cost of GCD calculation for each cell
	fn slopes_for_size(width: usize, height: usize) -> Vec<(usize, usize)> {
		assert!(width > 0);
		assert!(height > 0);

		// seems to be ~0.6 of capacity, lets do .75
		let mut buf = Vec::with_capacity(width * height * 3 / 4);

		buf.push((0,0));
		// buf.push((0,1));
		buf.push((1,0));
		buf.push((1,1));
		for x in 2..width {
			buf.push((x, 1));
		}
		for y in 2..height {
			buf.push((1, y));
		}
		for x in 2..width {
			for y in 2..height {
				if num::integer::gcd(x, y) == 1 {
					buf.push((x, y));
				}
			}
		}

		buf.sort();
		debug_assert_eq!(buf, {
			let mut b2 = buf.clone();
			b2.sort();
			b2.dedup();
			b2
		});

		/*
def yieldthem(dim):
	yield (0,1)
	yield (1,0)
	yield (1,1)
	for x in range(0,dim[0]):
		yield (x,1)
	for y in range(0,dim[1]):
		yield (1,y)
	for x in range(0,dim[0]):
		for y in range(0,dim[1]):
			if math.gcd(x,y) == 1:
				yield(x,y)

		*/

		// TODO: better cache line usage?
		// buf.sort_by_key(|(x,y)| (y,x));
		buf
	}
}

#[derive(Debug)]
struct AsteroidPositions<'a> {
	field: &'a AsteroidField<'a>,
	last: Option<(usize, usize)>,
}
impl<'a> Iterator for AsteroidPositions<'a> {
	type Item = (usize, usize);
	fn next(&mut self) -> Option<Self::Item> {
		let (sx, sy) = match self.last {
			None => (0, 0),
			Some((x, y)) if x == self.field.width && y == self.field.height-1 => {
				return None;
			},
			Some((x, y)) if x == self.field.width-1 => {
				(0, y+1)
			},
			Some((x, y)) => (x+1, y),
		};
		for (y, row) in self.field.rows.iter().enumerate().skip(sy) {
			for (x, asteroid) in row.iter().enumerate().skip(sx) {
				match asteroid {
					b'.' => continue,
					b'#' => {
						self.last = Some((x, y));
						return self.last;
					},
					_ => panic!("unexpected asteroid char at ({}, {}): {:?}", x, y, asteroid),
				}
			}
		}
		self.last = Some((self.field.width, self.field.height-1));
		None
	}
}

#[derive(Debug,Clone)]
struct SlopeRotator {
	slope: (usize, usize),
	state: SlopeRotatorState
}

struct SlopeRotator2 {
	slope: (usize, usize),
	elements: SmallVec<[(isize, isize); 4]>,
}
impl SlopeRotator2 {
	fn new(raw: (usize, usize)) -> SlopeRotator2 {
		let (x, y) = (raw.0 as isize, raw.1 as isize);
		let mut sv = SmallVec::new();		

		sv.push((x, y));
		sv.push((x, -y));
		sv.push((-x, y));
		sv.push((-x, -y));

		sv.dedup();
		eprintln!("new slope rotator for ({:>2},{:>2}) -> {:?}", x, y, sv);
		sv.reverse();

		SlopeRotator2 { slope: raw, elements: sv }
	}
}
impl Iterator for SlopeRotator2 {
	type Item = (isize, isize);
	fn next(&mut self) -> Option<Self::Item> {
		self.elements.pop()
	}
}
// impl<'a> IntoIterator for &'a SlopeRotator2 {
// 	type Item = (isize, isize);
// 	type IntoIter = Copied<Iter<'a, (isize, isize)>>;
// 	fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
// 		self.elements.iter().copied()
// 	}
// }

#[derive(Debug,Clone)]
enum SlopeRotatorState {
	PosPos,
	PosNeg,
	NegPos,
	NegNeg,
	Done,
}
impl SlopeRotator {
	fn new(raw: (usize, usize)) -> SlopeRotator {
		SlopeRotator { slope: raw, state: SlopeRotatorState::PosPos }
	}
}
impl Iterator for SlopeRotator {
	type Item = (isize, isize);
	fn next(&mut self) -> Option<Self::Item> {
		let mut og_state = self.state.clone();
		let mul = match (&self.state, self.slope) {
			(SlopeRotatorState::PosPos, (0, 0)) => {
				self.state = SlopeRotatorState::Done;
				og_state.next()?
			},
			(SlopeRotatorState::PosPos, (_, 0)) => {
				self.state = SlopeRotatorState::NegPos;
				og_state.next()?
			},
			(SlopeRotatorState::NegPos, (_, 0)) => {
				self.state = SlopeRotatorState::Done;
				og_state.next()?
			},
			(SlopeRotatorState::PosNeg, (0, _)) => {
				self.state = SlopeRotatorState::Done;
				og_state.next()?
			},
			(_, _) => {
				self.state = og_state;
				self.state.next()?
			}
		};
		Some((
			self.slope.0 as isize * mul.0,
			self.slope.1 as isize * mul.1
		))
	}
}
impl Iterator for SlopeRotatorState {
	type Item = (isize, isize);
	fn next(&mut self) -> Option<Self::Item> {
		use SlopeRotatorState::*;
		match self {
			PosPos => { *self = PosNeg; Some((1, 1)) },
			PosNeg => { *self = NegPos; Some((1, -1)) },
			NegPos => { *self = NegNeg; Some((-1, 1)) },
			NegNeg => { *self = Done; Some((-1, -1)) },
			Done => None,
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Day10;

impl AoCDay for Day10 {
	type Data<'i> = AsteroidField<'i>;
	type Answer = usize;
	fn day(&self) -> u8 { 10 }
	fn parse<'i>(&self, input: &'i str) -> Self::Data<'i> {
		AsteroidField::parse(input)
	}
	fn part1(&self, _data: &mut Self::Data<'_>) -> Self::Answer {
		todo!("Day {} Part 1", Self::day(&Self));
	}
	fn part2(&self, _data: &mut Self::Data<'_>) -> Self::Answer {
		todo!("Day {} Part 2", Self::day(&Self));
	}
}

const EXAMPLES: &[((usize, usize), &str)] = &[
	((3, 4), "
.#..#
.....
#####
....#
...##"),
];

#[test]
fn unrotated_coordinate_visits() {
	let field = AsteroidField::parse(EXAMPLES[0].1);
	let slopes = AsteroidField::slopes_for_size(field.width, field.height);
	panic!("{:?}", slopes);
}

// fn rotated_coordinate_visits() {
// 	let field = AsteroidField::parse(EXAMPLES[0].1);
// 	let slopes = AsteroidField::slopes_for_size(field.width, field.height);

// 	for sl in slopes.iter() {
// 		for slr in SlopeRotator::new(sl) {
// 			let found = field.find_visible((around), slope)
// 		}
// 	}
// 	slopes.iter().copied()
// 		.map(SlopeRotator::new)
// 		.flatten()
// 		.map(move |sl| (sl, self.find_visible((x, y), sl)))
// 		.inspect(move |o| eprintln!("asteroid ({:>2},{:>2}) found ({:>2},{:>2}) visible: {:?}", x, y, o.0.0, o.0.1, o.1))
// 		.filter_map(|o| o.1)
// }

#[test]
fn asteroids_positions() {
	let field = AsteroidField::parse(EXAMPLES[0].1);
	let pos: Vec<(usize, usize)> = field.asteroids().collect();
	let expected: &[(usize, usize)] = &[
		(1, 0), (4, 0),
		(0, 2), (1, 2), (2, 2), (3, 2), (4, 2),
		(4, 3),
		(3, 4), (4, 4),
	];
	assert_eq!(expected, &pos);
}

#[test]
fn asteroids_visible_from() {
	let field = AsteroidField::parse(EXAMPLES[0].1);
	let mut pos: Vec<(usize, usize)> = field.visible_from(1, 0)
		.collect();
	pos.sort();
	let expected: &mut [(usize, usize)] = &mut [
		(1, 0), (4, 0),
		(0, 2), (1, 2), (2, 2), (3, 2), (4, 2),
		(4, 4),
	];
	expected.sort();
	let pos_set: HashSet<(usize, usize)> = pos.iter().copied().collect();
	let expected_set: HashSet<(usize, usize)> = expected.iter().copied().collect();
	assert_eq!(&expected_set, &pos_set, "unordered set was bad (generated ordered: {:?})", pos);
	assert_eq!(expected, &pos, "generated order bad (repeated elements?)");
}

#[test]
fn find_positions() {
	
	for (exp, inp) in EXAMPLES.iter() {
		let mut d = Day10::parse(&Day10, inp);
		eprintln!("{:?}", d);
		let asts = d.asteroids().collect::<Vec<_>>();
		eprintln!("asteriods: (len={}) {:?}", asts.len(), asts);
		let coords = d.find_best();
		if Some(*exp) != coords {
			panic!("expected coordinates ({:?}), found ({:?}).\n{}", *exp, coords, d.render_counts());
		}
		assert_eq!(Some(*exp), coords);
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
