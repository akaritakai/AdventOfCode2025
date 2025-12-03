use crate::puzzle::Puzzle;

pub struct Day {
    banks: Vec<Vec<u8>>,
}

impl Puzzle for Day {
    /// For each bank, finds the largest two-digit subsequence and then returns the sum across all
    /// banks.
    ///
    /// Time complexity: O(n * m) where n is the number of banks, and m is the number of digits in
    /// each bank.
    /// Auxiliary space complexity: O(m)
    fn solve_part_1(&self) -> String {
        self.banks
            .iter()
            .map(|bank| max_subsequence(bank, 2))
            .sum::<u64>()
            .to_string()
    }

    /// For each bank, finds the largest 12-digit subsequence and then returns the sum across all
    /// banks.
    ///
    /// Time complexity: O(n * m) where n is the number of banks, and m is the number of digits in
    /// each bank.
    /// Auxiliary space complexity: O(m)
    fn solve_part_2(&self) -> String {
        self.banks
            .iter()
            .map(|bank| max_subsequence(bank, 12))
            .sum::<u64>()
            .to_string()
    }
}

fn max_subsequence(digits: &[u8], length: usize) -> u64 {
    let mut deletions = digits.len() - length;
    let mut stack: Vec<u8> = Vec::with_capacity(digits.len());
    for &digit in digits {
        while deletions > 0 && matches!(stack.last(), Some(&last) if last < digit) {
            stack.pop();
            deletions -= 1;
        }
        stack.push(digit);
    }
    stack.truncate(length);
    stack.into_iter().fold(0u64, |acc, d| acc * 10 + d as u64)
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        let banks: Vec<Vec<u8>> = input
            .lines()
            .map(|line| line.trim().bytes().map(|b| b - b'0').collect())
            .collect();
        Box::new(Day { banks })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "\
            987654321111111\n\
            811111111111119\n\
            234234234234278\n\
            818181911112111";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "357");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/03")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "17034");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "\
            987654321111111\n\
            811111111111119\n\
            234234234234278\n\
            818181911112111";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "3121910778619");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/03")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "168798209663590");
    }
}
