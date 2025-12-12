use std::collections::HashSet;
use crate::puzzle::Puzzle;

pub struct Day {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}

impl Puzzle for Day {
    /// Count how many regions can fit all required presents (packing with rotations/flips).
    ///
    /// Time complexity: TODO
    /// Auxiliary space complexity: TODO
    fn solve_part_1(&self) -> String {
        self.regions.iter().filter(|r| region_can_fit(r, &self.shapes)).count().to_string()
    }

    fn solve_part_2(&self) -> String {
        "Day 12 has no part 2".to_string()
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        let (shapes_raw, regions) = parse_input(input);
        let mut shapes: Vec<Shape> = Vec::with_capacity(shapes_raw.len());
        for cells in shapes_raw {
            let variants = gen_variants(&cells);
            shapes.push(Shape {
                area: cells.len(),
                variants,
            });
        }
        Box::new(Day { shapes, regions })
    }
}


#[derive(Clone)]
struct Region {
    w: usize,
    h: usize,
    counts: Vec<u8>,
}

#[derive(Clone)]
struct Shape {
    area: usize,
    variants: Vec<Variant>,
}

#[derive(Clone)]
struct Variant {
    w: usize,
    h: usize,
    cells: Vec<(u8, u8)>,
}

#[derive(Clone)]
struct PlacementList {
    words: usize,
    masks: Vec<u64>,
}

impl PlacementList {
    fn num_masks(&self) -> usize {
        if self.words == 0 { 0 } else { self.masks.len() / self.words }
    }

    fn mask(&self, idx: usize) -> &[u64] {
        let start = idx * self.words;
        &self.masks[start..start + self.words]
    }

    fn iter_masks(&self) -> impl Iterator<Item = &[u64]> {
        (0..self.num_masks()).map(move |i| self.mask(i))
    }

    fn generate(region_w: usize, region_h: usize, shape: &Shape) -> Self {
        let cells = region_w * region_h;
        let words = (cells + 63) / 64;
        let mut masks: Vec<u64> = Vec::new();
        for v in &shape.variants {
            if v.w > region_w || v.h > region_h {
                continue;
            }
            let max_x0 = region_w - v.w;
            let max_y0 = region_h - v.h;

            for y0 in 0..=max_y0 {
                for x0 in 0..=max_x0 {
                    let mut mask = vec![0u64; words];
                    for &(dx, dy) in &v.cells {
                        let x = x0 + dx as usize;
                        let y = y0 + dy as usize;
                        let idx = y * region_w + x;
                        let w = idx / 64;
                        let b = idx % 64;
                        mask[w] |= 1u64 << b;
                    }
                    masks.extend_from_slice(&mask);
                }
            }
        }
        Self { words, masks }
    }
}

fn region_can_fit(region: &Region, shapes: &[Shape]) -> bool {
    if region.counts.len() != shapes.len() {
        return false;
    }
    let board_cells = region.w * region.h;
    let needed_cells: usize = region
        .counts
        .iter()
        .enumerate()
        .map(|(i, &c)| (c as usize) * shapes[i].area)
        .sum();
    if needed_cells > board_cells {
        return false;
    }
    let mut placements: Vec<Option<PlacementList>> = vec![None; shapes.len()];
    for (i, &c) in region.counts.iter().enumerate() {
        if c == 0 {
            continue;
        }
        let plist = PlacementList::generate(region.w, region.h, &shapes[i]);
        if plist.num_masks() == 0 {
            return false;
        }
        placements[i] = Some(plist);
    }
    let words = (board_cells + 63) / 64;
    let mut occ = vec![0u64; words];
    let mut remaining = region.counts.clone();
    dfs_pack(&mut occ, &mut remaining, &placements)
}

fn dfs_pack(occ: &mut [u64], remaining: &mut [u8], placements: &[Option<PlacementList>]) -> bool {
    if remaining.iter().all(|&c| c == 0) {
        return true;
    }
    let mut best_t: Option<usize> = None;
    let mut best_fit_count: usize = usize::MAX;
    for (t, &cnt) in remaining.iter().enumerate() {
        if cnt == 0 {
            continue;
        }
        let plist = placements[t].as_ref().unwrap();
        let mut fit = 0usize;
        for mask in plist.iter_masks() {
            if fits(occ, mask) {
                fit += 1;
                if fit >= best_fit_count {
                    break;
                }
            }
        }
        if fit == 0 {
            return false;
        }
        if fit < best_fit_count {
            best_fit_count = fit;
            best_t = Some(t);
            if best_fit_count == 1 {
                break;
            }
        }
    }
    let t = best_t.unwrap();
    let plist = placements[t].as_ref().unwrap();
    for mask in plist.iter_masks() {
        if !fits(occ, mask) {
            continue;
        }
        apply(occ, mask);
        remaining[t] -= 1;
        if dfs_pack(occ, remaining, placements) {
            return true;
        }
        remaining[t] += 1;
        unapply(occ, mask);
    }
    false
}

fn fits(occ: &[u64], mask: &[u64]) -> bool {
    for i in 0..occ.len() {
        if (occ[i] & mask[i]) != 0 {
            return false;
        }
    }
    true
}

