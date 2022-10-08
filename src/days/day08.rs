
use std::io::Write;

#[allow(unused_imports)]
use aoch::{AoCDay, DayPart, daystr, run_test, test_runner};

use crate::rendering::*;

#[derive(Debug, Clone, Copy)]
pub struct Day08;


pub struct SIF<'s, const H: usize, const W: usize> where [(); H*W]: {
	raw: &'s [u8],
	layers: Vec<&'s [u8]>,
	flattened: Vec<u8>,
}
impl<'s, const H: usize, const W: usize> SIF<'s, H, W> where [(); H*W]: {
	pub fn new(raw: &'s str) -> SIF<'s, H, W> {
		assert_eq!(raw.len() % (H*W), 0, "data is not a multiple length of the dimensions");
		let bytes = raw.as_bytes();
		let layer_count = raw.len() / (H*W);
		let mut layers = Vec::with_capacity(layer_count);
		for i in 0..layer_count {
			layers.push(&bytes[i*H*W..(i+1)*H*W]);
		}

		SIF {
			raw: bytes,
			layers,
			flattened: Vec::new(),
		}
	}
	pub fn checksum(&self) -> usize {
		// layer with fewest '0' elements
		fn count_bytes(bs: &[u8], t: u8) -> usize {
			let mut total = 0;
			for b in bs {
				if *b == t { total += 1; }
			}
			total
		}
		
		let layer = self.layers.iter()
			.min_by_key(|l| count_bytes(l, b'0'))
			.unwrap();
		
		let ones = count_bytes(layer, b'1');
		let twos = count_bytes(layer, b'2');
		ones * twos
	}

	pub fn flatten(&mut self) -> &[u8] {
		if self.flattened.len() > 0 {
			return &self.flattened;
		}

		let mut output = self.layers.last().unwrap().to_vec();
		for l in (0..self.layers.len()-1).rev() {
			for i in 0..H*W {
				let p = self.layers[l][i];
				if p != b'2' {
					output[i] = p;
				}
			}
		}
		self.flattened = output;
		&self.flattened
	}

	pub fn render<Wr: Write>(&mut self, mut w: Wr) -> std::io::Result<()> {
		let flattened = self.flatten();
		for y in 0..H {
			let line = &flattened[y*W..(y+1)*W];
			for b in line {
				write!(w, "{}", match *b {
					b'0' => ' ',
					b'1' => '#',
					_ => panic!("bad character: {}", b),
				})?;
			}
			writeln!(w)?;
		}
		Ok(())
	}

	pub fn parse(&mut self) -> String {
		parse(self.flatten())
	}

	pub fn parse_(&mut self) -> String {
		assert_eq!(H, CHAR_HEIGHT);
		assert_eq!(W % CHAR_WIDTH, 0);
		let letters = W / CHAR_WIDTH;
		let flattened = self.flatten();
		let mut result = String::with_capacity(letters);
		for li in 0..letters {
			'letters: for gi in 0..RENDERED_CHARS.len() {
				let alpha = std::char::from_u32(b'A' as u32 + gi as u32).unwrap();
				let char = RENDERED_CHARS[gi];
				for row in 0..CHAR_HEIGHT {
					let row_start = row*W + li*CHAR_WIDTH;
					let rendered = &flattened[row_start..row_start+CHAR_WIDTH];
					let reference = &char[row*CHAR_WIDTH..(row+1)*CHAR_WIDTH];

					if DEBUG_OUTPUT {
						eprintln!("[LI={}][LETTER={},{}][ROW={}]", li, gi, alpha, row);
						eprintln!("\t[RENDERED] = {:?}", rendered);
						eprintln!("\t[REFERENC] = {:?}", reference);
					}
					if rendered != reference {
						continue 'letters;
					}
				}
				
				// found good letter
				result.push(alpha);
				break;
			}
			if DEBUG_OUTPUT { eprintln!("after li={}, result={:?}", li, result); }

			if result.len() != li+1 {
				panic!("unable to determine letter {} from flattened input render (not yet recognized?)", li);
			}
		}

		result
	}
}

impl AoCDay for Day08 {
	type Data<'i> = &'i str;
	type Answer = String;
	fn day(&self) -> u8 { 08 }
	fn parse<'i>(&self, input: &'i str) -> Self::Data<'i> {
		input.trim()
	}
	fn part1(&self, _data: &mut Self::Data<'_>) -> Self::Answer {
		let sif = SIF::<'_, 6, 25>::new(_data);
		sif.checksum().to_string()
	}
	fn part2(&self, _data: &mut Self::Data<'_>) -> Self::Answer {
		let mut sif = SIF::<'_, 6, 25>::new(_data);
		sif.render(std::io::stdout()).unwrap();
		sif.parse()
	}
}

#[test]
fn smol() {
	const IMAGE_DATA: &str = "123456789012";
	let sif = SIF::<'_, 2, 3>::new(IMAGE_DATA);
	assert_eq!(sif.checksum(), 1);
}

#[test]
fn part1() {
	let cases = [
		// (TEST_INPUT, 0),
		(daystr!("08"), "1848"),
	];
	test_runner::<_, _>(Day08, DayPart::Part1, &cases);
}
#[test]
fn part2() {
	let cases = [
		// (TEST_INPUT, 0),
		(daystr!("08"), "FGJUZ"),
	];
	test_runner::<_, _>(Day08, DayPart::Part2, &cases);
}
