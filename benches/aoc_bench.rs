use aoc2025::{day01, day02, day03};
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use std::path::PathBuf;

fn day01_bench(c: &mut Criterion) {
    let input = std::fs::read_to_string(PathBuf::from("resources/tests/01")).unwrap();
    let puzzle = day01::Day::create(&input);

    c.bench_function("Day 01 Part 1", |b| {
        b.iter(|| black_box(puzzle.solve_part_1()))
    });
    c.bench_function("Day 01 Part 2", |b| {
        b.iter(|| black_box(puzzle.solve_part_2()))
    });
}

fn day02_bench(c: &mut Criterion) {
    let input = std::fs::read_to_string(PathBuf::from("resources/tests/02")).unwrap();
    let puzzle = day02::Day::create(&input);

    c.bench_function("Day 02 Part 1", |b| {
        b.iter(|| black_box(puzzle.solve_part_1()))
    });
    c.bench_function("Day 02 Part 2", |b| {
        b.iter(|| black_box(puzzle.solve_part_2()))
    });
}

fn day03_bench(c: &mut Criterion) {
    let input = std::fs::read_to_string(PathBuf::from("resources/tests/03")).unwrap();
    let puzzle = day03::Day::create(&input);

    c.bench_function("Day 03 Part 1", |b| {
        b.iter(|| black_box(puzzle.solve_part_1()))
    });
    c.bench_function("Day 03 Part 2", |b| {
        b.iter(|| black_box(puzzle.solve_part_2()))
    });
}

criterion_group!(benches, day01_bench, day02_bench, day03_bench);
criterion_main!(benches);
