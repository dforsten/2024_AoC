use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Specify the input file path
    let input_path = "input.txt";

    // Initialize the total sum
    let mut total_sum = 0;

    // Open the file and create a buffered reader
    if let Ok(lines) = read_lines(input_path) {
        // Process each line
        for line in lines {
            if let Ok(content) = line {
                // Find the first and last digits in the line
                let first_digit = content.chars().find(|c| c.is_digit(10));
                let last_digit = content.chars().rev().find(|c| c.is_digit(10));

                // Ensure both digits are found
                if let (Some(fd), Some(ld)) = (first_digit, last_digit) {
                    // Combine the digits to form a two-digit number
                    let calibration_value = format!("{}{}", fd, ld).parse::<i32>().unwrap();
                    total_sum += calibration_value;
                } else {
                    // Handle lines that do not contain at least two digits
                    eprintln!("Line skipped (does not contain at least two digits): {}", content);
                }
            }
        }
    } else {
        eprintln!("Failed to read from file: {}", input_path);
    }

    // Output the total sum
    println!("The sum of all calibration values is: {}", total_sum);
}

// Helper function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    // Open the file
    let file = File::open(filename)?;
    // Return an iterator to the reader's lines
    Ok(io::BufReader::new(file).lines())
}