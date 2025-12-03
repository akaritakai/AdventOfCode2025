use aoc2025::input_fetcher::InputFetcher;
use aoc2025::puzzle::Puzzle;
use aoc2025::{day01, day02, day03};

fn main() {
    let fetcher = InputFetcher::create();
    let puzzles: Vec<Box<dyn Puzzle>> = vec![
        day01::Day::create(fetcher.get_input(1).unwrap().as_str()),
        day02::Day::create(fetcher.get_input(2).unwrap().as_str()),
        day03::Day::create(fetcher.get_input(3).unwrap().as_str()),
    ];
    for (i, puzzle) in puzzles.iter().enumerate() {
        println!("Day {:02} Part 1: {}", i + 1, puzzle.solve_part_1());
        println!("Day {:02} Part 2: {}", i + 1, puzzle.solve_part_2());
    }
}
