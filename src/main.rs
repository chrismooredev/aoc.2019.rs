use aoch::DayPart;
use clap::Parser;

aoch::load_days!();

#[derive(Parser, Debug)]
struct Args {
    /// Day to run. If not supplied, all are ran.
    #[clap(short, long, value_parser(1..=25))]
    day: Option<i64>,

    /// Part of the day to run. If not supplied, both are ran.
    #[clap(short, long, value_parser(1..=2))]
    part: Option<i64>,
}

fn main() {
    let args = Args::parse();

    let day = args.day.map(|n| n as usize);
    let part = args.part.map(|n| match n {
        1 => DayPart::Part1,
        2 => DayPart::Part2,
        _ => panic!("clap allowed bad part value: {}", n),
    });

    if let Some(day) = day {
        let (inp, fun) = RUNNERS[day-1];
        fun(part, inp);
    } else {
        for (inp, fun) in RUNNERS {
            fun(part, inp);
        }
    }
}
