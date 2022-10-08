use std::collections::{HashMap, HashSet};
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};

// TODO: detect infinite loops?
// take hash of (self.pc, self.ram) after every write. if same, then loop has been detected.

pub type ICInt = i128;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParamMode {
    Position, // day02
    Immediate, // day05
    Relative, // day09
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Instruction(ICInt);
impl Instruction {
    fn instr(&self) -> u8 {
        (self.0 % 100) as u8
    }
    fn param(&self, i: usize) -> ParamMode {
        match self.0 / (10 as ICInt).pow(2+i as u32) % 10 {
            0 => ParamMode::Position,
            1 => ParamMode::Immediate,
            2 => ParamMode::Relative,
            u => {
                unreachable!("unknown parameter mode on instruction {} for parameter {}: {}", self.0, i, u);
            },
        }
    }
}
impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Instruction")
            .field("value", &self.0)
            .field("instr", &self.instr())
            .finish()
    }
}

#[test]
fn instr_decode() {
    let instr = Instruction(21002);
    assert_eq!(instr.instr(), 2);
    assert_eq!(instr.param(0), ParamMode::Position);
    assert_eq!(instr.param(1), ParamMode::Immediate);
    assert_eq!(instr.param(2), ParamMode::Relative);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use]
pub enum RunResult {
    Halted,
    Starved,
    InvalidInstruction(Instruction),
    RepeatedState,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Intcode {
	pc: usize,
    relative_base: ICInt,
	stepped: usize,
	pub ram: Vec<ICInt>,
	original: Vec<ICInt>,
    pub input: Vec<ICInt>,
    pub output: Vec<ICInt>,
    prev_states: HashSet<u64>,
}
impl Intcode {
	pub fn new(ram: Vec<ICInt>) -> Intcode {
		Intcode {
			original: ram.clone(),
			ram,
            ..Default::default()
		}
	}
    pub fn parse(s: &str) -> Intcode {
        let v: Vec<ICInt> = s.split(',')
			.filter_map(aoch::parsing::trimmed)
            .map(|n| n.parse::<_>().unwrap())
            .collect();
		Intcode::new(v)
    }
	pub fn reset(&mut self) {
		self.pc = 0;
		self.stepped = 0;
        self.ram.resize(self.original.len(), 0); // shrink if necessary
        self.ram.copy_from_slice(&self.original); // copy original back into it
        self.input = Vec::new();
        self.output = Vec::new();
	}
    pub fn is_halted(&self) -> bool {
        Instruction(self.ram[self.pc as usize]).instr() == 99
    }
    fn hash_value(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }

    #[cfg(test)]
    pub fn pc(&self) -> usize {
        self.pc
    }
    
    #[cfg(test)]
    pub fn stepped(&self) -> usize {
        self.stepped
    }

    fn resolve_param(&self, instr: Instruction, ind: usize) -> ICInt {
        //param.resolve(&self.ram, self.pc + offset, self.relative_base)
        let param = instr.param(ind);
        let arg_ptr = self.pc + ind + 1;
        let out_ptr = match param {
            ParamMode::Position => self.ram[arg_ptr] as usize,
            ParamMode::Immediate => arg_ptr,
            ParamMode::Relative if self.relative_base == 0 => self.ram[arg_ptr] as usize,
            ParamMode::Relative => (self.relative_base + self.ram[arg_ptr]) as usize,
        };
        if out_ptr >= self.ram.len() {
            return 0;
        }
        self.ram[out_ptr]
    }
    fn resolve_param_mut(&mut self, instr: Instruction, ind: usize) -> &mut ICInt {
        let param = instr.param(ind);
        let arg_ptr = self.pc + ind + 1;
        let arg_val = self.ram[arg_ptr];
        let out_ptr = match param {
            ParamMode::Position => arg_val as usize,
            ParamMode::Immediate => panic!("attempt to use immediate value as output parameter"),
            // ParamMode::Relative if self.relative_base == 0 => arg_val as usize,
            ParamMode::Relative => (self.relative_base + arg_val) as usize,
        };
        if out_ptr >= self.ram.len() {
            self.ram.resize(out_ptr+1, 0);
        }
        &mut self.ram[out_ptr]
    }

