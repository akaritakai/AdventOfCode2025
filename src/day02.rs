use crate::puzzle::Puzzle;
use std::ops::RangeInclusive;

pub struct Day {
    ranges: Vec<RangeInclusive<u64>>,
}

impl Puzzle for Day {
    /// Finds the sum of all numbers within the given ranges that are formed by repeating a seed
    /// strictly twice.
    ///
    /// Time complexity: O(n * m) where n is the number of ranges and m is the size of the range
    /// Auxiliary space complexity: O(1)
    fn solve_part_1(&self) -> String {
        self.ranges
            .iter()
            .map(|range| solve_range(*range.start(), *range.end(), true))
            .sum::<u64>()
            .to_string()
    }

    /// Finds the sum of all numbers within the given ranges that are formed by repeating a seed
    /// more than once.
    ///
    /// Time complexity: O(n * m) where n is the number of ranges and m is the size of the range
    /// Auxiliary space complexity: O(1)
    fn solve_part_2(&self) -> String {
        self.ranges
            .iter()
            .map(|range| solve_range(*range.start(), *range.end(), false))
            .sum::<u64>()
            .to_string()
    }
}

// Pre-computed powers of 10 for O(1) access.
const POW10: [u64; 20] = [
    1,
    10,
    100,
    1_000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    100_000_000,
    1_000_000_000,
    10_000_000_000,
    100_000_000_000,
    1_000_000_000_000,
    10_000_000_000_000,
    100_000_000_000_000,
    1_000_000_000_000_000,
    10_000_000_000_000_000,
    100_000_000_000_000_000,
    1_000_000_000_000_000_000,
    10_000_000_000_000_000_000,
];

/// Unified solver for both parts.
fn solve_range(start: u64, end: u64, strictly_two_repeats: bool) -> u64 {
    let mut sum: u64 = 0;
    let start_len = start.ilog10() + 1;
    let end_len = end.ilog10() + 1;
    for total_len in start_len..=end_len {
        let max_seed_len = total_len / 2;
        for seed_len in 1..=max_seed_len {
            if total_len % seed_len != 0 {
                continue;
            }
            let repeats = total_len / seed_len;
            if strictly_two_repeats && repeats != 2 {
                continue;
            }
            let multiplier = calculate_multiplier(seed_len, repeats);
            let start_seed = start
                .div_ceil(multiplier)
                .max(POW10[(seed_len - 1) as usize]);
            let end_seed = (end / multiplier).min(POW10[seed_len as usize] - 1);
            for seed in start_seed..=end_seed {
                if strictly_two_repeats || !is_periodic(seed) {
                    sum += seed * multiplier;
                }
            }
        }
    }
    sum
}

/// Helper to calculate the multiplier to turn a seed into a repeated number.
fn calculate_multiplier(seed_len: u32, repeats: u32) -> u64 {
    (0..repeats).fold(0, |acc, i| acc + POW10[(i * seed_len) as usize])
}

/// Returns true if `n` is formed by repeating a shorter substring.
fn is_periodic(n: u64) -> bool {
    if n < 10 {
        return false;
    }
    let digits = n.ilog10() + 1;
    for sub_len in 1..=(digits / 2) {
        if digits.is_multiple_of(sub_len) {
            let shift = digits - sub_len;
            let seed = n / POW10[shift as usize];
            let repeats = digits / sub_len;
            let multiplier = calculate_multiplier(sub_len, repeats);
            if seed * multiplier == n {
                return true;
            }
        }
    }
    false
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        let ranges = input
            .trim()
            .split(',')
            .map(|range| {
                let mut parts = range.trim().split('-');
                let start = parts.next().unwrap().parse::<u64>().unwrap();
                let end = parts.next().unwrap().parse::<u64>().unwrap();
                start..=end
            })
            .collect::<Vec<RangeInclusive<u64>>>();
        Box::new(Day { ranges })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "\
            11-22,\
            95-115,\
            998-1012,\
            1188511880-1188511890,\
            222220-222224,\
            1698522-1698528,\
            446443-446449,\
            38593856-38593862,\
            565653-565659,\
            824824821-824824827,\
            2121212118-2121212124";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "1227775554");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/02")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "28146997880");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "\
            11-22,\
            95-115,\
            998-1012,\
            1188511880-1188511890,\
            222220-222224,\
            1698522-1698528,\
            446443-446449,\
            38593856-38593862,\
            565653-565659,\
            824824821-824824827,\
            2121212118-2121212124";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "4174379265");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/02")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "40028128307");
    }
}
