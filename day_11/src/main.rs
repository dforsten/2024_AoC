use dashmap::DashMap;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Global memoization cache: (number, steps) -> count_of_stones
static MEMO: Lazy<DashMap<(u64, u32), u64>> = Lazy::new(|| DashMap::new());

fn main() {
    // Read the input line of initial stones
    let file = File::open("input.txt").expect("Could not open input file.");
    let line = BufReader::new(file)
        .lines()
        .next()
        .expect("No input line")
        .expect("Error reading line");
    let stones: Vec<u64> = line
        .split_whitespace()
        .map(|s| s.parse::<u64>().expect("Invalid number"))
        .collect();

    // Define the number of blinks (iterations)
    let steps = 75;

    // Compute the total count in parallel
    let total_count: u64 = stones.par_iter().map(|&st| count_stones(st, steps)).sum();

    println!("{}", total_count);
}

/// Count how many stones result after `steps` transformations, given the initial number `num`.
fn count_stones(num: u64, steps: u32) -> u64 {
    if steps == 0 {
        return 1;
    }

    // Check memo first
    if let Some(val) = MEMO.get(&(num, steps)) {
        return *val;
    }

    let result = if num == 0 {
        // Rule for 0: becomes 1
        count_stones(1, steps - 1)
    } else {
        let digits = num.to_string();
        if digits.len() % 2 == 0 {
            // Even length: split into two stones
            let (left, right) = split_even(&digits);
            let left_count = count_stones(left, steps - 1);
            let right_count = count_stones(right, steps - 1);
            left_count + right_count
        } else {
            // Odd length: multiply by 2024
            let new_num = num
                .checked_mul(2024)
                .expect("Assumption: no overflow occurs");
            count_stones(new_num, steps - 1)
        }
    };

    MEMO.insert((num, steps), result);
    result
}

/// Split an even-length digit string into two halves, remove leading zeros.
fn split_even(s: &str) -> (u64, u64) {
    let mid = s.len() / 2;
    let left_str = s[..mid].trim_start_matches('0');
    let right_str = s[mid..].trim_start_matches('0');

    let left_num = if left_str.is_empty() {
        0
    } else {
        left_str.parse::<u64>().unwrap()
    };
    let right_num = if right_str.is_empty() {
        0
    } else {
        right_str.parse::<u64>().unwrap()
    };

    (left_num, right_num)
}
