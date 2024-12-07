use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn forward_offset(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

fn find_guard_position_and_direction(map: &[Vec<char>]) -> Option<(usize, usize, Direction)> {
    for (r, row) in map.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            match cell {
                '^' => return Some((r, c, Direction::Up)),
                'v' => return Some((r, c, Direction::Down)),
                '<' => return Some((r, c, Direction::Left)),
                '>' => return Some((r, c, Direction::Right)),
                _ => {}
            }
        }
    }
    None
}

/// Simulate the guard's movement on the given map.
/// Returns true if a loop is detected, false otherwise.
fn simulate(map: &[Vec<char>], start_row: usize, start_col: usize, start_dir: Direction) -> bool {
    let rows = map.len();
    let cols = if rows > 0 { map[0].len() } else { 0 };

    let mut guard_row = start_row;
    let mut guard_col = start_col;
    let mut guard_dir = start_dir;

    // Keep track of visited states: (row, col, direction)
    let mut visited_states = HashSet::new();
    visited_states.insert((guard_row, guard_col, guard_dir));

    loop {
        let (dr, dc) = guard_dir.forward_offset();
        let front_r_isize = guard_row as isize + dr;
        let front_c_isize = guard_col as isize + dc;

        // If out of bounds, guard leaves the area - no loop
        if front_r_isize < 0
            || front_c_isize < 0
            || front_r_isize >= rows as isize
            || front_c_isize >= cols as isize
        {
            return false;
        }

        let front_r = front_r_isize as usize;
        let front_c = front_c_isize as usize;

        if map[front_r][front_c] == '#' {
            // Turn right
            guard_dir = guard_dir.turn_right();
        } else {
            // Move forward
            guard_row = front_r;
            guard_col = front_c;

            // Check if we've been in this state before
            if visited_states.contains(&(guard_row, guard_col, guard_dir)) {
                // Loop detected
                return true;
            }
            visited_states.insert((guard_row, guard_col, guard_dir));
        }
    }
}

fn main() {
    let file = File::open("input.txt").expect("Could not open input file");
    let reader = BufReader::new(file);

    let mut map: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        map.push(line.unwrap().chars().collect());
    }

    // Find the guard's initial position and direction
    let (guard_row, guard_col, guard_dir) =
        find_guard_position_and_direction(&map).expect("Guard not found in the map");

    let rows = map.len();
    let cols = if rows > 0 { map[0].len() } else { 0 };

    let mut loop_positions_count = 0;

    // Try placing a new obstruction in every possible cell that:
    // - Is not the guard's starting position
    // - Is not currently an obstruction or the guard itself
    for r in 0..rows {
        for c in 0..cols {
            if (r, c) != (guard_row, guard_col) && map[r][c] != '#' && map[r][c] != '^' && map[r][c] != 'v' && map[r][c] != '<' && map[r][c] != '>' {
                // Temporarily place an obstacle
                let mut test_map = map.clone();
                test_map[r][c] = '#';

                // Simulate and check for loop
                if simulate(&test_map, guard_row, guard_col, guard_dir) {
                    loop_positions_count += 1;
                }
            }
        }
    }

    println!("{}", loop_positions_count);
}