use std::collections::HashSet;
use std::fs;

fn main() {
    // Read the input from "input.txt"
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let lines: Vec<&str> = input.trim_end().lines().collect();

    let rows = lines.len();
    let cols = lines[0].len();

    let mut map = Vec::with_capacity(rows);
    for line in lines {
        let row: Vec<u8> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        map.push(row);
    }

    // We'll need to memoize the results. For each cell, we store the set of reachable endpoints (9-cells).
    // Use an Option to represent uncomputed, and Some(HashSet) for computed.
    let mut memo: Vec<Vec<Option<HashSet<(usize, usize)>>>> = vec![vec![None; cols]; rows];

    // Directions for up/down/left/right
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    // Find all trailheads (height 0 cells)
    let mut trailheads = Vec::new();
    for r in 0..rows {
        for c in 0..cols {
            if map[r][c] == 0 {
                trailheads.push((r, c));
            }
        }
    }

    // Compute the score for each trailhead
    let mut total_score = 0;
    for &(r, c) in &trailheads {
        let endpoints = dfs(r, c, &map, &mut memo, &directions);
        total_score += endpoints.len();
    }

    // Print the sum of the scores
    println!("{}", total_score);
}

// Define a recursive function with memoization:
// Returns the set of (r,c) positions of height 9 reachable from cell (r,c).
fn dfs(
    r: usize,
    c: usize,
    map: &Vec<Vec<u8>>,
    memo: &mut Vec<Vec<Option<HashSet<(usize, usize)>>>>,
    dirs: &[(i32, i32)],
) -> HashSet<(usize, usize)> {
    if let Some(ref stored) = memo[r][c] {
        return stored.clone();
    }

    let h = map[r][c];
    // If this cell is height 9, it's an endpoint
    if h == 9 {
        let mut set = HashSet::new();
        set.insert((r, c));
        memo[r][c] = Some(set.clone());
        return set;
    }

    // Otherwise, we need to explore neighbors of height h+1
    let mut result = HashSet::new();
    let rows = map.len();
    let cols = map[0].len();

    let next_h = h + 1;
    for &(dr, dc) in dirs {
        let nr = (r as i32 + dr) as usize;
        let nc = (c as i32 + dc) as usize;
        if (dr < 0 && r == 0) || (dc < 0 && c == 0) || nr >= rows || nc >= cols {
            continue; // out of bounds
        }
        if map[nr][nc] == next_h {
            // Recurse
            let endpoints = dfs(nr, nc, map, memo, dirs);
            for e in endpoints {
                result.insert(e);
            }
        }
    }

    memo[r][c] = Some(result.clone());
    result
}
