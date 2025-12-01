use crate::input_fetcher::InputFetcher;
use crate::puzzle::Puzzle;

mod input_fetcher;
mod puzzle;

fn main() {
    let fetcher = InputFetcher::create();
    let puzzles: Vec<Box<dyn Puzzle>> = vec![
    ];
    for (i, puzzle) in puzzles.iter().enumerate() {
        println!("Day {:02} Part 1: {}", i + 1, puzzle.solve_part_1());
        println!("Day {:02} Part 2: {}", i + 1, puzzle.solve_part_2());
    }
}
