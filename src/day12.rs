use crate::puzzle::Puzzle;

use ahash::{AHashMap, AHashSet};
use rayon::prelude::*;
use smallvec::SmallVec;
use std::sync::Arc;

pub struct Day {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}

impl Puzzle for Day {
    /// Count how many regions can fit all required presents (packing with rotations/flips).
    ///
    /// Time complexity: O(N * e^M) where N is the number of regions and M is the number of distinct
    /// shapes.
    /// Auxiliary space complexity: O(2^B) where B is the area of the region.
    fn solve_part_1(&self) -> String {
        let mut used_shape = vec![false; self.shapes.len()];
        let mut sizes: AHashSet<(usize, usize)> = AHashSet::new();
        for r in &self.regions {
            sizes.insert((r.w, r.h));
            for (i, &c) in r.counts.iter().enumerate() {
                if c != 0 {
                    used_shape[i] = true;
                }
            }
        }
        type PLKey = (usize, usize, usize);
        let mut placement_map: AHashMap<PLKey, Arc<PlacementList>> = AHashMap::new();
        for (w, h) in sizes.into_iter() {
            for (i, shape) in self.shapes.iter().enumerate() {
                if !used_shape[i] {
                    continue;
                }
                placement_map.insert((w, h, i), Arc::new(PlacementList::generate(w, h, shape)));
            }
        }
        let shapes = &self.shapes;
        let pm = &placement_map;
        self.regions
            .par_iter()
            .filter(|r| region_can_fit(r, shapes, pm))
            .count()
            .to_string()
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
struct Placement {
    chunks: SmallVec<[(u16, u64); 4]>,
}

#[derive(Clone)]
struct PlacementList {
    placements: Vec<Placement>,
}

impl PlacementList {
    fn generate(region_w: usize, region_h: usize, shape: &Shape) -> Self {
        let mut placements = Vec::new();
        for v in &shape.variants {
            if v.w > region_w || v.h > region_h {
                continue;
            }
            let max_x0 = region_w - v.w;
            let max_y0 = region_h - v.h;
            for y0 in 0..=max_y0 {
                for x0 in 0..=max_x0 {
                    let mut chunks: SmallVec<[(u16, u64); 4]> = SmallVec::new();
                    for &(dx, dy) in &v.cells {
                        let x = x0 + dx as usize;
                        let y = y0 + dy as usize;
                        let idx = y * region_w + x;
                        let wi = (idx >> 6) as u16;
                        let bit = 1u64 << (idx & 63);
                        if let Some((_, m)) = chunks.iter_mut().find(|(w, _)| *w == wi) {
                            *m |= bit;
                        } else {
                            chunks.push((wi, bit));
                        }
                    }
                    placements.push(Placement { chunks });
                }
            }
        }
        Self { placements }
    }

    #[inline]
    fn iter(&self) -> impl Iterator<Item = &Placement> {
        self.placements.iter()
    }
}

fn region_can_fit(
    region: &Region,
    shapes: &[Shape],
    placement_map: &AHashMap<(usize, usize, usize), Arc<PlacementList>>,
) -> bool {
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
    let mut placements: Vec<Option<Arc<PlacementList>>> = vec![None; shapes.len()];
    let mut types: Vec<usize> = Vec::new();
    for (i, &c) in region.counts.iter().enumerate() {
        if c == 0 {
            continue;
        }
        let plist = placement_map.get(&(region.w, region.h, i)).unwrap();
        if plist.placements.is_empty() {
            return false;
        }
        placements[i] = Some(plist.clone());
        types.push(i);
    }
    let words = board_cells.div_ceil(64);
    let mut occ = vec![0u64; words];
    let mut remaining = region.counts.clone();
    let pieces_left: usize = remaining.iter().map(|&c| c as usize).sum();
    let mut memo: AHashSet<StateKey> = AHashSet::new();
    dfs_pack(
        &mut occ,
        &mut remaining,
        &placements,
        &types,
        pieces_left,
        &mut memo,
    )
}

#[derive(Hash, Eq, PartialEq)]
struct StateKey {
    occ: SmallVec<[u64; 16]>,
    remaining: SmallVec<[u8; 32]>,
}

fn dfs_pack(
    occ: &mut [u64],
    remaining: &mut [u8],
    placements: &[Option<Arc<PlacementList>>],
    types: &[usize],
    pieces_left: usize,
    memo: &mut AHashSet<StateKey>,
) -> bool {
    if pieces_left == 0 {
        return true;
    }
    let key = StateKey {
        occ: SmallVec::from_slice(occ),
        remaining: SmallVec::from_slice(remaining),
    };
    if memo.contains(&key) {
        return false;
    }
    let mut best_t: Option<usize> = None;
    let mut best_fit_count: usize = usize::MAX;
    for &t in types {
        let cnt = remaining[t];
        if cnt == 0 {
            continue;
        }
        let plist = placements[t].as_ref().unwrap();
        let mut fit = 0usize;
        for p in plist.iter() {
            if fits(occ, p) {
                fit += 1;
                if fit >= best_fit_count {
                    break;
                }
            }
        }
        if fit == 0 {
            memo.insert(key);
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
    for p in plist.iter() {
        if !fits(occ, p) {
            continue;
        }
        apply(occ, p);
        remaining[t] -= 1;
        if dfs_pack(occ, remaining, placements, types, pieces_left - 1, memo) {
            return true;
        }
        remaining[t] += 1;
        unapply(occ, p);
    }
    memo.insert(key);
    false
}

#[inline(always)]
fn fits(occ: &[u64], p: &Placement) -> bool {
    for &(wi, m) in p.chunks.iter() {
        if (occ[wi as usize] & m) != 0 {
            return false;
        }
    }
    true
}

#[inline(always)]
fn apply(occ: &mut [u64], p: &Placement) {
    for &(wi, m) in p.chunks.iter() {
        occ[wi as usize] |= m;
    }
}

#[inline(always)]
fn unapply(occ: &mut [u64], p: &Placement) {
    for &(wi, m) in p.chunks.iter() {
        occ[wi as usize] ^= m;
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
    let shape_grids: Vec<Vec<String>> = shapes_map.into_iter().map(|opt| opt.unwrap()).collect();
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
    let Some((wh, _rest)) = s.split_once(':') else {
        return false;
    };
    let Some((w, h)) = wh.split_once('x') else {
        return false;
    };
    !w.is_empty()
        && !h.is_empty()
        && w.chars().all(|c| c.is_ascii_digit())
        && h.chars().all(|c| c.is_ascii_digit())
}

fn gen_variants(base: &[(i32, i32)]) -> Vec<Variant> {
    let mut seen: AHashSet<Vec<(u8, u8)>> = AHashSet::new();
    let mut out: Vec<Variant> = Vec::new();
    for flip in [false, true] {
        for rot in 0..4 {
            let mut coords: Vec<(i32, i32)> = Vec::with_capacity(base.len());
            for &(x0, y0) in base {
                let mut x = x0;
                let y = y0;
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
