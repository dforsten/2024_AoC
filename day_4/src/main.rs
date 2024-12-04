use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let filename = "input.txt";

    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let chars: Vec<char> = line.chars().collect();
        grid.push(chars);
    }

    let rows = grid.len();
    if rows == 0 {
        println!("0");
        return Ok(());
    }
    let cols = grid[0].len();

    let word = "XMAS";
    let word_len = word.len();
    let word_chars: Vec<char> = word.chars().collect();

    let directions = [
        (-1, -1), // NW
        (-1, 0),  // N
        (-1, 1),  // NE
        (0, -1),  // W
        (0, 1),   // E
        (1, -1),  // SW
        (1, 0),   // S
        (1, 1),   // SE
    ];

    let mut count = 0;

    for row in 0..rows {
        for col in 0..cols {
            for &(dr, dc) in &directions {
                let mut r = row as isize;
                let mut c = col as isize;
                let mut matched = true;

                for i in 0..word_len {
                    if r < 0 || r >= rows as isize || c < 0 || c >= cols as isize {
                        matched = false;
                        break;
                    }
                    if grid[r as usize][c as usize] != word_chars[i] {
                        matched = false;
                        break;
                    }
                    r += dr;
                    c += dc;
                }

                if matched {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);

    Ok(())
}