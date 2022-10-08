
#[allow(unused_imports)]
use aoch::{AoCDay, DayPart, daystr, run_test, test_runner};

use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Day04;

const WORD_LEN: usize = 6;

#[derive(Debug, Clone, Copy)]
struct Word<const LEN: usize, const P2: bool>(u32);
impl<const LEN: usize, const P2: bool> Word<LEN, P2> {
	fn new(n: u32) -> Word<LEN, P2> {
		let mut new = Word(n);
		if new.has_adjacent() && new.first_decreasing().is_none() {
			return new;
		}
		while let Some(w) = new.inc_checked() {
			new = w;
			if new.has_adjacent() {
				break;
			}
		}
		new
	}
	fn digit(&self, i: usize) -> u8 {
		(self.0 / 10u32.pow((WORD_LEN - 1 - i) as u32) % 10) as u8
	}
	fn digits(&self) -> [u8; WORD_LEN] {
		let mut dig = [0; WORD_LEN];

		for i in 0..WORD_LEN {
			dig[i] = self.digit(i);
		}

		dig
	}
	
	/// Increases the digit by one, ensuring that consecutive digits do not decrease.
	fn inc_checked(&self) -> Option<Word<LEN, P2>> {
		let mut new = Word(self.0);
		new.0 += 1;

		if let Some((i, v)) = new.first_decreasing() {
			let mag = 10u32.pow((WORD_LEN - i) as u32);
			
			// clear lower digits
			new.0 = new.0 / mag * mag;

			// println!("cleared {} digits: {} -> {} (setting to {})", WORD_LEN - i, self.0, new.0, v);

			// set lower digits
			for ii in 0..(WORD_LEN - i) {
				let val = v * 10u32.pow(ii as u32);
				// println!("\tinc by: {}", val);
				new.0 += val;
			}

		}
		// println!("\tdone: {}", new.0);
		((new.0 as f32).log10() < (WORD_LEN as f32)).then(|| new)
	}
	fn first_decreasing(&self) -> Option<(usize, u32)> {
		let digits = self.digits();
		for (i, windir) in digits.windows(2).enumerate() {
			if windir[0] > windir[1] {
				return Some((i+1, windir[0] as u32));
			}
		}
		None
	}
	fn has_adjacent(&self) -> bool {
		if ! P2 {
			for windir in self.digits().windows(2) {
				if windir[0] == windir[1] { return true; }
			}
			false
		} else {
			match self.digits() {
				[a,b,c,_,_,_] if a == b && b != c => true,
				[a,b,c,d,_,_] if a != b && b == c && c != d => true,
				[_,a,b,c,d,_] if a != b && b == c && c != d => true,
				[_,_,a,b,c,d] if a != b && b == c && c != d => true,
				[_,_,_,a,b,c] if a != b && b == c => true,
				_ => false,
			}
		}
	}
}

struct NumIter<const P2: bool>(Word<WORD_LEN, P2>);
impl<const P2: bool> Iterator for NumIter<P2> {
	type Item = Word<WORD_LEN, P2>;
	fn next(&mut self) -> Option<Self::Item> {
		// increment, yield if applicable
		while let Some(w) = self.0.inc_checked() {
			self.0 = w;
			if w.has_adjacent() {
				return Some(w);
			}
		}
		None
	}
}

impl AoCDay for Day04 {
	type Data<'i> = (u32, u32);
	type Answer = usize;
	fn day(&self) -> u8 { 04 }
	fn parse<'i>(&self, input: &'i str) -> Self::Data<'i> {
		let (bgn, end) = input.trim().split_once('-').unwrap();

		(bgn.parse().unwrap(), end.parse().unwrap())
	}
	fn part1(&self, _data: &mut Self::Data<'_>) -> Self::Answer {
		let mut ni = NumIter::<false>(Word::new(_data.0));
		if ni.0.0 > _data.1 { return 0; }
		// println!("{}", ni.0.0);
		let mut count = 1; // first number
		for n in &mut ni {
			if n.0 > _data.1 {
				break;
			}
			// println!("{}", n.0);
			count += 1;
		}
		count
	}
	fn part2(&self, _data: &mut Self::Data<'_>) -> Self::Answer {
		let mut ni = NumIter::<true>(Word::new(_data.0));
		if ni.0.0 > _data.1 { return 0; }
		// println!("{}", ni.0.0);
		let mut count = 1; // first number
		for n in &mut ni {
			if n.0 > _data.1 {
				break;
			}
			// println!("{}", n.0);
			count += 1;
		}
		count
	}
}


#[cfg(test)]
fn works(n: u32) -> bool {
	let w = Word::<WORD_LEN, false>(n);
	w.has_adjacent() && w.first_decreasing().is_none()
}

#[test]
fn digit_works() {
	assert!(works(111111));
	assert!(!works(223450));
	assert!(!works(123789));
	assert_eq!(Word::<WORD_LEN, false>::new(111111).0, 111111);
	assert_eq!(Word::<WORD_LEN, false>::new(223450).0, 223455);
	assert_eq!(Word::<WORD_LEN, false>::new(123789).0, 123799);
	assert_eq!(Word::<WORD_LEN, false>::new(111123).0, 111123);
	assert_eq!(Word::<WORD_LEN, false>::new(135679).0, 135688);
	assert_eq!(Word::<WORD_LEN, false>::new(122345).0, 122345);
	assert_eq!(Word::<WORD_LEN, false>::new(112111).0, 112222);
}

#[test]
fn part1() {
	let cases = [
		(daystr!("04"), 1246),
	];
	test_runner::<_, _>(Day04, DayPart::Part1, &cases);
}
#[test]
fn part2() {
	let cases = [
		(daystr!("04"), 814),
	];
	test_runner::<_, _>(Day04, DayPart::Part2, &cases);
}
