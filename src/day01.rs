use crate::puzzle::Puzzle;
use num::Integer;

pub struct Day {
    moves: Vec<i32>,
}

impl Puzzle for Day {
    /// Simulates the movement of a dial being rotated using modulo arithmetic.
    ///
    /// Time complexity: O(n)
    /// Auxiliary space complexity: O(1)
    fn solve_part_1(&self) -> String {
        let mut dial = 50;
        let mut count = 0;
        for &mov in &self.moves {
            dial = (dial + mov) % 100;
            if dial == 0 {
                count += 1;
            }
        }
        count.to_string()
    }

    /// Calculates the number of times a dial being rotated crosses a specific point (0).
    ///
    /// Time complexity: O(n)
    /// Auxiliary space complexity: O(1)
    fn solve_part_2(&self) -> String {
        let mut dial: i32 = 50;
        let mut count = 0;
        for &mov in &self.moves {
            let prev = dial;
            dial += mov;
            if mov > 0 {
                count += Integer::div_floor(&dial, &100) - Integer::div_floor(&prev, &100);
            } else {
                count += Integer::div_ceil(&prev, &100) - Integer::div_ceil(&dial, &100);
            }
        }
        count.to_string()
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        let moves: Vec<i32> = input
            .lines()
            .map(|line| {
                let (dir, dist) = line.split_at(1);
                let dist: i32 = dist.parse().unwrap();
                match dir {
                    "L" => -dist,
                    "R" => dist,
                    _ => unreachable!(),
                }
            })
            .collect();
        Box::new(Day { moves })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "\
            L68\n\
            L30\n\
            R48\n\
            L5\n\
            R60\n\
            L55\n\
            L1\n\
            L99\n\
            R14\n\
            L82";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "3");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/01")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "1118");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "\
            L68\n\
            L30\n\
            R48\n\
            L5\n\
            R60\n\
            L55\n\
            L1\n\
            L99\n\
            R14\n\
            L82";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "6");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/01")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "6289");
    }
}
