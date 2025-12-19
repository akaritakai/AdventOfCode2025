use crate::puzzle::Puzzle;
use std::collections::{HashMap, HashSet};

pub struct Day {
    start: Pos,
    splitters: HashSet<Pos>,
}

impl Puzzle for Day {
    /// Simulates a set of unique beam positions falling one row at a time.
    ///
    /// A splitter (`^`) causes a beam to branch to down-left and down-right  when the splitter is
    /// directly below the beam's current position.
    ///
    /// This returns the number of *unique* split events encountered by the deduplicated beam
    /// frontier.
    ///
    /// Time complexity: O(N^2) where N is the larger of vertical/horizontal distance covered.
    /// Auxiliary space complexity: O(N)
    fn solve_part_1(&self) -> String {
        let mut num_splits: u64 = 0;
        let mut beams: Vec<Pos> = Vec::new();
        let mut next: Vec<Pos> = Vec::new();
        beams.push(self.start);
        for _ in self.start.0..self.last_splitter_row() {
            next.clear();
            for &(r, c) in &beams {
                let nr = r + 1;
                if self.splitters.contains(&(nr, c)) {
                    num_splits += 1;
                    unique_push(&mut next, (nr, c - 1));
                    unique_push(&mut next, (nr, c + 1));
                } else {
                    unique_push(&mut next, (nr, c));
                }
            }
            std::mem::swap(&mut beams, &mut next);
        }
        num_splits.to_string()
    }

    /// Simulates falling particles, but tracks multiplicity of timelines.
    ///
    /// Each time a timeline hits a splitter, it branches to left/right with the full count of
    /// timelines arriving at that position.
    ///
    /// The answer is the total number of timelines after the last splitter row.
    ///
    /// Time complexity: O(N^2) where N is the larger of vertical/horizontal distance covered.
    /// Auxiliary space complexity: O(N)
    fn solve_part_2(&self) -> String {
        let mut beams: HashMap<Pos, u128> = HashMap::new();
        let mut next: HashMap<Pos, u128> = HashMap::new();
        beams.insert(self.start, 1);
        for _ in self.start.0..self.last_splitter_row() {
            next.clear();
            for (&(r, c), &count) in &beams {
                let nr = r + 1;
                if self.splitters.contains(&(nr, c)) {
                    *next.entry((nr, c - 1)).or_insert(0) += count;
                    *next.entry((nr, c + 1)).or_insert(0) += count;
                } else {
                    *next.entry((nr, c)).or_insert(0) += count;
                }
            }
            std::mem::swap(&mut beams, &mut next);
        }
        beams.values().sum::<u128>().to_string()
    }
}

type Pos = (usize, usize);

fn unique_push(vec: &mut Vec<Pos>, pos: Pos) {
    if vec.last() != Some(&pos) {
        vec.push(pos);
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        let mut start: Option<Pos> = None;
        let mut splitters: HashSet<Pos> = HashSet::new();
        for (row, line) in input.trim().lines().enumerate() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let chars: Vec<char> = line.chars().collect();
            for (col, c) in chars.into_iter().enumerate() {
                match c {
                    'S' => start = Some((row, col)),
                    '^' => {
                        splitters.insert((row, col));
                    }
                    _ => {}
                }
            }
        }
        let start = start.unwrap();
        Box::new(Day { start, splitters })
    }

    fn last_splitter_row(&self) -> usize {
        self.splitters.iter().map(|&(row, _)| row).max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "\
            .......S.......\n\
            ...............\n\
            .......^.......\n\
            ...............\n\
            ......^.^......\n\
            ...............\n\
            .....^.^.^.....\n\
            ...............\n\
            ....^.^...^....\n\
            ...............\n\
            ...^.^...^.^...\n\
            ...............\n\
            ..^...^.....^..\n\
            ...............\n\
            .^.^.^.^.^...^.\n\
            ...............";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_1(), "21");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/07")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "1711");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "\
            .......S.......\n\
            ...............\n\
            .......^.......\n\
            ...............\n\
            ......^.^......\n\
            ...............\n\
            .....^.^.^.....\n\
            ...............\n\
            ....^.^...^....\n\
            ...............\n\
            ...^.^...^.^...\n\
            ...............\n\
            ..^...^.....^..\n\
            ...............\n\
            .^.^.^.^.^...^.\n\
            ...............";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "40");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/07")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "36706966158365");
    }
}
