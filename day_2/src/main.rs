use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Maximum number of cubes of each color in the bag
    const MAX_RED: i32 = 12;
    const MAX_GREEN: i32 = 13;
    const MAX_BLUE: i32 = 14;

    // Calculate the sum of possible game IDs
    let sum_of_ids = calculate_sum_of_possible_game_ids("input.txt", MAX_RED, MAX_GREEN, MAX_BLUE);

    println!("Sum of possible game IDs: {}", sum_of_ids);
}

/// Calculates the sum of game IDs that are possible given the constraints
fn calculate_sum_of_possible_game_ids(
    file_path: &str,
    max_red: i32,
    max_green: i32,
    max_blue: i32,
) -> i32 {
    let mut sum_of_ids = 0;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(game_line) = line {
                if let Some(game_id) = parse_game_id(&game_line) {
                    if is_game_possible(&game_line, max_red, max_green, max_blue) {
                        sum_of_ids += game_id;
                    }
                }
            }
        }
    }

    sum_of_ids
}

/// Parses the game ID from a line, if available
fn parse_game_id(game_line: &str) -> Option<i32> {
    if game_line.starts_with("Game ") {
        if let Some(colon_pos) = game_line.find(':') {
            let id_part = &game_line[5..colon_pos];
            return id_part.trim().parse::<i32>().ok();
        }
    }
    None
}

/// Checks if a game is possible based on the maximum allowed cubes
fn is_game_possible(game_line: &str, max_red: i32, max_green: i32, max_blue: i32) -> bool {
    if let Some(colon_pos) = game_line.find(':') {
        let draws_part = &game_line[(colon_pos + 1)..];
        let draws: Vec<&str> = draws_part.split(';').collect();

        for draw in draws {
            if !is_draw_valid(draw, max_red, max_green, max_blue) {
                return false;
            }
        }
    }
    true
}

/// Validates a single draw against the maximum allowed cubes
fn is_draw_valid(draw: &str, max_red: i32, max_green: i32, max_blue: i32) -> bool {
    let mut red_count = 0;
    let mut green_count = 0;
    let mut blue_count = 0;

    let items: Vec<&str> = draw.trim().split(',').collect();

    for item in items {
        if let Some((num, color)) = parse_item(item) {
            match color {
                "red" => red_count += num,
                "green" => green_count += num,
                "blue" => blue_count += num,
                _ => {}
            }
        }
    }

    red_count <= max_red && green_count <= max_green && blue_count <= max_blue
}

/// Parses an individual item like "3 red" into (3, "red")
fn parse_item(item: &str) -> Option<(i32, &str)> {
    let item = item.trim();
    let mut parts = item.split_whitespace();

    if let Some(num_str) = parts.next() {
        if let Ok(num) = num_str.parse::<i32>() {
            if let Some(color) = parts.next() {
                return Some((num, color));
            }
        }
    }
    None
}

/// Helper function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
