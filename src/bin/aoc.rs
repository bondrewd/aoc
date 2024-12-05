use aoc;
use clap::Parser;

/// A compilation of Advent of Code problems
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Advent of Code year (>=2015)
    #[arg(short, long)]
    year: i32,

    /// Problem number
    #[arg(short, long)]
    problem: u32,
}

fn main() {
    let args = Args::parse();
    let year = args.year;
    let problem = args.problem;

    match year {
        2015 => match args.problem {
            1 => aoc::y2015::ex01(),
            2 => aoc::y2015::ex02(),
            3 => aoc::y2015::ex03(),
            4 => aoc::y2015::ex04(),
            5 => aoc::y2015::ex05(),
            6 => aoc::y2015::ex06(),
            _ => println!("year {}, problem {} not yet implemented", year, problem),
        }
        2024 => match args.problem {
            1 => aoc::y2024::ex01(),
            _ => println!("year {}, problem {} not yet implemented", year, problem),
        }
        _ => println!("year {} not yet implemented", year),
    }
}
