use std::fs;

fn main() {
    // Read the input from "input.txt"
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let lines: Vec<&str> = input.trim_end().lines().collect();

    let rows = lines.len();
    let cols = lines[0].len();

    let map: Vec<Vec<u8>> = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    // Memoization structures
    let mut paths_memo: Vec<Vec<Option<u128>>> = vec![vec![None; cols]; rows];

    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    // Identify trailheads
    let mut trailheads = Vec::new();
    for r in 0..rows {
        for c in 0..cols {
            if map[r][c] == 0 {
                trailheads.push((r, c));
            }
        }
    }

    // Compute score and rating for each trailhead
    // Rating = number of distinct hiking trails (paths) from that cell
    let mut total_rating = 0u128;
    for &(r, c) in &trailheads {
        let rating = get_paths(r, c, &map, &mut paths_memo, &directions);
        total_rating = total_rating.saturating_add(rating);
    }

    // Print the sum of the ratings of all trailheads
    println!("{}", total_rating);
}

// DFS to get number of distinct paths
fn get_paths(
    r: usize,
    c: usize,
    map: &Vec<Vec<u8>>,
    memo: &mut Vec<Vec<Option<u128>>>,
    dirs: &[(i32, i32)],
) -> u128 {
    if let Some(val) = memo[r][c] {
        return val;
    }

    let h = map[r][c];
    let rows = map.len();
    let cols = map[0].len();

    // If height = 9, there's exactly one path: this cell itself is the endpoint
    if h == 9 {
        memo[r][c] = Some(1);
        return 1;
    }

    let mut total_paths: u128 = 0;
    let next_h = h + 1;
    for &(dr, dc) in dirs {
        let nr = r as i32 + dr;
        let nc = c as i32 + dc;
        if nr < 0 || nc < 0 {
            continue;
        }
        let nr = nr as usize;
        let nc = nc as usize;
        if nr < rows && nc < cols && map[nr][nc] == next_h {
            total_paths = total_paths.saturating_add(get_paths(nr, nc, map, memo, dirs));
        }
    }

    memo[r][c] = Some(total_paths);
    total_paths
}
