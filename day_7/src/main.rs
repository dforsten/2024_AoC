use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut total_calibration_result = 0;

    for line_res in reader.lines() {
        let line = line_res?;
        if line.trim().is_empty() {
            continue;
        }

        // Split into target and sequence
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            continue;
        }

        let target_str = parts[0].trim();
        let target: i64 = match target_str.parse() {
            Ok(val) => val,
            Err(_) => continue,
        };

        let sequence_str = parts[1].trim();
        let numbers_res: Result<Vec<i64>, _> =
            sequence_str.split_whitespace().map(|s| s.parse()).collect();
        let numbers = match numbers_res {
            Ok(nums) => nums,
            Err(_) => continue,
        };

        // If there's only one number, just check if it equals the target.
        if numbers.len() == 1 {
            if numbers[0] == target {
                total_calibration_result += target;
            }
            continue;
        }

        let num_count = numbers.len();
        let operator_count = num_count - 1;
        let mut found_solution = false;

        // Try all combinations of operators
        // Each operator can be '+' or '*', so 2^(operator_count) combinations
        for mask in 0..(1 << operator_count) {
            let mut result = numbers[0];
            for i in 0..operator_count {
                let next_num = numbers[i + 1];
                // Determine operator from mask
                let use_multiply = (mask & (1 << i)) != 0;
                if use_multiply {
                    // Multiply
                    result = result * next_num;
                } else {
                    // Add
                    result = result + next_num;
                }
            }

            if result == target {
                found_solution = true;
                break;
            }
        }

        if found_solution {
            total_calibration_result += target;
        }
    }

    println!("{}", total_calibration_result);
    Ok(())
}
