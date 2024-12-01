use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Parses the input file and returns two vectors of integers representing the left and right lists.
fn parse_input_file(filename: &str) -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.len() != 2 {
            return Err(format!("Invalid line format: '{}'", line).into());
        }

        let left_num = tokens[0].parse::<i32>()?;
        let right_num = tokens[1].parse::<i32>()?;

        left_numbers.push(left_num);
        right_numbers.push(right_num);
    }

    Ok((left_numbers, right_numbers))
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input file
    let (mut left_numbers, mut right_numbers) = parse_input_file("input.txt")?;

    // Check if both lists have the same length
    if left_numbers.len() != right_numbers.len() {
        return Err("The left and right lists have different lengths.".into());
    }

    // Sort both lists
    left_numbers.sort();
    right_numbers.sort();

    // Compute the sum of absolute differences
    let total_distance: i32 = left_numbers
        .iter()
        .zip(right_numbers.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    // Output the total distance
    println!("Total distance: {}", total_distance);

    Ok(())
}
