use crate::puzzle::Puzzle;
use rayon::prelude::*;
use std::collections::VecDeque;

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

    /// For each machine, compute the minimum total number of button presses needed to satisfy the
    /// per-light joltage requirements, then sum these minima across all machines.
    ///
    /// We form this problem as a system of linear equations and solve for non-negative integer
    /// solutions that minimize the sum of variables by solving for the Reduced Row Echelon Form.
    ///
    /// Time complexity: Constraint construction is O(N * B * L^2) where N is the number of
    /// machines, B is the number of buttons per machine, and L is the number of lights per machine.
    /// Auxiliary space complexity: O(B * L)
    fn solve_part_2(&self) -> String {
        self.machines
            .par_iter()
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
        let num_vars = self.button_wires.len();
        let num_eqs = self.num_lights;
        let mut matrix = vec![vec![0.0; num_vars + 1]; num_eqs];
        for (btn_idx, wires) in self.button_wires.iter().enumerate() {
            for &light_idx in wires {
                matrix[light_idx][btn_idx] = 1.0;
            }
        }
        for (light_idx, &goal) in self.joltage_goal.iter().enumerate() {
            matrix[light_idx][num_vars] = goal as f64;
        }
        let mut pivot_row = 0;
        let mut pivot_cols = Vec::new();
        for col in 0..num_vars {
            if pivot_row >= num_eqs {
                break;
            }
            let mut selection = pivot_row;
            while selection < num_eqs && matrix[selection][col].abs() < 1e-9 {
                selection += 1;
            }
            if selection < num_eqs {
                matrix.swap(pivot_row, selection);
                let pivot_val = matrix[pivot_row][col];
                for j in col..=num_vars {
                    matrix[pivot_row][j] /= pivot_val;
                }
                for i in 0..num_eqs {
                    if i != pivot_row {
                        let factor = matrix[i][col];
                        if factor.abs() > 1e-9 {
                            for j in col..=num_vars {
                                matrix[i][j] -= factor * matrix[pivot_row][j];
                            }
                        }
                    }
                }
                pivot_cols.push(col);
                pivot_row += 1;
            }
        }
        for i in pivot_row..num_eqs {
            if matrix[i][num_vars].abs() > 1e-4 {
                return None;
            }
        }
        let mut free_vars = Vec::new();
        for col in 0..num_vars {
            if !pivot_cols.contains(&col) {
                free_vars.push(col);
            }
        }
        let mut best_total = None;
        let mut bounds = vec![u64::MAX; num_vars];
        for (btn_idx, wires) in self.button_wires.iter().enumerate() {
            for &light in wires {
                let limit = self.joltage_goal[light] as u64;
                if limit < bounds[btn_idx] {
                    bounds[btn_idx] = limit;
                }
            }
        }
        let free_var_bounds: Vec<u64> = free_vars.iter().map(|&idx| bounds[idx]).collect();
        self.recursive_search(
            0,
            &free_vars,
            &free_var_bounds,
            &mut vec![0; num_vars],
            &matrix,
            &pivot_cols,
            &mut best_total
        );
        best_total
    }

    fn recursive_search(
        &self,
        free_idx: usize,
        free_vars: &[usize],
        bounds: &[u64],
        current_sol: &mut Vec<u64>,
        matrix: &Vec<Vec<f64>>,
        pivot_cols: &[usize],
        best_total: &mut Option<u64>
    ) {
        let current_sum: u64 = current_sol.iter().sum();
        if let Some(best) = *best_total {
            if current_sum >= best {
                return;
            }
        }
        if free_idx == free_vars.len() {
            let num_vars = current_sol.len();
            let mut valid = true;
            let mut derived_sol = current_sol.clone();
            for (row_idx, &p_col) in pivot_cols.iter().enumerate() {
                let mut val = matrix[row_idx][num_vars];
                for &f_col in free_vars {
                    val -= matrix[row_idx][f_col] * (current_sol[f_col] as f64);
                }
                if val < -1e-4 {
                    valid = false;
                    break;
                }
                let rounded = val.round();
                if (val - rounded).abs() > 1e-4 {
                    valid = false;
                    break;
                }
                derived_sol[p_col] = rounded as u64;
            }
            if valid {
                let total: u64 = derived_sol.iter().sum();
                match best_total {
                    Some(b) => *b = (*b).min(total),
                    None => *best_total = Some(total),
                }
            }
            return;
        }
        let f_var_idx = free_vars[free_idx];
        let limit = bounds[free_idx];
        for val in 0..=limit {
            current_sol[f_var_idx] = val;
            self.recursive_search(
                free_idx + 1, 
                free_vars, 
                bounds, 
                current_sol, 
                matrix, 
                pivot_cols, 
                best_total
            );
            current_sol[f_var_idx] = 0;
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
