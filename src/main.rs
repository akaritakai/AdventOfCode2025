use aoc2025::input_fetcher::InputFetcher;
use aoc2025::puzzle::Puzzle;
use aoc2025::{day01, day02, day03, day04, day05, day06, day07, day08, day09};

fn main() {
    let fetcher = InputFetcher::create();
    let puzzles: Vec<Box<dyn Puzzle>> = vec![
        day01::Day::create(fetcher.get_input(1).unwrap().as_str()),
        day02::Day::create(fetcher.get_input(2).unwrap().as_str()),
        day03::Day::create(fetcher.get_input(3).unwrap().as_str()),
        day04::Day::create(fetcher.get_input(4).unwrap().as_str()),
        day05::Day::create(fetcher.get_input(5).unwrap().as_str()),
        day06::Day::create(fetcher.get_input(6).unwrap().as_str()),
        day07::Day::create(fetcher.get_input(7).unwrap().as_str()),
        day08::Day::create(fetcher.get_input(8).unwrap().as_str()),
        day09::Day::create(fetcher.get_input(9).unwrap().as_str()),
    ];
    for (i, puzzle) in puzzles.iter().enumerate() {
        println!("Day {:02} Part 1: {}", i + 1, puzzle.solve_part_1());
        println!("Day {:02} Part 2: {}", i + 1, puzzle.solve_part_2());
    }
}