fn apply(occ: &mut [u64], mask: &[u64]) {
    for i in 0..occ.len() {
        occ[i] |= mask[i];
    }
}

fn unapply(occ: &mut [u64], mask: &[u64]) {
    for i in 0..occ.len() {
        occ[i] ^= mask[i];
    }
}

fn parse_input(input: &str) -> (Vec<Vec<(i32, i32)>>, Vec<Region>) {
    let lines: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();
    let mut shapes_map: Vec<Option<Vec<String>>> = Vec::new();
    let mut i = 0usize;
    while i < lines.len() {
        let line = &lines[i];
        if line.is_empty() {
            i += 1;
            continue;
        }
        if is_region_line(line) {
            break;
        }
        if let Some(idx_str) = line.strip_suffix(':') {
            let idx: usize = idx_str.parse().unwrap();
            i += 1;
            let mut grid: Vec<String> = Vec::new();
            while i < lines.len() && !lines[i].is_empty() {
                grid.push(lines[i].clone());
                i += 1;
            }
            if shapes_map.len() <= idx {
                shapes_map.resize_with(idx + 1, || None);
            }
            shapes_map[idx] = Some(grid);
        } else {
            i += 1;
        }
    }
    let shape_grids: Vec<Vec<String>> = shapes_map
        .into_iter()
        .map(|opt| opt.expect("missing shape index"))
        .collect();
    let shapes_raw: Vec<Vec<(i32, i32)>> = shape_grids
        .into_iter()
        .map(|grid| {
            let mut cells = Vec::new();
            for (y, row) in grid.iter().enumerate() {
                for (x, ch) in row.chars().enumerate() {
                    if ch == '#' {
                        cells.push((x as i32, y as i32));
                    }
                }
            }
            cells
        })
        .collect();
    let n_shapes = shapes_raw.len();
    let mut regions: Vec<Region> = Vec::new();
    while i < lines.len() {
        let line = &lines[i];
        i += 1;
        if line.is_empty() {
            continue;
        }
        if !is_region_line(line) {
            continue;
        }
        let (wh, rest) = line.split_once(':').unwrap();
        let (w_str, h_str) = wh.split_once('x').unwrap();
        let w: usize = w_str.parse().unwrap();
        let h: usize = h_str.parse().unwrap();
        let mut counts: Vec<u8> = rest
            .split_whitespace()
            .map(|s| s.parse::<u8>().unwrap())
            .collect();
        if counts.len() < n_shapes {
            counts.resize(n_shapes, 0);
        }
        regions.push(Region { w, h, counts });
    }
    (shapes_raw, regions)
}

fn is_region_line(s: &str) -> bool {
    let Some((wh, _rest)) = s.split_once(':') else { return false; };
    let Some((w, h)) = wh.split_once('x') else { return false; };
    !w.is_empty()
        && !h.is_empty()
        && w.chars().all(|c| c.is_ascii_digit())
        && h.chars().all(|c| c.is_ascii_digit())
}

fn gen_variants(base: &[(i32, i32)]) -> Vec<Variant> {
    let mut seen: HashSet<Vec<(u8, u8)>> = HashSet::new();
    let mut out: Vec<Variant> = Vec::new();
    for flip in [false, true] {
        for rot in 0..4 {
            let mut coords: Vec<(i32, i32)> = Vec::with_capacity(base.len());
            for &(x0, y0) in base {
                let mut x = x0;
                let mut y = y0;
                if flip {
                    x = -x;
                }
                let (rx, ry) = match rot {
                    0 => (x, y),
                    1 => (-y, x),
                    2 => (-x, -y),
                    3 => (y, -x),
                    _ => unreachable!(),
                };
                coords.push((rx, ry));
            }
            let min_x = coords.iter().map(|(x, _)| *x).min().unwrap();
            let min_y = coords.iter().map(|(_, y)| *y).min().unwrap();
            for (x, y) in coords.iter_mut() {
                *x -= min_x;
                *y -= min_y;
            }
            let max_x = coords.iter().map(|(x, _)| *x).max().unwrap();
            let max_y = coords.iter().map(|(_, y)| *y).max().unwrap();
            let w = (max_x + 1) as usize;
            let h = (max_y + 1) as usize;
            let mut cells_u8: Vec<(u8, u8)> = coords
                .into_iter()
                .map(|(x, y)| (x as u8, y as u8))
                .collect();
            cells_u8.sort_unstable();
            if seen.insert(cells_u8.clone()) {
                out.push(Variant {
                    w,
                    h,
                    cells: cells_u8,
                });
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "\
            0:\n\
            ###\n\
            ##.\n\
            ##.\n\
            \n\
            1:\n\
            ###\n\
            ##.\n\
            .##\n\
            \n\
            2:\n\
            .##\n\
            ###\n\
            ##.\n\
            \n\
            3:\n\
            ##.\n\
            ###\n\
            ##.\n\
            \n\
            4:\n\
            ###\n\
            #..\n\
            ###\n\
            \n\
            5:\n\
            ###\n\
            .#.\n\
            ###\n\
            \n\
            4x4: 0 0 0 0 2 0\n\
            12x5: 1 0 1 0 2 2\n\
            12x5: 1 0 1 0 3 2";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "2");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/12")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "519");
    }
}
