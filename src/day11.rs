use std::collections::{HashMap, HashSet};
use crate::puzzle::Puzzle;

pub struct Day {
    graph: Graph,
}

impl Puzzle for Day {
    /// Count the number of distinct directed paths from "you" to "out" in a DAG.
    ///
    /// Time complexity: O(V + E) where V is the number of devices and E is the number of
    /// connections.
    /// Auxiliary space complexity: O(V)
    fn solve_part_1(&self) -> String {
        self.count_paths("you", "out").to_string()
    }

    /// Count the number of paths from "svr" to "out" that pass through both "dac" and "fft" (in any
    /// order).
    ///
    /// Time complexity: O(V + E) where V is the number of devices and E is the number of
    /// connections.
    /// Auxiliary space complexity: O(V)
    fn solve_part_2(&self) -> String {
        let svr_to_dac = self.count_paths("svr", "dac");
        let dac_to_fft = self.count_paths("dac", "fft");
        let fft_to_out = self.count_paths("fft", "out");
        let dac_before_fft = svr_to_dac * dac_to_fft * fft_to_out;
        let svr_to_fft = self.count_paths("svr", "fft");
        let fft_to_dac = self.count_paths("fft", "dac");
        let dac_to_out = self.count_paths("dac", "out");
        let fft_before_dac = svr_to_fft * fft_to_dac * dac_to_out;
        (dac_before_fft + fft_before_dac).to_string()
    }
}

type Graph = HashMap<String, HashSet<String>>;

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        let graph: Graph = input.trim().lines()
            .map(|line| {
                let (from, to_part) = line.trim().split_once(": ").unwrap();
                let to = to_part.split_whitespace().map(|t| t.to_string()).collect();
                (from.to_string(), to)
            })
            .collect();
        Box::new(Day { graph })
    }

    fn count_paths(&self, start: &str, end: &str) -> usize {
        fn dfs<'a>(
            node: &'a str,
            end: &str,
            edges: &'a Graph,
            memo: &mut HashMap<&'a str, usize>,
        ) -> usize {
            if let Some(&cached) = memo.get(node) {
                return cached;
            }
            if node == end {
                memo.insert(node, 1);
                return 1;
            }
            let mut total = 0;
            if let Some(neighbors) = edges.get(node) {
                for neighbor in neighbors {
                    total += dfs(neighbor, end, edges, memo);
                }
            }
            memo.insert(node, total);
            total
        }
        let mut memo: HashMap<&str, usize> = HashMap::new();
        dfs(start, end, &self.graph, &mut memo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "\
            aaa: you hhh\n\
            you: bbb ccc\n\
            bbb: ddd eee\n\
            ccc: ddd eee fff\n\
            ddd: ggg\n\
            eee: out\n\
            fff: out\n\
            ggg: out\n\
            hhh: ccc fff iii\n\
            iii: out";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "5");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/11")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "470");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "\
            svr: aaa bbb\n\
            aaa: fft\n\
            fft: ccc\n\
            bbb: tty\n\
            tty: ccc\n\
            ccc: ddd eee\n\
            ddd: hub\n\
            hub: fff\n\
            eee: dac\n\
            dac: fff\n\
            fff: ggg hhh\n\
            ggg: out\n\
            hhh: out";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "2");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/11")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "384151614084875");
    }
}
