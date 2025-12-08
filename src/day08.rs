use crate::puzzle::Puzzle;

pub struct Day {
    points: Vec<Point>,
}

impl Puzzle for Day {
    /// Connect the 1000 closest pairs, then multiply sizes of the 3 largest circuits.
    ///
    /// Time complexity: O(N^2)
    /// Auxiliary space complexity: O(N^2)
    fn solve_part_1(&self) -> String {
        short_connections_product(&self.points, 1000).to_string()
    }

    /// Keep connecting closest pairs until all junction boxes are in one circuit.
    /// Return product of X coordinates of the last edge that merges the final two components.
    ///
    /// Time complexity: O(N^2 log N)
    /// Auxiliary space complexity: O(N^2)
    fn solve_part_2(&self) -> String {
        let n = self.points.len();
        let mut edges = all_edges(&self.points);
        edges.sort_unstable_by_key(|e| e.dist2);
        let mut dsu = Dsu::new(n);
        let mut last_merged: Option<Edge> = None;
        for e in edges {
            if dsu.union(e.from, e.to) {
                last_merged = Some(e);
                if dsu.components == 1 {
                    break;
                }
            }
        }
        let e = last_merged.unwrap();
        let p1_x = self.points[e.from].x;
        let p2_x = self.points[e.to].x;
        (p1_x * p2_x).to_string()
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        let points = input
            .trim()
            .lines()
            .map(|line| {
                let mut it = line.trim().split(',');
                let x = it.next().unwrap().parse::<i64>().unwrap();
                let y = it.next().unwrap().parse::<i64>().unwrap();
                let z = it.next().unwrap().parse::<i64>().unwrap();
                Point { x, y, z }
            })
            .collect();
        Box::new(Day { points })
    }
}

struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    /// Squared Euclidean distance
    fn dist2(&self, other: &Point) -> u64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz) as u64
    }
}

struct Edge {
    from: usize,
    to: usize,
    dist2: u64,
}

fn all_edges(points: &[Point]) -> Vec<Edge> {
    let n = points.len();
    let mut edges = Vec::with_capacity(n * (n - 1) / 2);
    for i in 0..n {
        for j in (i + 1)..n {
            let dist2 = points[i].dist2(&points[j]);
            edges.push(Edge {
                from: i,
                to: j,
                dist2,
            });
        }
    }
    edges
}

struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
    components: usize,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            components: n,
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        let mut root = x;
        while self.parent[root] != root {
            root = self.parent[root];
        }
        while self.parent[x] != x {
            let p = self.parent[x];
            self.parent[x] = root;
            x = p;
        }
        root
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return false;
        }
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        self.components -= 1;
        true
    }

    fn component_sizes(&mut self) -> Vec<usize> {
        let n = self.parent.len();
        for i in 0..n {
            self.find(i);
        }
        let mut sizes = Vec::new();
        for i in 0..n {
            if self.parent[i] == i {
                sizes.push(self.size[i]);
            }
        }
        sizes
    }
}

fn short_connections_product(points: &[Point], count: usize) -> usize {
    let n = points.len();
    let mut edges = all_edges(points);
    let k = count.min(edges.len());
    edges.select_nth_unstable_by_key(k - 1, |e| e.dist2);
    edges.truncate(k);
    let mut dsu = Dsu::new(n);
    for e in edges {
        dsu.union(e.from, e.to);
    }
    let mut sizes = dsu.component_sizes();
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    sizes.into_iter().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let points = vec![
            Point {
                x: 162,
                y: 817,
                z: 812,
            },
            Point {
                x: 57,
                y: 618,
                z: 57,
            },
            Point {
                x: 906,
                y: 360,
                z: 560,
            },
            Point {
                x: 592,
                y: 479,
                z: 940,
            },
            Point {
                x: 352,
                y: 342,
                z: 300,
            },
            Point {
                x: 466,
                y: 668,
                z: 158,
            },
            Point {
                x: 542,
                y: 29,
                z: 236,
            },
            Point {
                x: 431,
                y: 825,
                z: 988,
            },
            Point {
                x: 739,
                y: 650,
                z: 466,
            },
            Point {
                x: 52,
                y: 470,
                z: 668,
            },
            Point {
                x: 216,
                y: 146,
                z: 977,
            },
            Point {
                x: 819,
                y: 987,
                z: 18,
            },
            Point {
                x: 117,
                y: 168,
                z: 530,
            },
            Point {
                x: 805,
                y: 96,
                z: 715,
            },
            Point {
                x: 346,
                y: 949,
                z: 466,
            },
            Point {
                x: 970,
                y: 615,
                z: 88,
            },
            Point {
                x: 941,
                y: 993,
                z: 340,
            },
            Point {
                x: 862,
                y: 61,
                z: 35,
            },
            Point {
                x: 984,
                y: 92,
                z: 344,
            },
            Point {
                x: 425,
                y: 690,
                z: 689,
            },
        ];
        assert_eq!(short_connections_product(&points, 10), 40);
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/08")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "26400");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "\
            162,817,812\n\
            57,618,57\n\
            906,360,560\n\
            592,479,940\n\
            352,342,300\n\
            466,668,158\n\
            542,29,236\n\
            431,825,988\n\
            739,650,466\n\
            52,470,668\n\
            216,146,977\n\
            819,987,18\n\
            117,168,530\n\
            805,96,715\n\
            346,949,466\n\
            970,615,88\n\
            941,993,340\n\
            862,61,35\n\
            984,92,344\n\
            425,690,689";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "25272");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/08")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "8199963486");
    }
}
