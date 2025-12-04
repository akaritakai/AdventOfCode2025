use crate::puzzle::Puzzle;
use itertools::iproduct;
use std::collections::VecDeque;

pub struct Day {
    grid: Vec<Vec<bool>>,
    num_rows: usize,
    num_cols: usize,
}

impl Puzzle for Day {
    /// Counts how many occupied cells have fewer than four occupied neighbors.
    ///
    /// Time complexity: O(M * N) where M is the number of rows and N is the number of columns
    /// Auxiliary space complexity: O(1)
    fn solve_part_1(&self) -> String {
        iproduct!(0..self.num_rows, 0..self.num_cols)
            .filter(|&(r, c)| self.grid[r][c] && self.count_neighbors(r, c) < MIN_NEIGHBORS)
            .count()
            .to_string()
    }

    /// Counts how many occupied cells can be removed in total if occupied cells with fewer than
    /// four occupied neighbors are removed iteratively.
    ///
    /// This is equivalent to peeling a grid graph down to its 4-core and counting all removed
    /// vertices.
    ///
    /// Time complexity:  O(M * N) where M is the number of rows and N is the number of columns.
    /// Auxiliary space complexity: O(M * N)
    fn solve_part_2(&self) -> String {
        let mut neighbor_counts = self.build_neighbor_counts();
        let mut grid = self.grid.clone();
        let mut in_queue = vec![vec![false; self.num_cols]; self.num_rows];
        let mut queue = VecDeque::<(usize, usize)>::new();
        for (r, c) in iproduct!(0..self.num_rows, 0..self.num_cols) {
            if grid[r][c] && neighbor_counts[r][c] < MIN_NEIGHBORS {
                in_queue[r][c] = true;
                queue.push_back((r, c));
            }
        }
        let mut removed = 0;
        while let Some((row, col)) = queue.pop_front() {
            if !grid[row][col] {
                continue;
            }
            grid[row][col] = false;
            removed += 1;
            for (dr, dc) in NEIGHBOR_DIRS {
                let nr = row as isize + dr;
                let nc = col as isize + dc;
                if !self.in_bounds(nr, nc) {
                    continue;
                }
                let ur = nr as usize;
                let uc = nc as usize;
                if !grid[ur][uc] {
                    continue;
                }
                let count = &mut neighbor_counts[ur][uc];
                if *count > 0 {
                    *count -= 1;
                }
                if *count < MIN_NEIGHBORS && !in_queue[ur][uc] {
                    in_queue[ur][uc] = true;
                    queue.push_back((ur, uc));
                }
            }
        }
        removed.to_string()
    }
}

const MIN_NEIGHBORS: u8 = 4;

const NEIGHBOR_DIRS: &[(isize, isize); 8] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        let grid: Vec<Vec<bool>> = input
            .trim()
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|ch| match ch {
                        '.' => false,
                        '@' => true,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();
        let num_rows = grid.len();
        let num_cols = grid[0].len();
        Box::new(Day {
            grid,
            num_rows,
            num_cols,
        })
    }

    fn in_bounds(&self, row: isize, col: isize) -> bool {
        row >= 0 && row < self.num_rows as isize && col >= 0 && col < self.num_cols as isize
    }

    fn count_neighbors(&self, row: usize, col: usize) -> u8 {
        NEIGHBOR_DIRS
            .iter()
            .filter(|(dr, dc)| {
                let nr = row as isize + dr;
                let nc = col as isize + dc;
                self.in_bounds(nr, nc) && self.grid[nr as usize][nc as usize]
            })
            .count() as u8
    }

    fn build_neighbor_counts(&self) -> Vec<Vec<u8>> {
        let mut counts = vec![vec![0u8; self.num_cols]; self.num_rows];
        for (r, c) in iproduct!(0..self.num_rows, 0..self.num_cols) {
            if self.grid[r][c] {
                counts[r][c] = self.count_neighbors(r, c);
            }
        }
        counts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "\
            ..@@.@@@@.\n\
            @@@.@.@.@@\n\
            @@@@@.@.@@\n\
            @.@@@@..@.\n\
            @@.@@@@.@@\n\
            .@@@@@@@.@\n\
            .@.@.@.@@@\n\
            @.@@@.@@@@\n\
            .@@@@@@@@.\n\
            @.@.@@@.@.";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "13");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/04")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "1424");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "\
            ..@@.@@@@.\n\
            @@@.@.@.@@\n\
            @@@@@.@.@@\n\
            @.@@@@..@.\n\
            @@.@@@@.@@\n\
            .@@@@@@@.@\n\
            .@.@.@.@@@\n\
            @.@@@.@@@@\n\
            .@@@@@@@@.\n\
            @.@.@@@.@.";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "43");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/04")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "8727");
    }
}
