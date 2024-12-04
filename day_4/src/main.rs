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
    if rows < 3 {
        println!("0");
        return Ok(());
    }
    let cols = grid[0].len();
    if cols < 3 {
        println!("0");
        return Ok(());
    }

    let word_variants = [vec!['M', 'A', 'S'], vec!['S', 'A', 'M']];

    let mut count = 0;

    // Start from 1 to rows-2 to avoid index out of bounds
    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            // For each combination of word variants on both diagonals
            for nw_se in &word_variants {
                for ne_sw in &word_variants {

                    // NW-SE diagonal
                    if grid[row - 1][col - 1] != nw_se[0]
                        || grid[row][col] != nw_se[1]
                        || grid[row + 1][col + 1] != nw_se[2]
                    {
                        continue;
                    }

                    // NE-SW diagonal
                    if grid[row - 1][col + 1] != ne_sw[0]
                        || grid[row][col] != ne_sw[1]
                        || grid[row + 1][col - 1] != ne_sw[2]
                    {
                        continue;
                    }

                    count += 1;
                }
            }
        }
    }

    println!("{}", count);

    Ok(())
}
