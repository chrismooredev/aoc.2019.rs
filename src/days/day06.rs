
use std::collections::{HashMap, HashSet};
use std::cell::RefCell;

#[allow(unused_imports)]
use aoch::{AoCDay, DayPart, daystr, run_test, test_runner};

#[derive(Debug, Clone, Copy)]
pub struct Day06;

#[derive(Debug, Default)]
pub struct Orbits {
	child2parent: HashMap<String, String>,
	occ_cache: RefCell<HashMap<String, usize>>,
}
impl Orbits {
	fn orbits(&self, of: &str) -> usize {
		if of == "COM" {
			return 0;
		}

		let parent = {
			let parent = self.child2parent.get(of).expect(&format!("unable to find parent for satellite {:?}", of));
			if let Some(v) = self.occ_cache.borrow().get(parent) {
				return *v;
			}
			parent.clone()
		};

		let res = 1 + self.orbits(&parent);
		self.occ_cache.borrow_mut().insert(parent, res);
		res
	}
	fn path(&self, of: &str) -> Vec<&str> {
		let mut res = Vec::with_capacity(self.orbits(of));
		let (mut this, _parent) = self.child2parent.get_key_value(of).unwrap();
		
		res.push(this.as_str());
		while let Some(kid) = self.child2parent.get(this) {
			this = kid;
			res.push(this.as_str());
		}

		res.reverse();
		res
	}
}

impl AoCDay for Day06 {
	type Data<'i> = Orbits;
	type Answer = usize;
	fn day(&self) -> u8 { 06 }
	fn parse<'i>(&self, input: &'i str) -> Self::Data<'i> {
		let mut orbits: Orbits = Default::default();
		input.split('\n')
			.filter_map(aoch::parsing::trimmed)
			.for_each(|l| {
				let (parent, child) = l.split_once(')').unwrap();
				let prev = orbits.child2parent.insert(child.trim().to_string(), parent.trim().to_string());
				assert!(prev.is_none());
			});
		orbits
	}
	fn part1(&self, _data: &mut Self::Data<'_>) -> Self::Answer {
		_data.child2parent.keys()
			.map(|child| _data.orbits(child))
			.sum()
	}
	fn part2(&self, _data: &mut Self::Data<'_>) -> Self::Answer {
		let orbits_you = _data.path("YOU");
		let orbits_san = _data.path("SAN");
		let orbits_you_set: HashSet<&str> = orbits_you.iter().copied().collect();
		let orbits_san_set: HashSet<&str> = orbits_san.iter().copied().collect();

		// faster? no alloc
		// { orbits_you_set.drain_filter(|k| ! orbits_san_set.contains(k)); }

		let inters: HashSet<&str> = orbits_you_set.intersection(&orbits_san_set).copied().collect();
		// OPT: look up .orbits as a function of index into orbits_you/orbits_san ?
		let gcd = inters.iter()
			.max_by_key(|k| _data.orbits(k))
			.unwrap();
		let gcd_orbits = _data.orbits(gcd);

		debug_assert_eq!(_data.orbits("YOU"), orbits_you.len() - 1);

		// eprintln!("Path(YOU) = (len: {}) {:?}", orbits_you.len(), &orbits_you);
		// eprintln!("Path(SAN) = (len: {}) {:?}", orbits_san.len(), &orbits_san);
		// eprintln!("Set(Path(YOU)) = {:?}", &orbits_you_set);
		// eprintln!("Set(Path(SAN)) = {:?}", &orbits_san_set);
		// eprintln!("Intersection(Set(Path(YOU)), Set(Path(SAN))) = {:?}", inters);
		// eprintln!("GCD: {:?} (Orbits = {})", gcd, gcd_orbits);

		let gcd_to_you = _data.orbits("YOU") - gcd_orbits - 1;
		let gcd_to_san = _data.orbits("SAN") - gcd_orbits - 1;

		// eprintln!("{}->YOU: {}", gcd, gcd_to_you);
		// eprintln!("{}->SAN: {}", gcd, gcd_to_san);

		gcd_to_you + gcd_to_san
	}
}

#[cfg(test)]
const TEST_INPUT_P1: &str = "
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
";

#[cfg(test)]
const TEST_INPUT_P2: &str = "
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN
";

#[test]
fn orbit_counts() {
	let d6 = Day06;
	let orbits = d6.parse(TEST_INPUT_P1);
	assert_eq!(orbits.orbits("D"), 3);
	assert_eq!(orbits.orbits("L"), 7);
	assert_eq!(orbits.orbits("COM"), 0);
}

#[test]
fn orbit_path() {
	let d6 = Day06;
	let orbits = d6.parse(TEST_INPUT_P2);
	assert_eq!(orbits.path("YOU"), vec![
		"COM",
		"B",
		"C",
		"D",
		"E",
		"J",
		"K",
		"YOU",
	]);
}

#[test]
fn part1() {
	let cases = [
		(TEST_INPUT_P1, 42),
		(daystr!("06"), 0),
	];
	test_runner::<_, _>(Day06, DayPart::Part1, &cases);
}
#[test]
fn part2() {
	let cases = [
		(TEST_INPUT_P2, 4),
		(daystr!("06"), 385),
		// 186 too low
		// 187 too low
	];
	test_runner::<_, _>(Day06, DayPart::Part2, &cases);
}