    /// Steps the CPU once. Returns Some(_) if the computer needed to stop.
    /// Returns None if another instruction can be executed.
	pub fn step<const DEBUG: bool>(&mut self) -> Option<RunResult> {
		let pc = self.pc as usize;
        let instr = Instruction(self.ram[pc]);

        if DEBUG {
            eprintln!("PC:{:>2}    {:?}", self.pc, instr);
        }

		match instr.instr() {
			99 => { // d2: hlt
				return Some(RunResult::Halted);
			},
			1 => { // d2: add [a] [b] [out]
                let in_a = self.resolve_param(instr, 0);
                let in_b = self.resolve_param(instr, 1);
                *self.resolve_param_mut(instr, 2) = in_a + in_b;
				self.pc += 4;
			},
			2 => { // d2: mul [a] [b] [out]
                let in_a = self.resolve_param(instr, 0);
                let in_b = self.resolve_param(instr, 1);
                *self.resolve_param_mut(instr, 2) = in_a * in_b;
				self.pc += 4;
			},
            3 => { // d5: inp [out]
                if self.input.len() == 0 {
                    return Some(RunResult::Starved);
                }
                *self.resolve_param_mut(instr, 0) = self.input.remove(0);
                self.pc += 2;
            },
            4 => { // d5: out [a]
                let in_a = self.resolve_param(instr, 0);
                self.output.push(in_a);
                self.pc += 2;
            },
            5 => { // d5: jnz [a] [tgt]
                let in_a = self.resolve_param(instr, 0);
                if in_a != 0 {
                    let in_b = self.resolve_param(instr, 1);
                    if in_b < 0 {
                        panic!("attempt to set program counter to negative value ({})", in_b);
                    }
                    self.pc = in_b as usize;
                } else {
                    self.pc += 3;
                }
            },
            6 => { // d5: jez [a] [tgt]
                let in_a = self.resolve_param(instr, 0);
                if in_a == 0 {
                    let in_b = self.resolve_param(instr, 1);
                    if in_b < 0 {
                        panic!("attempt to set program counter to negative value ({})", in_b);
                    }
                    self.pc = in_b as usize;
                } else {
                    self.pc += 3;
                }
            },
            7 => { // d5: lt [a] [b] [out]
                let in_a = self.resolve_param(instr, 0);
                let in_b = self.resolve_param(instr, 1);
                let out_ptr = self.resolve_param_mut(instr, 2);
                *out_ptr = (in_a < in_b) as ICInt;
                self.pc += 4;
            },
            8 => { // d5: eq [a] [b] [out]
                let in_a = self.resolve_param(instr, 0);
                let in_b = self.resolve_param(instr, 1);
                let out_ptr = self.resolve_param_mut(instr, 2);
                *out_ptr = (in_a == in_b) as ICInt;
                self.pc += 4;
            },
            9 => { // d9: arb [a]
                let in_a = self.resolve_param(instr, 0);
                self.relative_base += in_a;
                self.pc += 2;
            },
            _ => {
                eprintln!("unexpected opcode enountered @ PC={} after {} steps: {:?}", self.pc, self.stepped, instr);
                return Some(RunResult::InvalidInstruction(instr));
            },
		}
		self.stepped += 1;

        // only check state on backwards jumps - going backwards is an opportunity for a loop
        // match instr.instr() {
        //     5 | 6 => {
            if DEBUG {
                let hash = self.hash_value();
                // println!("hash: {}", hash);
                if ! self.prev_states.insert(hash) {
                    return Some(RunResult::RepeatedState);
               }
            }
        //     },
        //     _ => {},
        // }
        None
	}

    /// Runs until the computer needs to stop due to halting, needing input, or erroring.
	fn run_inner<const DEBUG: bool>(&mut self) -> RunResult {
        let mut heatmap: HashMap<usize, usize> = HashMap::new();
        loop {
            let ent = heatmap.entry(self.pc).or_default();
            if *ent > 10_000_000 {
                panic!("instruction at {} exceeded 10M calls!", self.pc);
            }
            *ent += 1;
			if let Some(rr) = self.step::<DEBUG>() {
                return rr;
            }
		}
	}
    pub fn run(&mut self) -> RunResult {
        self.run_inner::<false>()
    }
    #[cfg(test)]
    pub fn run_dbg(&mut self) -> RunResult {
        self.run_inner::<true>()
    }
}
impl Hash for Intcode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pc.hash(state);
        self.relative_base.hash(state);
        self.input.hash(state);
        self.ram.hash(state);
    }
}

#[cfg(test)]
fn test_for_input(prog: &[ICInt], inp: &[ICInt], exp: &[ICInt]) {
    let mut ic = Intcode::new(prog.to_vec());
    // ic.input.push(inp);
    ic.input = inp.to_vec();
    assert_eq!(ic.run(), RunResult::Halted);
    // assert_eq!(ic.output.pop(), Some(exp), "computer did not produce expected output in test");
    assert_eq!(&ic.output, exp, "computer did not produce expected output in test");
}
#[cfg(test)]
fn test_for_input_dbg(prog: &[ICInt], inp: &[ICInt], exp: &[ICInt]) {
    let mut ic = Intcode::new(prog.to_vec());
    // ic.input.push(inp);
    ic.input = inp.to_vec();
    assert_eq!(ic.run_dbg(), RunResult::Halted);
    // assert_eq!(ic.output.pop(), Some(exp), "computer did not produce expected output in test");
    assert_eq!(&ic.output, exp, "computer did not produce expected output in test");
}

