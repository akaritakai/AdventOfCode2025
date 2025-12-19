use crate::puzzle::Puzzle;
use std::collections::HashMap;

pub struct Day {
    points: Vec<Point>,
}

impl Puzzle for Day {
    /// Find the maximum inclusive-tile area of an axis-aligned rectangle whose
    /// two opposite corners are red tiles (points).
    ///
    /// This checks all pairs with light pruning after sorting by X.
    ///
    /// Time complexity: O(N^2)
    /// Auxiliary space complexity: O(N)
    fn solve_part_1(&self) -> String {
        let min_y = self.points.iter().map(|p| p.1).min().unwrap();
        let max_y = self.points.iter().map(|p| p.1).max().unwrap();
        let max_possible_height = (max_y - min_y).abs() as i128 + 1;
        let pts = self.sorted_points();
        let x_last = pts.last().unwrap().0;
        let mut best: i128 = 0;
        for (i, &(x1, y1)) in pts.iter().enumerate() {
            let max_possible_width = (x_last - x1).abs() as i128 + 1;
            if max_possible_width * max_possible_height <= best {
                continue;
            }
            for &(x2, y2) in pts.iter().skip(i + 1) {
                best = std::cmp::max(best, inclusive_area((x1, y1), (x2, y2)));
            }
        }
        best.to_string()
    }

    /// Interprets the input as a rectilinear polygonal loop (points in order),
    /// then finds the maximum inclusive-tile area axis-aligned rectangle whose
    /// opposite corners are vertices and whose interior lies completely inside
    /// the polygon.
    ///
    /// Uses coordinate compression and a scanline parity fill to build a grid of
    /// inside-cells, then a 2D prefix sum for O(1) area-inside queries.
    ///
    /// Time complexity: O(N^2)
    /// Auxiliary space complexity: O(N^2)
    fn solve_part_2(&self) -> String {
        let (xs, ys, x_index, y_index) = compress_axes(&self.points);
        let v_edges = build_vertical_edges(&self.points, &x_index);
        let pref = build_prefix_sums(&xs, &ys, &v_edges);
        let pts = self.sorted_points();
        let x_last = pts.last().unwrap().0;
        let max_possible_height = (ys.last().unwrap() - ys[0]).abs() as i128 + 1;
        let mut best: i128 = 0;
        for (i, &(x1, y1)) in pts.iter().enumerate() {
            let max_possible_width = (x_last - x1).abs() as i128 + 1;
            if max_possible_width * max_possible_height <= best {
                continue;
            }
            let xi1 = *x_index.get(&x1).unwrap();
            let yi1 = *y_index.get(&y1).unwrap();
            for &(x2, y2) in pts.iter().skip(i + 1) {
                let area = inclusive_area((x1, y1), (x2, y2));
                if area <= best {
                    continue;
                }
                let xi2 = *x_index.get(&x2).unwrap();
                let yi2 = *y_index.get(&y2).unwrap();
                let x_min = xi1.min(xi2);
                let x_max = xi1.max(xi2);
                let y_min = yi1.min(yi2);
                let y_max = yi1.max(yi2);
                let target_cells = ((x_max - x_min) * (y_max - y_min)) as i128;
                let actual_cells = rect_sum(&pref, x_min, x_max, y_min, y_max);
                if actual_cells == target_cells {
                    best = area;
                }
            }
        }
        best.to_string()
    }
}

type Point = (i64, i64);

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        let points: Vec<Point> = input
            .trim()
            .lines()
            .map(|line| {
                let mut it = line.trim().split(',');
                let x = it.next().unwrap().parse::<i64>().unwrap();
                let y = it.next().unwrap().parse::<i64>().unwrap();
                (x, y)
            })
            .collect();
        Box::new(Day { points })
    }

    /// Returns points sorted by x-coordinate.
    fn sorted_points(&self) -> Vec<Point> {
        let mut pts = self.points.clone();
        pts.sort_unstable_by_key(|p| p.0);
        pts
    }
}

/// Inclusive tile-area for two opposite corners.
fn inclusive_area(a: Point, b: Point) -> i128 {
    let dx = (a.0 - b.0).abs() as i128 + 1;
    let dy = (a.1 - b.1).abs() as i128 + 1;
    dx * dy
}
type CompressedAxes = (Vec<i64>, Vec<i64>, HashMap<i64, usize>, HashMap<i64, usize>);

/// Builds sorted unique coordinate axes and index maps.
fn compress_axes(points: &[Point]) -> CompressedAxes {
    let mut xs: Vec<i64> = points.iter().map(|p| p.0).collect();
    let mut ys: Vec<i64> = points.iter().map(|p| p.1).collect();
    xs.sort_unstable();
    xs.dedup();
    ys.sort_unstable();
    ys.dedup();
    let x_index = xs
        .iter()
        .copied()
        .enumerate()
        .map(|(i, x)| (x, i))
        .collect();
    let y_index = ys
        .iter()
        .copied()
        .enumerate()
        .map(|(i, y)| (y, i))
        .collect();
    (xs, ys, x_index, y_index)
}

/// Extract vertical edges from an ordered polygonal chain.
fn build_vertical_edges(points: &[Point], x_index: &HashMap<i64, usize>) -> Vec<Vec<(i64, i64)>> {
    let mut xs: Vec<i64> = x_index.keys().copied().collect();
    xs.sort_unstable();
    let mut v_edges: Vec<Vec<(i64, i64)>> = vec![vec![]; xs.len()];
    let n = points.len();
    for i in 0..n {
        let p1 = points[i];
        let p2 = points[(i + 1) % n];
        if p1.0 == p2.0
            && let Some(&xi) = x_index.get(&p1.0)
        {
            let y_min = p1.1.min(p2.1);
            let y_max = p1.1.max(p2.1);
            v_edges[xi].push((y_min, y_max));
        }
    }

    v_edges
}

/// Build a 2D prefix sum over compressed cells indicating interior of the polygon.
fn build_prefix_sums(xs: &[i64], ys: &[i64], v_edges: &[Vec<(i64, i64)>]) -> Vec<Vec<i128>> {
    let w = xs.len() - 1;
    let h = ys.len() - 1;
    let mut pref = vec![vec![0i128; w + 1]; h + 1];
    for r in 0..h {
        let y_start = ys[r];
        let y_end = ys[r + 1];
        let mut inside = false;
        let mut row_sum: i128 = 0;
        for c in 0..w {
            if let Some(edges) = v_edges.get(c) {
                for &(ey_min, ey_max) in edges {
                    if ey_min <= y_start && ey_max >= y_end {
                        inside = !inside;
                    }
                }
            }
            if inside {
                row_sum += 1;
            }
            pref[r + 1][c + 1] = pref[r][c + 1] + row_sum;
        }
    }
    pref
}

/// Query number of inside cells in rectangle of compressed cell indices.
fn rect_sum(pref: &[Vec<i128>], x_min: usize, x_max: usize, y_min: usize, y_max: usize) -> i128 {
    pref[y_max][x_max] - pref[y_min][x_max] - pref[y_max][x_min] + pref[y_min][x_min]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "\
            7,1\n\
            11,1\n\
            11,7\n\
            9,7\n\
            9,5\n\
            2,5\n\
            2,3\n\
            7,3";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_1(), "50");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/09")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "4786902990");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "\
            7,1\n\
            11,1\n\
            11,7\n\
            9,7\n\
            9,5\n\
            2,5\n\
            2,3\n\
            7,3";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "24");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/09")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "1571016172");
    }
}
