use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Calculate the total power sum of the minimum sets of cubes required
    let power_sum = calculate_power_sum("input.txt");

    println!("Sum of the power of the minimum sets: {}", power_sum);
}

/// Calculates the sum of the power of the minimum sets of cubes required for all games
fn calculate_power_sum(file_path: &str) -> i32 {
    let mut total_power = 0;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(game_line) = line {
                if let Some((game_id, draws)) = parse_game(&game_line) {
                    let (min_red, min_green, min_blue) = find_minimum_cubes(&draws);
                    let power = min_red * min_green * min_blue;
                    println!(
                        "Game {}: Min Set (red: {}, green: {}, blue: {}) -> Power: {}",
                        game_id, min_red, min_green, min_blue, power
                    );
                    total_power += power;
                }
            }
        }
    }

    total_power
}

/// Parses a game line into its ID and draws
fn parse_game(game_line: &str) -> Option<(i32, Vec<Vec<(i32, &str)>>)> {
    if game_line.starts_with("Game ") {
        if let Some(colon_pos) = game_line.find(':') {
            let id_part = &game_line[5..colon_pos];
            if let Ok(game_id) = id_part.trim().parse::<i32>() {
                let draws_part = &game_line[(colon_pos + 1)..];
                let draws = draws_part
                    .split(';')
                    .map(|draw| parse_draw(draw))
                    .collect::<Option<Vec<_>>>()?;
                return Some((game_id, draws));
            }
        }
    }
    None
}

/// Parses a single draw into a list of (count, color) tuples
fn parse_draw(draw: &str) -> Option<Vec<(i32, &str)>> {
    let items: Vec<&str> = draw.trim().split(',').collect();
    let mut parsed_draw = Vec::new();

    for item in items {
        if let Some((num, color)) = parse_item(item) {
            parsed_draw.push((num, color));
        }
    }

    Some(parsed_draw)
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

/// Finds the minimum number of cubes required for each color across all draws
fn find_minimum_cubes(draws: &[Vec<(i32, &str)>]) -> (i32, i32, i32) {
    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;

    for draw in draws {
        let mut red_count = 0;
        let mut green_count = 0;
        let mut blue_count = 0;

        for &(count, color) in draw {
            match color {
                "red" => red_count += count,
                "green" => green_count += count,
                "blue" => blue_count += count,
                _ => {}
            }
        }

        min_red = min_red.max(red_count);
        min_green = min_green.max(green_count);
        min_blue = min_blue.max(blue_count);
    }

    (min_red, min_green, min_blue)
}

/// Helper function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}