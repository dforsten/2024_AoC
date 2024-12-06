use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

fn main() {
    let file = File::open("input.txt").expect("Could not open input file");
    let reader = BufReader::new(file);

    let mut map: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        map.push(line.unwrap().chars().collect());
    }

    let rows = map.len();
    let cols = if rows > 0 { map[0].len() } else { 0 };

    // Locate guard's initial position and direction
    let (mut guard_row, mut guard_col, mut guard_dir) =
        find_guard_position_and_direction(&map).expect("Guard not found in the map");

    // Keep track of visited positions
    use std::collections::HashSet;
    let mut visited = HashSet::new();
    visited.insert((guard_row, guard_col));

    loop {
        // Determine if there is something in front of the guard
        let (dr, dc) = guard_dir.forward_offset();

        // Compute the new position in isize to handle negatives
        let front_r_isize = guard_row as isize + dr;
        let front_c_isize = guard_col as isize + dc;

        // Explicitly check for negative or out-of-bounds values
        if front_r_isize < 0
            || front_c_isize < 0
            || front_r_isize >= rows as isize
            || front_c_isize >= cols as isize
        {
            break;
        }

        // Convert to usize after bounds have been validated
        let front_r = front_r_isize as usize;
        let front_c = front_c_isize as usize;

        if map[front_r][front_c] == '#' {
            // Turn right
            guard_dir = guard_dir.turn_right();
        } else {
            // Move forward
            guard_row = front_r;
            guard_col = front_c;
            visited.insert((guard_row, guard_col));

            // Check if we left the area after moving
            if guard_row >= rows || guard_col >= cols {
                break;
            }
        }
    }

    // Print the number of distinct visited positions
    println!("{}", visited.len());
}
