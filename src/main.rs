#![feature(generic_associated_types)]
#![feature(generic_const_exprs)]
#![feature(concat_bytes)]
#![feature(array_windows)]

use aoch::DayPart;
use clap::Parser;

mod intcode;
mod rendering;

aoch::load_days!("../input");

#[derive(Parser, Debug)]
struct Args {
    /// Day to run. If not supplied, all are ran.
    #[clap(short, long, value_parser(1..=25))]
    day: Option<i64>,

    /// Part of the day to run. If not supplied, both are ran.
    #[clap(short, long, value_parser(1..=2))]
    part: Option<i64>,

    /// Repeats each test N times. This includes parsing input fresh for each test.
    #[clap(short, long, value_parser(0..))]
    repeat: Option<i64>,

    /// Disable computed output. Can be used to more accurately measure runtime performance.
    #[clap(short, long, action)]
    quiet: bool,
}

fn main() {
    let args = Args::parse();

    let day = args.day.map(|n| n as usize);
    let part = args.part.map(|n| match n {
        1 => DayPart::Part1,
        2 => DayPart::Part2,
        _ => panic!("clap allowed bad part value: {}", n),
    });

    let repeat = args.repeat.unwrap_or(1);

    if let Some(day) = day {
        let (inp, fun) = RUNNERS[day-1];
        for _ in 0..repeat {
            fun(part, args.quiet, inp);
        }
    } else {
        for (inp, fun) in RUNNERS {
            for _ in 0..repeat {
                fun(part, args.quiet, inp);
            }
        }
    }
}
