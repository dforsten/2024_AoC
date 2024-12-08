use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Read the input map
    let file = File::open("input.txt").expect("Failed to open input.txt");
    let reader = BufReader::new(file);

    let mut map: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        map.push(line.chars().collect());
    }

    let rows = map.len();
    let cols = if rows > 0 { map[0].len() } else { 0 };

    // Collect all antennas: frequency -> list of positions
    let mut antennas_by_freq: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for r in 0..rows {
        for c in 0..cols {
            let ch = map[r][c];
            if ch != '.' {
                antennas_by_freq.entry(ch).or_default().push((r, c));
            }
        }
    }

    // For each frequency, consider all pairs of antennas.
    // For each pair (A,B):
    //   The two antinodes are:
    //     N1 = 2 * B - A
    //     N2 = 2 * A - B
    // Here A and B are positions (r,c).
    // We must ensure N1 and N2 are within the map bounds and then record them.
    let mut antinodes = HashSet::new();
    for (_freq, positions) in antennas_by_freq.iter() {
        // If fewer than 2 antennas for this frequency, no antinodes
        if positions.len() < 2 {
            continue;
        }

        // Generate all unique pairs
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let (r1, c1) = positions[i];
                let (r2, c2) = positions[j];

                // Compute antinodes
                // N1 = 2 * (r2, c2) - (r1, c1) = (2*r2 - r1, 2*c2 - c1)
                let n1 = (2 * r2 as isize - r1 as isize, 2 * c2 as isize - c1 as isize);

                // N2 = 2 * (r1, c1) - (r2, c2) = (2*r1 - r2, 2*c1 - c2)
                let n2 = (2 * r1 as isize - r2 as isize, 2 * c1 as isize - c2 as isize);

                // Check boundaries and add if within the map
                if n1.0 >= 0 && n1.0 < rows as isize && n1.1 >= 0 && n1.1 < cols as isize {
                    antinodes.insert((n1.0 as usize, n1.1 as usize));
                }

                if n2.0 >= 0 && n2.0 < rows as isize && n2.1 >= 0 && n2.1 < cols as isize {
                    antinodes.insert((n2.0 as usize, n2.1 as usize));
                }
            }
        }
    }

    // The result is the count of unique antinodes
    let result = antinodes.len();

    println!("{}", result);
}
