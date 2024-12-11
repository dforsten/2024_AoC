use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Read the initial arrangement from input.txt
    let file = File::open("input.txt").expect("Could not open input file.");
    let mut lines = BufReader::new(file).lines();
    let initial_line = lines
        .next()
        .expect("No line found in input.")
        .expect("Failed to read line.");

    let mut stones: Vec<String> = initial_line
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    // Apply transformations 25 times
    for _ in 0..25 {
        stones = transform_stones(&stones);
    }

    // Print the number of stones after 25 transformations
    println!("{}", stones.len());
}

/// Transform all stones according to the given rules.
fn transform_stones(stones: &[String]) -> Vec<String> {
    let mut new_stones = Vec::new();
    for stone in stones {
        let digits = stone;
        if digits == "0" {
            // Rule 1: stone is 0 -> replaced by 1
            new_stones.push("1".to_string());
        } else if is_even_length(digits) {
            // Rule 2: even number of digits -> split into two stones
            let (left, right) = split_even(digits);
            new_stones.push(left);
            new_stones.push(right);
        } else {
            // Rule 3: multiply by 2024
            let product = multiply_string_by_2024(digits);
            new_stones.push(product);
        }
    }
    new_stones
}

/// Check if a string has an even number of digits.
fn is_even_length(s: &str) -> bool {
    s.len() % 2 == 0
}

/// Split an even-length digit string into two halves, removing leading zeros.
fn split_even(s: &str) -> (String, String) {
    let mid = s.len() / 2;
    let left_half = &s[..mid];
    let right_half = &s[mid..];

    let left_num = strip_leading_zeros(left_half);
    let right_num = strip_leading_zeros(right_half);

    (left_num, right_num)
}

/// Strip leading zeros from a digit string, defaulting to "0" if empty.
fn strip_leading_zeros(s: &str) -> String {
    let s = s.trim_start_matches('0');
    if s.is_empty() {
        "0".to_string()
    } else {
        s.to_string()
    }
}

/// Multiply a potentially large number (given as a string) by 2024 using string arithmetic.
fn multiply_string_by_2024(num_str: &str) -> String {
    let num = num_str;
    let multiplier = "2024";
    string_multiply(num, multiplier)
}

/// Would also work with the given puzzle input, but cannot handle input larger than u128.
//fn multiply_string(a: &str, b: u128) -> String {
//    let a = a.parse::<u128>().unwrap();
//    let result = a * b;
//    result.to_string()
//}

/// Multiply two large numbers represented as strings.
fn string_multiply(a: &str, b: &str) -> String {
    if a == "0" || b == "0" {
        return "0".to_string();
    }

    // Reverse for convenience
    let a_rev: Vec<u32> = a.chars().rev().map(|c| c.to_digit(10).unwrap()).collect();
    let b_rev: Vec<u32> = b.chars().rev().map(|c| c.to_digit(10).unwrap()).collect();

    let mut result = vec![0u32; a.len() + b.len()];

    for i in 0..a_rev.len() {
        for j in 0..b_rev.len() {
            let mul = a_rev[i] * b_rev[j];
            let sum = result[i + j] + mul;
            result[i + j] = sum % 10;
            result[i + j + 1] += sum / 10;
        }
    }

    // Remove leading zeros
    while result.len() > 1 && *result.last().unwrap() == 0 {
        result.pop();
    }

    result
        .iter()
        .rev()
        .map(|d| std::char::from_digit(*d, 10).unwrap())
        .collect()
}
