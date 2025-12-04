use aoc2025::{day01, day02, day03, day04};
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::path::PathBuf;
use std::time::Duration;

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

fn day04_bench(c: &mut Criterion) {
    let input = std::fs::read_to_string(PathBuf::from("resources/tests/04")).unwrap();
    let puzzle = day04::Day::create(&input);

    c.bench_function("Day 04 Part 1", |b| {
        b.iter(|| black_box(puzzle.solve_part_1()))
    });
    c.bench_function("Day 04 Part 2", |b| {
        b.iter(|| black_box(puzzle.solve_part_2()))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(300).measurement_time(Duration::from_secs(30)).nresamples(200_000);
    targets = day01_bench, day02_bench, day03_bench, day04_bench
}
criterion_main!(benches);
