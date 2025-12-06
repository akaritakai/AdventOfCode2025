use crate::puzzle::Puzzle;
use std::ops::RangeInclusive;
use rangemap::RangeInclusiveSet;

pub struct Day {
    fresh_id_ranges: Vec<RangeInclusive<u64>>,
    available_ids: Vec<u64>,
}

impl Puzzle for Day {
    /// TODO
    ///
    /// Time complexity: TODO
    /// Auxiliary space complexity: TODO
    fn solve_part_1(&self) -> String {
        let mut ranges = RangeInclusiveSet::new();
        for range in &self.fresh_id_ranges {
            ranges.insert(range.clone());
        }
        self.available_ids
            .iter()
            .filter(|id| ranges.contains(id))
            .count()
            .to_string()
    }

    /// TODO
    ///
    /// Time complexity: TODO
    /// Auxiliary space complexity: TODO
    fn solve_part_2(&self) -> String {
        let mut ranges = RangeInclusiveSet::new();
        for range in &self.fresh_id_ranges {
            ranges.insert(range.clone());
        }
        ranges.iter().map(|range| range.end() - range.start() + 1).sum::<u64>().to_string()
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        let mut parts = input.split("\n\n");
        let fresh_id_ranges = parts
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                let (start, end) = line.split_once('-').unwrap();
                start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap()
            })
            .collect::<Vec<_>>();
        let available_ids = parts
            .next()
            .unwrap()
            .lines()
            .map(|line| line.parse::<u64>().unwrap())
            .collect();
        Box::new(Day {
            fresh_id_ranges,
            available_ids,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "\
            3-5\n\
            10-14\n\
            16-20\n\
            12-18\n\
            \n\
            1\n\
            5\n\
            8\n\
            11\n\
            17\n\
            32";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "3");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/05")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "509");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "\
            3-5\n\
            10-14\n\
            16-20\n\
            12-18\n\
            \n\
            1\n\
            5\n\
            8\n\
            11\n\
            17\n\
            32";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "14");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/05")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "336790092076620");
    }
}
