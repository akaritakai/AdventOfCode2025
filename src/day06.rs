use crate::puzzle::Puzzle;
use std::ops::Range;

pub struct Day {
    grid: Vec<Vec<char>>,
    num_grid: Vec<Vec<u64>>,
    col_ranges: Vec<Range<usize>>,
    ops: Vec<Op>,
}

impl Puzzle for Day {
    /// Reduces each numeric column group independently using the operator specified in the footer
    /// row, then sums the results across groups.
    ///
    /// Time complexity: O(M * N), where M is the number of data rows and N is the number of
    /// operator column groups.
    /// Auxiliary space complexity: O(M * N) for the pre-parsed numeric grid.
    fn solve_part_1(&self) -> String {
        (0..self.ops.len())
            .map(|col| {
                let mut it = self.num_grid.iter().map(|row| row[col]);
                let first = it.next().unwrap();
                it.fold(first, |acc, n| self.ops[col].apply(acc, n))
            })
            .sum::<u64>()
            .to_string()
    }

    /// For each contiguous group of digit-bearing columns, read a number per column  by
    /// concatenating vertical digits (top-to-bottom). Process columns right-to-left within each
    /// group, combining with that group's operator, then sum the group results.
    ///
    /// Time complexity: O(M * N), where M is the number of data rows and N is the number of
    /// character columns (each column scan touches all rows).
    /// Auxiliary space complexity: O(1)
    fn solve_part_2(&self) -> String {
        let number_for_col = |col: usize, grid: &Vec<Vec<char>>| -> u64 {
            grid.iter()
                .map(|row| row[col])
                .filter_map(|c| c.to_digit(10).map(|d| d as u64))
                .fold(0u64, |n, d| n * 10 + d)
        };
        self.col_ranges
            .iter()
            .zip(self.ops.iter().copied())
            .map(|(range, op)| {
                range
                    .clone()
                    .map(|col| number_for_col(col, &self.grid))
                    .reduce(|a, b| op.apply(a, b))
                    .unwrap()
            })
            .sum::<u64>()
            .to_string()
    }
}

#[derive(Clone, Copy)]
enum Op {
    Add,
    Mul,
}

impl Op {
    fn from_char(c: char) -> Self {
        match c {
            '+' => Op::Add,
            '*' => Op::Mul,
            _ => unreachable!(),
        }
    }

    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Op::Add => a + b,
            Op::Mul => a * b,
        }
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        let mut lines: Vec<&str> = input.lines().collect();
        while matches!(lines.last(), Some(l) if l.is_empty()) {
            lines.pop();
        }
        let ops_line = lines.pop().unwrap();
        let data_lines = lines;
        let ops: Vec<Op> = ops_line
            .chars()
            .filter(|&c| c == '+' || c == '*')
            .map(Op::from_char)
            .collect();
        let grid: Vec<Vec<char>> = data_lines.iter().map(|l| l.chars().collect()).collect();
        let num_grid: Vec<Vec<u64>> = data_lines
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>()
            })
            .collect();
        let num_rows = grid.len();
        let mut col_ranges: Vec<Range<usize>> = Vec::new();
        let mut start: Option<usize> = None;
        for (col, _) in grid[0].iter().enumerate() {
            let has_digit = (0..num_rows).any(|row| grid[row][col].is_ascii_digit());
            match (start, has_digit) {
                (None, true) => start = Some(col),
                (Some(s), false) => {
                    col_ranges.push(s..col);
                    start = None;
                }
                _ => {}
            }
        }
        if let Some(s) = start {
            col_ranges.push(s..grid[0].len());
        }
        Box::new(Day {
            grid,
            num_grid,
            col_ranges,
            ops,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = [
            "123 328  51 64 ",
            " 45 64  387 23 ",
            "  6 98  215 314",
            "*   +   *   +  ",
        ]
        .join("\n");
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "4277556");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/06")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "5227286044585");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = [
            "123 328  51 64 ",
            " 45 64  387 23 ",
            "  6 98  215 314",
            "*   +   *   +  ",
        ]
        .join("\n");
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "3263827");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/06")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "10227753257799");
    }
}
