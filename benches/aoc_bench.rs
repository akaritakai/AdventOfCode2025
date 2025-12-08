use aoc2025::{day01, day02, day03, day04, day05, day06, day07, day08};
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::time::Duration;

macro_rules! make_day_bench {
    ($fn_name:ident, $mod:ident, $day:literal) => {
        fn $fn_name(c: &mut Criterion) {
            let input = std::fs::read_to_string(concat!("resources/tests/", $day)).unwrap();
            let puzzle = $mod::Day::create(&input);

            c.bench_function(concat!("Day ", $day, " Part 1"), |b| {
                b.iter(|| black_box(puzzle.solve_part_1()))
            });
            c.bench_function(concat!("Day ", $day, " Part 2"), |b| {
                b.iter(|| black_box(puzzle.solve_part_2()))
            });
        }
    };
}

make_day_bench!(day01_bench, day01, "01");
make_day_bench!(day02_bench, day02, "02");
make_day_bench!(day03_bench, day03, "03");
make_day_bench!(day04_bench, day04, "04");
make_day_bench!(day05_bench, day05, "05");
make_day_bench!(day06_bench, day06, "06");
make_day_bench!(day07_bench, day07, "07");
make_day_bench!(day08_bench, day08, "08");

criterion_group! {
    name = benches;
    config = Criterion::default()
                 .sample_size(300)
                 .measurement_time(Duration::from_secs(30))
                 .nresamples(200_000);
    targets = day01_bench, day02_bench, day03_bench, day04_bench, day05_bench, day06_bench,
              day07_bench, day08_bench
}
criterion_main!(benches);
