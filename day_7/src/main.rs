use std::fs::File;
use std::io::{BufRead, BufReader};

fn concat_nums(a: i64, b: i64) -> Option<i64> {
    // Concatenation: if a=12 and b=345, result=12345
    // Convert to string and parse back:
    let a_str = a.to_string();
    let b_str = b.to_string();
    let concat_str = format!("{}{}", a_str, b_str);
    concat_str.parse::<i64>().ok()
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut total_calibration_result = 0;

    for line_res in reader.lines() {
        let line = line_res?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

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

        // If only one number, just check equality
        if numbers.len() == 1 {
            if numbers[0] == target {
                total_calibration_result += target;
            }
            continue;
        }

        let num_count = numbers.len();
        let operator_count = num_count - 1;
        let mut found_solution = false;

        // We now have three operators: +, *, ||
        // We will represent them with 0, 1, 2 for (e.g.) +, *, ||
        // So we try all combinations from 0..3^(operator_count)
        let total_combinations = 3_usize.pow(operator_count as u32);

        for comb in 0..total_combinations {
            let mut result = numbers[0];
            let mut current_comb = comb;
            let mut valid = true;

            for i in 0..operator_count {
                // Interpret current_comb as a base-3 number
                let op = current_comb % 3;
                current_comb /= 3;
                let next_num = numbers[i + 1];

                match op {
                    0 => {
                        // +
                        result = result.checked_add(next_num).expect("Overflow");
                    }
                    1 => {
                        // *
                        result = result.checked_mul(next_num).expect("Overflow");
                    }
                    2 => {
                        // ||
                        match concat_nums(result, next_num) {
                            Some(concat_val) => result = concat_val,
                            None => {
                                // If parsing fails or overflow occurs, mark invalid and break
                                valid = false;
                                break;
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }

            if valid && result == target {
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
