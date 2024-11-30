use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Maximum number of cubes of each color in the bag
    const MAX_RED: i32 = 12;
    const MAX_GREEN: i32 = 13;
    const MAX_BLUE: i32 = 14;

    let mut sum_of_ids = 0;

    // Open the input file
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(game_line) = line {
                // Check if the line starts with "Game "
                if game_line.starts_with("Game ") {
                    // Parse the game ID
                    if let Some(colon_pos) = game_line.find(':') {
                        let id_part = &game_line[5..colon_pos];
                        if let Ok(game_id) = id_part.trim().parse::<i32>() {
                            // Parse the draws
                            let draws_part = &game_line[(colon_pos + 1)..];
                            let draws: Vec<&str> = draws_part.split(';').collect();
                            let mut possible_game = true;

                            for draw in draws {
                                let mut red_count = 0;
                                let mut green_count = 0;
                                let mut blue_count = 0;
                                let items: Vec<&str> = draw.trim().split(',').collect();

                                for item in items {
                                    let item = item.trim();
                                    let mut parts = item.split_whitespace();

                                    if let Some(num_str) = parts.next() {
                                        if let Ok(num) = num_str.parse::<i32>() {
                                            if let Some(color) = parts.next() {
                                                match color {
                                                    "red" => red_count += num,
                                                    "green" => green_count += num,
                                                    "blue" => blue_count += num,
                                                    _ => {
                                                        // Unknown color, ignore or handle error
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                // Check if any counts exceed the maximum allowed
                                if red_count > MAX_RED || green_count > MAX_GREEN || blue_count > MAX_BLUE {
                                    possible_game = false;
                                    break; // No need to check further draws
                                }
                            }

                            if possible_game {
                                sum_of_ids += game_id;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Sum of possible game IDs: {}", sum_of_ids);
}

// Helper function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}