#[cfg(test)]
mod day05 {
    use super::*;

    #[test]
    fn pos_teq() {
        const PROGRAM: &[ICInt] = &[3,9,8,9,10,9,4,9,99,-1,8];
        
        test_for_input(PROGRAM, &[-8], &[0]);
        test_for_input(PROGRAM, &[0], &[0]);
        test_for_input(PROGRAM, &[7], &[0]);
        test_for_input(PROGRAM, &[8], &[1]);
        test_for_input(PROGRAM, &[9], &[0]);
    }

    #[test]
    fn pos_lt() {
        const PROGRAM: &[ICInt] = &[3,9,7,9,10,9,4,9,99,-1,8];
        
        test_for_input(PROGRAM, &[-8], &[1]);
        test_for_input(PROGRAM, &[0], &[1]);
        test_for_input(PROGRAM, &[7], &[1]);
        test_for_input(PROGRAM, &[8], &[0]);
        test_for_input(PROGRAM, &[9], &[0]);
    }

    #[test]
    fn imm_eq() {
        const PROGRAM: &[ICInt] = &[3,3,1108,-1,8,3,4,3,99];
        
        test_for_input(PROGRAM, &[-8], &[0]);
        test_for_input(PROGRAM, &[0], &[0]);
        test_for_input(PROGRAM, &[7], &[0]);
        test_for_input(PROGRAM, &[8], &[1]);
        test_for_input(PROGRAM, &[9], &[0]);
    }

    #[test]
    fn imm_lt() {
        const PROGRAM: &[ICInt] = &[3,3,1107,-1,8,3,4,3,99];
        
        test_for_input(PROGRAM, &[-8], &[1]);
        test_for_input(PROGRAM, &[0], &[1]);
        test_for_input(PROGRAM, &[7], &[1]);
        test_for_input(PROGRAM, &[8], &[0]);
        test_for_input(PROGRAM, &[9], &[0]);
    }

    #[test]
    fn pos_jmp() {
        const PROGRAM: &[ICInt] = &[3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
        
        test_for_input(PROGRAM, &[-8], &[1]);
        test_for_input(PROGRAM, &[0], &[0]);
        test_for_input(PROGRAM, &[7], &[1]);
        test_for_input(PROGRAM, &[8], &[1]);
        test_for_input(PROGRAM, &[9], &[1]);
    }

    #[test]
    fn imm_jmp() {
        const PROGRAM: &[ICInt] = &[3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
        
        test_for_input(PROGRAM, &[-8], &[1]);
        test_for_input(PROGRAM, &[0], &[0]);
        test_for_input(PROGRAM, &[7], &[1]);
        test_for_input(PROGRAM, &[8], &[1]);
        test_for_input(PROGRAM, &[9], &[1]);
    }

    #[test]
    fn cmp() {
        const PROGRAM: &[ICInt] = &[
            3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99,
        ];
        
        test_for_input(PROGRAM, &[-8], &[999]);
        test_for_input(PROGRAM, &[0], &[999]);
        test_for_input(PROGRAM, &[7], &[999]);
        test_for_input(PROGRAM, &[8], &[1000]);
        test_for_input(PROGRAM, &[9], &[1001]);
    }
}

#[cfg(test)]
mod day09 {
    use super::*;

    #[test]
    fn quine() {
        const PROGRAM: &[ICInt] = &[
            109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99
        ];

        test_for_input(PROGRAM, &[], PROGRAM);
    }

    #[test]
    fn output_16_digit() {
        const PROGRAM: &[ICInt] = &[1102,34915192,34915192,7,4,7,99,0];

        let mut ic = Intcode::new(PROGRAM.to_vec());
        assert_eq!(ic.run(), RunResult::Halted);
        let out = ic.output.remove(0);
        assert_eq!((out as f32).log10().round(), 15.0, "expected 16-digit number, got {:?}", {
            ic.output.insert(0, out);
            ic.output
        });
    }

    #[test]
    fn output_large() {
        const PROGRAM: &[ICInt] = &[104,1125899906842624,99];

        test_for_input(PROGRAM, &[], &[PROGRAM[1]]);
    }
}
