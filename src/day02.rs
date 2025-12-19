use crate::puzzle::Puzzle;
use cached::proc_macro::cached;
use divisors_fixed::Divisors;
use num::Integer;
use std::cmp::{max, min};
use std::ops::RangeInclusive;

pub struct Day {
    ranges: Vec<RangeInclusive<u64>>,
}

impl Puzzle for Day {
    /// Finds the sum of all doublets (numbers that are the concatenation of two identical strings)
    /// within the given ranges.
    ///
    /// Time complexity: O(n * log(m)) where n is the number of ranges, and m is the largest number
    /// in the range.
    /// Auxiliary space complexity: O(1)
    fn solve_part_1(&self) -> String {
        self.ranges
            .iter()
            .map(|range| sum_doublets_in_range(*range.start(), *range.end()))
            .sum::<u128>()
            .to_string()
    }

    /// Finds the sum of all non-primitive numbers (i.e., numbers that are the concatenation of the
    /// same string multiple times) within the given ranges.
    ///
    /// Time complexity: O(n * log^3(m)) where n is the number of ranges, and m is the largest
    /// number in the range.
    /// Auxiliary space complexity: O(1)
    fn solve_part_2(&self) -> String {
        self.ranges
            .iter()
            .map(|range| sum_nonprimitives_in_range(*range.start(), *range.end()))
            .sum::<u128>()
            .to_string()
    }
}

fn num_digits(n: u64) -> u32 {
    if n == 0 { 1 } else { n.ilog10() + 1 }
}

#[cached]
fn pow10(exp: u32) -> u128 {
    10u128.pow(exp)
}

fn ceil_div<T: Integer>(a: T, b: T) -> T {
    Integer::div_ceil(&a, &b)
}

fn floor_div<T: Integer>(a: T, b: T) -> T {
    Integer::div_floor(&a, &b)
}

#[cached]
fn divisors(n: u32) -> Vec<u32> {
    n.divisors()
}

/// Möbius function μ(n) for n ≥ 0.
///
/// μ(0) = 0 (by convention here)
/// μ(1) = 1
/// μ(n) = 0 if n has a squared prime factor
/// μ(n) = (-1)^k if n is a product of k distinct primes
#[cached]
fn mobius(mut n: u32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let mut mu: i32 = 1;
    let mut p: u32 = 2;
    // Trial division up to sqrt(n)
    while p * p <= n {
        if n.is_multiple_of(p) {
            let mut count = 0;
            while n.is_multiple_of(p) {
                n /= p;
                count += 1;
                if count > 1 {
                    // Squared prime factor ⇒ μ(n) = 0
                    return 0;
                }
            }
            // Flip sign for each distinct prime factor.
            mu = -mu;
        }
        p += if p == 2 { 1 } else { 2 }; // Check 2, then odd numbers only.
    }
    // If there is a prime factor > sqrt(original n), flip sign once more.
    if n > 1 { -mu } else { mu }
}

fn calculate_multiplier(seed_len: u32, num_repeats: u32) -> u128 {
    (0..num_repeats).fold(0u128, |acc, i| acc + pow10(i * seed_len))
}

fn sum_doublets_in_range(start: u64, end: u64) -> u128 {
    if end < 11 {
        return 0;
    }
    let start = if start < 11 { 11 } else { start };
    let min_len = ceil_div(num_digits(start), 2);
    let max_len = floor_div(num_digits(end), 2);
    let mut sum: u128 = 0;
    for len in min_len..=max_len {
        let multiplier = calculate_multiplier(len, 2);
        let low = max(pow10(len - 1), ceil_div(start as u128, multiplier));
        let high = min(pow10(len) - 1, floor_div(end as u128, multiplier));
        if low > high {
            continue;
        }
        let num_terms = high - low + 1;
        let sum_terms = num_terms * (low + high) / 2;
        sum += sum_terms * multiplier;
    }
    sum
}

const MAX_DIGITS: usize = 20; // because u64::MAX has 20 decimal digits
fn sum_nonprimitives_in_range(start: u64, end: u64) -> u128 {
    if end < 11 {
        return 0;
    }
    let mut sum: u128 = 0;
    for len in num_digits(start)..=num_digits(end) {
        // Clamp range to numbers with exactly `len` digits.
        let low = max(pow10(len - 1), start as u128);
        let high = min(pow10(len) - 1, end as u128);
        if low > high {
            continue;
        }
        // Candidate primitive periods (divisors of len with at least 2 repeats).
        let periods: Vec<u32> = divisors(len)
            .into_iter()
            .filter(|&d| d * 2 <= len)
            .collect();
        if periods.is_empty() {
            continue;
        }
        let mut sum_by_period = [0u128; MAX_DIGITS + 1];
        for &period in &periods {
            let multiplier = calculate_multiplier(period, len / period);
            let low = max(pow10(period - 1), ceil_div(low, multiplier));
            let high = min(pow10(period) - 1, floor_div(high, multiplier));
            if low > high {
                continue;
            }
            let num_terms = high - low + 1;
            let sum_terms = num_terms * (low + high) / 2;
            sum_by_period[period as usize] = sum_terms * multiplier;
        }
        let mut primitive_sum_by_period = [0u128; 21];
        for &period in &periods {
            let mut acc: i128 = 0;
            for d in divisors(period) {
                let mu = mobius(period / d) as i128;
                let bd = sum_by_period[d as usize] as i128;
                acc += mu * bd;
            }
            primitive_sum_by_period[period as usize] = acc as u128;
        }
        sum += periods
            .iter()
            .map(|&period| primitive_sum_by_period[period as usize])
            .sum::<u128>();
    }
    sum
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        let ranges = input
            .trim()
            .split(',')
            .map(|range| {
                let mut parts = range.trim().split('-');
                let start = parts.next().unwrap().parse::<u64>().unwrap();
                let end = parts.next().unwrap().parse::<u64>().unwrap();
                start..=end
            })
            .collect::<Vec<RangeInclusive<u64>>>();
        Box::new(Day { ranges })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "\
            11-22,\
            95-115,\
            998-1012,\
            1188511880-1188511890,\
            222220-222224,\
            1698522-1698528,\
            446443-446449,\
            38593856-38593862,\
            565653-565659,\
            824824821-824824827,\
            2121212118-2121212124";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_1(), "1227775554");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/02")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "28146997880");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "\
            11-22,\
            95-115,\
            998-1012,\
            1188511880-1188511890,\
            222220-222224,\
            1698522-1698528,\
            446443-446449,\
            38593856-38593862,\
            565653-565659,\
            824824821-824824827,\
            2121212118-2121212124";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "4174379265");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/02")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "40028128307");
    }
}
