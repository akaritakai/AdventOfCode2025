# Advent of Code 2025 Solutions

[![Build Status](https://github.com/akaritakai/AdventOfCode2025/actions/workflows/main.yml/badge.svg)](https://github.com/akaritakai/AdventOfCode2025/actions)
[![Code Coverage](https://img.shields.io/codecov/c/github/akaritakai/AdventOfCode2025.svg)](https://codecov.io/gh/akaritakai/AdventOfCode2025)
![Stars](https://img.shields.io/badge/gold%20stars%20⭐-14-yellow)
![Days Completed](https://img.shields.io/badge/days%20completed-7-green)

This repo contains my Advent of Code 2025 solutions in Rust. After providing it with your puzzle inputs (or your
session token), running the program will print out the answers to all currently solved days of the puzzle.
A Docker image is provided to ensure compatibility with machines that do not want to install dependencies.

The goal of this repo is to provide fast, highly tested, and easy-to-use solutions.

This repo may see changes in the future to improve runtime. If you have any suggestions, issues running the code, or
find a correctness error: please open an issue or pull request.

### Example output:
```
Day 01 Part 1: 1118
Day 01 Part 2: 6289
Day 02 Part 1: 28146997880
Day 02 Part 2: 40028128307
Day 03 Part 1: 17034
Day 03 Part 2: 168798209663590
Day 04 Part 1: 1424
Day 04 Part 2: 8727
Day 05 Part 1: 509
Day 05 Part 2: 336790092076620
Day 06 Part 1: 5227286044585
Day 06 Part 2: 10227753257799
Day 07 Part 1: 1711
Day 07 Part 2: 36706966158365
```

### Performance

| Solution       | Execution Time |
|----------------|----------------|
| Day 01 Part 01 | 11.597 µs      |
| Day 01 Part 02 | 8.8473 µs      |
| Day 02 Part 01 | 667.49 ns      |
| Day 02 Part 02 | 5.0826 µs      |
| Day 03 Part 01 | 23.290 µs      |
| Day 03 Part 02 | 23.779 µs      |
| Day 04 Part 01 | 51.885 µs      |
| Day 04 Part 02 | 430.92 µs      |
| Day 05 Part 01 | 17.620 µs      |
| Day 05 Part 02 | 7.6199 µs      |
| Day 06 Part 01 | 1.4939 µs      |
| Day 06 Part 02 | 5.6753 µs      |
| Day 07 Part 01 | 4.6171 ms      |
| Day 07 Part 02 | 189.55 µs      |
| **Total**      | 5.3951 ms      |

Benchmarks were measured using `cargo bench` on an [AMD Ryzen 9 7950X processor](https://www.cpubenchmark.net/cpu.php?id=5031).

## Docker Instructions

1. Follow the instructions below for providing your puzzle input.
2. Run `docker build -t aoc2025 .`
3. Run `docker run --rm --name aoc2025-run aoc2025`

## Providing Your Puzzle Input

There are two supported methods for inputting your puzzle data into this application.

### Automatic Puzzle Fetcher (via Session Cookie)

First, get your cookie session data.

You will need to log into the Advent of Code website and then inspect your cookies.
If you are using Chrome, you can follow the directions [here](https://developers.google.com/web/tools/chrome-devtools/storage/cookies).

You will be looking for a cookie called `session`. It will contain a long sequence of hexadecimal digits.

Place that data into a file called `cookie.txt` in the project directory.

The application will use that data to automatically fetch your puzzle input for each day.

### Manual Input

This code will also look in a particular location on your local machine for puzzle input.

In the project directory, it will check a directory called `puzzle`.
Within that directory it will expect Day 1's input to be in a file called `01`, Day 2's input to be in a file called `02`, etc.

You can find your puzzle input for a given day by logging into the Advent of Code website and then navigating to the URL
for that puzzle's input.

The URL for your puzzle input will be at:
```
https://adventofcode.com/2025/day/${DAY}/input
```
where `${DAY}` is the day number of the puzzle.

As an example, Day 1's input is at https://adventofcode.com/2025/day/1/input,
Day 2's input is at https://adventofcode.com/2025/day/2/input, etc.
