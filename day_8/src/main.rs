use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn gcd(a: isize, b: isize) -> isize {
    let mut a = a.abs();
    let mut b = b.abs();
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn main() {
    let file = File::open("input.txt").expect("Failed to open input.txt");
    let reader = BufReader::new(file);

    let mut map: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        map.push(line.chars().collect());
    }

    let rows = map.len();
    let cols = if rows > 0 { map[0].len() } else { 0 };

    // Collect all antennas by frequency.
    let mut antennas_by_freq: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for r in 0..rows {
        for c in 0..cols {
            let ch = map[r][c];
            if ch != '.' {
                antennas_by_freq.entry(ch).or_default().push((r, c));
            }
        }
    }

    let mut antinodes = HashSet::new();

    // For each frequency group, consider all pairs of antennas to define lines
    for (_freq, positions) in antennas_by_freq.iter() {
        if positions.len() < 2 {
            // If there's only one antenna of a particular frequency, no antinodes from it.
            continue;
        }

        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let (r1, c1) = positions[i];
                let (r2, c2) = positions[j];

                let dr = r2 as isize - r1 as isize;
                let dc = c2 as isize - c1 as isize;

                // Reduce the direction vector to its smallest step
                let g = gcd(dr, dc);
                let dr_step = dr / g;
                let dc_step = dc / g;

                // We will use (r1, c1) as an anchor and go in both directions along the line.

                // Direction 1: forward line
                {
                    let mut k = 0isize;
                    loop {
                        let rr = r1 as isize + k * dr_step;
                        let cc = c1 as isize + k * dc_step;
                        if rr < 0 || rr >= rows as isize || cc < 0 || cc >= cols as isize {
                            break;
                        }
                        antinodes.insert((rr as usize, cc as usize));
                        k += 1;
                    }
                }

                // Direction 2: backward line
                {
                    let mut k = -1isize;
                    loop {
                        let rr = r1 as isize + k * dr_step;
                        let cc = c1 as isize + k * dc_step;
                        if rr < 0 || rr >= rows as isize || cc < 0 || cc >= cols as isize {
                            break;
                        }
                        antinodes.insert((rr as usize, cc as usize));
                        k -= 1;
                    }
                }
            }
        }
    }

    let result = antinodes.len();
    println!("{}", result);
}
