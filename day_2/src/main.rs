use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Initialize the safe reports counter
    let mut safe_reports = 0;

    // Open the file "input.txt"
    if let Ok(lines) = read_lines("input.txt") {
        // Iterate over each line (report)
        for line in lines {
            if let Ok(report) = line {
                // Split the line into numbers
                let levels: Vec<i32> = report
                    .split_whitespace()
                    .filter_map(|s| s.parse::<i32>().ok())
                    .collect();

                // Check if the report is safe
                if is_safe(&levels) {
                    safe_reports += 1;
                } else {
                    for i in 0..levels.len() {
                        let mut modified_levels = levels.clone();
                        modified_levels.remove(i);
                        if is_safe(&modified_levels) {
                            safe_reports += 1;
                            // Stop checking after finding one valid removal
                            break;
                        }
                    }
                }
            }
        }
    } else {
        println!("Error: Could not read input.txt");
    }

    // Print the number of safe reports
    println!("{}", safe_reports);
}

// Function to read lines from a file, returning an iterator
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Function to check if a report is safe
fn is_safe(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        // A report with less than 2 levels cannot be evaluated
        return false;
    }

    // Compute the differences between adjacent levels
    let diffs: Vec<i32> = levels.windows(2).map(|w| w[1] - w[0]).collect();

    // Check that all differences are non-zero and have the same sign
    let all_increasing = diffs.iter().all(|&d| d > 0 && d >= 1 && d <= 3);
    let all_decreasing = diffs.iter().all(|&d| d < 0 && d <= -1 && d >= -3);

    // The report is safe if either all differences are increasing or all decreasing
    all_increasing || all_decreasing
}
