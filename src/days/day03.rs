
use std::collections::HashMap;

#[allow(unused_imports)]
use aoch::{AoCDay, DayPart, daystr, run_test, test_runner};

#[derive(Debug, Clone, Copy)]
pub struct Day03;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}
impl Direction {
	fn go_from(&self, len: isize, at: (isize, isize)) -> (LineSegment, (isize, isize)) {
		match self {
			Direction::Up => LineSegment::vertical(at.0, at.1, at.1 + len),
			Direction::Down => LineSegment::vertical(at.0, at.1, at.1 - len),
			Direction::Left => LineSegment::horizontal(at.0, at.0 - len, at.1),
			Direction::Right => LineSegment::horizontal(at.0, at.0 + len, at.1),
		}
	}
	fn unit(&self) -> (isize, isize) {
		match self {
			Direction::Up => (0, 1),
			Direction::Down => (0, -1),
			Direction::Left => (-1, 0),
			Direction::Right => (1, 0),
		}
	}
}

#[derive(Debug)]
enum LineSegment {
	Vertical { x: isize, y1: isize, y2: isize },
	Horizontal { x1: isize, x2: isize, y: isize },
}
impl LineSegment {
	fn vertical(x: isize, y1: isize, y2: isize) -> (Self, (isize, isize)) {
		let lower = std::cmp::min(y1, y2);
		let upper = std::cmp::max(y1, y2);
		(
			LineSegment::Vertical { x, y1: lower, y2: upper },
			(x, y2)
		)
	}
	fn horizontal(x1: isize, x2: isize, y: isize) -> (Self, (isize, isize)) {
		let lower = std::cmp::min(x1, x2);
		let upper = std::cmp::max(x1, x2);
		(
			LineSegment::Horizontal { x1: lower, x2: upper, y },
			(x2, y)
		)
	}
	fn intersects(&self, other: &LineSegment) -> bool {
		let mut left = self;
		let mut right = other;
		if matches!(self, LineSegment::Horizontal { .. }) && matches!(other, LineSegment::Vertical { .. }) {
			std::mem::swap(&mut left, &mut right);
		}
		match (self, other) {
			(LineSegment::Vertical { x: sx, y1: sy1, y2: sy2 }, LineSegment::Vertical { x: ox, y1: oy1, y2: oy2 }) => {
				if sx != ox { return false; }
				let mut pairs = [(sy1, sy2), (oy1, oy2)];
				if pairs[1].0 > pairs[0].0 { pairs.swap(0, 1); }
				pairs[1].0 < pairs[0].1
			},
			(LineSegment::Horizontal { x1: sx1, x2: sx2, y: sy }, LineSegment::Horizontal { x1: ox1, x2: ox2, y: oy }) => {
				if sy != oy { return false; }
				let mut pairs = [(sx1, sx2), (ox1, ox2)];
				if pairs[1].0 > pairs[0].0 { pairs.swap(0, 1); }
				pairs[1].0 < pairs[0].1
			},
			(LineSegment::Vertical { x, y1, y2 }, LineSegment::Horizontal { x1, x2, y }) => {
				if x < x1 || x2 < x { return false; }
				if y < y1 || y2 < y { return false; }
				true
			},
			(LineSegment::Horizontal { .. }, LineSegment::Vertical { .. }) => {
				unreachable!()
			},
		}
	}
}

struct Map {
	cursor: (isize, isize),
	steps: usize,
	points: HashMap<(isize, isize), usize>,
}
impl Map {
	fn new() -> Map {
		let mut m = Map {
			cursor: (0, 0),
			steps: 0,
			points: HashMap::with_capacity(200000),
		};
		m.points.insert((0, 0), 0);
		m
	}
	fn mov(&mut self, dir: Direction, length: isize) {
		let (xoff, yoff) = dir.unit();
		for _ in 0..length {
			self.cursor.0 += xoff;
			self.cursor.1 += yoff;
			self.steps += 1;
			self.points.entry((self.cursor.0, self.cursor.1))
				.or_insert(self.steps);
		}
	}
}


impl AoCDay for Day03 {
	type Data<'i> = Vec<Vec<(Direction, isize)>>;
	type Answer = isize;
	fn day(&self) -> u8 { 03 }
	fn parse<'i>(&self, input: &'i str) -> Self::Data<'i> {
		input.split('\n')
			.filter_map(aoch::parsing::trimmed)
			.map(|cmds| {
				let mut cmd_cache = Vec::with_capacity(cmds.len() / 3);
				cmds.split(',')
					.filter_map(aoch::parsing::trimmed)
					.map(|n| {
						let (dir, num) = n.split_at(1);
						let dir = match dir {
							"R" => Direction::Right,
							"L" => Direction::Left,
							"U" => Direction::Up,
							"D" => Direction::Down,
							_ => panic!("bad direction: {}", dir),
						};
						let num = num.parse().expect(&format!("unable to parse string: {:?}", num));
						(dir, num)
					})
					.for_each(|t| cmd_cache.push(t));
					//.collect()
				cmd_cache
			})
			.collect()
	}
	fn part1(&self, _data: &mut Self::Data<'_>) -> Self::Answer {
		let common = _data.iter()
			.map(|lis| {
				let mut m = Map::new();
				lis.iter().for_each(|(dir, len)| m.mov(*dir, *len));
				eprintln!("points len: {}", m.points.len());
				m.points
			})
			.reduce(|mut acc, e| {
				acc.retain(|k, _v| e.contains_key(k));
				acc
			}).unwrap();

		common.keys()
			.map(|(x, y)| x.abs() + y.abs())
			.filter(|v| *v != 0)
			.min().unwrap()
	}
	fn part2(&self, _data: &mut Self::Data<'_>) -> Self::Answer {
		let common: Vec<HashMap<(isize, isize), usize>> = _data.iter()
			.map(|lis| {
				let mut m = Map::new();
				lis.iter().for_each(|(dir, len)| m.mov(*dir, *len));
				m.points
			})
			.collect();
		
		
		let intersections = common[1..].iter()
			.fold(common[0].clone(), |mut acc, e| {
				acc.retain(|k, v| {
					if *v == 0 { return false; }
					match e.get(k) {
						Some(vi) => { *v += vi; true },
						None => false,
					}
				});
				acc
			});

		*intersections.values().min().unwrap() as isize
	}
}

#[cfg(test)]
const TEST_CASE_1: (isize, isize, &str) = (159,610,
"R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"
);

#[cfg(test)]
const TEST_CASE_2: (isize, isize, &str) = (135,410,
"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
);

#[test]
fn part1() {
	let cases = [
		(TEST_CASE_1.2, TEST_CASE_1.0),
		(TEST_CASE_2.2, TEST_CASE_2.0),
		(daystr!("03"), 248),
	];
	test_runner::<_, _>(Day03, DayPart::Part1, &cases);
}
#[test]
fn part2() {
	let cases = [
		(TEST_CASE_1.2, TEST_CASE_1.1),
		(TEST_CASE_2.2, TEST_CASE_2.1),
		(daystr!("03"), 28580),
	];
	test_runner::<_, _>(Day03, DayPart::Part2, &cases);
}
