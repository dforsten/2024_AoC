use std::collections::HashMap;
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
    let (left_numbers, right_numbers) = parse_input_file("input.txt")?;

    // Build a frequency map for the right numbers
    let mut right_freq = HashMap::new();
    for num in right_numbers {
        *right_freq.entry(num).or_insert(0) += 1;
    }

    // Compute the similarity score
    let similarity_score: i32 = left_numbers
        .iter()
        .map(|&num| num * right_freq.get(&num).unwrap_or(&0))
        .sum();

    // Output the similarity score
    println!("Similarity score: {}", similarity_score);

    Ok(())
}
