use crate::puzzle::Puzzle;
use std::collections::VecDeque;
use z3::{Optimize, SatResult, ast::Int};

pub struct Day {
    machines: Vec<Machine>,
}

impl Puzzle for Day {
    /// For each machine, compute the minimum number of button presses needed to reach the target
    /// lighting pattern (treating each button as a bitmask toggle), then sum these minima across
    /// all machines.
    ///
    /// This uses a BFS over the state space of light configurations.
    ///
    /// Time complexity: O(N * B * 2^L) where N is the number of machines, B is the number of
    /// buttons per machine, and L is the number of lights per machine.
    /// Auxiliary space complexity: O(2^L)
    fn solve_part_1(&self) -> String {
        self.machines
            .iter()
            .map(|m| m.min_lighting_presses().unwrap())
            .sum::<usize>()
            .to_string()
    }

    /// For each machine, compute the minimum total number of button presses
    /// needed to satisfy the per-light joltage requirements, then sum these
    /// minima across all machines.
    ///
    /// This formulates a nonnegative-integer optimization problem and solves
    /// it using Z3 Optimize.
    ///
    /// Time complexity: Constraint construction is O(N * B * L^2) where N is the number of
    /// machines, B is the number of buttons per machine, and L is the number of lights per machine.
    /// Auxiliary space complexity: O(B * L)
    fn solve_part_2(&self) -> String {
        self.machines
            .iter()
            .map(|m| m.min_joltage_presses().unwrap())
            .sum::<u64>()
            .to_string()
    }
}

struct Machine {
    num_lights: usize,
    lighting_goal: u16,
    button_masks: Vec<u16>,
    button_wires: Vec<Vec<usize>>,
    joltage_goal: Vec<usize>,
}

impl Machine {
    fn from_line(line: &str) -> Self {
        let (rest, joltage_part) = line.split_once('{').unwrap();
        let (lights_part, buttons_part) = rest.split_once(']').unwrap();
        let lights_str = lights_part.trim_start_matches('[');
        let num_lights = lights_str.len();
        let mut lighting_goal = 0;
        for (i, c) in lights_str.chars().enumerate() {
            if c == '#' {
                lighting_goal |= 1 << (num_lights - i - 1);
            }
        }
        let joltage_goal: Vec<usize> = joltage_part
            .trim_end_matches('}')
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect();
        let mut button_masks = Vec::new();
        let mut button_wires = Vec::new();
        for segment in buttons_part.split('(').skip(1) {
            let content = segment.split(')').next().unwrap();
            let wires: Vec<usize> = content
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();
            let mut mask = 0;
            for &wire in &wires {
                mask |= 1 << (num_lights - wire - 1);
            }
            button_wires.push(wires);
            button_masks.push(mask);
        }
        Machine {
            num_lights,
            lighting_goal,
            button_masks,
            button_wires,
            joltage_goal,
        }
    }

    fn min_lighting_presses(&self) -> Option<usize> {
        let target = self.lighting_goal;
        let limit = 1 << self.num_lights;
        let mut visited = vec![false; limit];
        let mut queue = VecDeque::new();
        queue.push_back((0, 0));
        visited[0] = true;
        while let Some((current, steps)) = queue.pop_front() {
            for &mask in &self.button_masks {
                let next_val = current ^ mask;
                if next_val == target {
                    return Some(steps + 1);
                }
                let next_idx = next_val as usize;
                if next_idx < limit && !visited[next_idx] {
                    visited[next_idx] = true;
                    queue.push_back((next_val, steps + 1));
                }
            }
        }
        None
    }

    fn min_joltage_presses(&self) -> Option<u64> {
        let opt = Optimize::new();
        let mut press_counts = Vec::new();
        let zero = Int::from_i64(0);
        let mut total_presses_expr = Int::from_i64(0);
        for i in 0..self.button_wires.len() {
            let x_i = Int::new_const(format!("x_{}", i));
            opt.assert(&x_i.ge(&zero));
            total_presses_expr = &total_presses_expr + &x_i;
            press_counts.push(x_i);
        }
        for light_idx in 0..self.num_lights {
            let mut light_sum = Int::from_i64(0);
            for (btn_idx, wires) in self.button_wires.iter().enumerate() {
                if wires.contains(&light_idx) {
                    light_sum = &light_sum + &press_counts[btn_idx];
                }
            }
            let goal_val = self.joltage_goal[light_idx] as i64;
            opt.assert(&light_sum.eq(Int::from_i64(goal_val)));
        }
        opt.minimize(&total_presses_expr);
        match opt.check(&[]) {
            SatResult::Sat => {
                let model = opt.get_model().unwrap();
                let result = model.eval(&total_presses_expr, true).unwrap();
                result.as_u64()
            }
            _ => None,
        }
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        let machines: Vec<Machine> = input.trim().lines().map(Machine::from_line).collect();
        Box::new(Day { machines })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "\
            [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n\
            [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n\
            [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "7");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/10")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "527");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "\
            [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n\
            [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n\
            [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "33");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/10")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "19810");
    }
}
