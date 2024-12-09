use std::fs;

fn main() {
    // Read the puzzle input
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.trim();

    // Parse the input line into file/space segments
    let digits: Vec<usize> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    // Parse file/free pairs
    let mut files_free_pairs = Vec::new();
    let mut idx = 0;
    while idx < digits.len() {
        let file_len = digits[idx];
        idx += 1;
        let free_len = if idx < digits.len() {
            let fl = digits[idx];
            idx += 1;
            fl
        } else {
            0
        };
        files_free_pairs.push((file_len, free_len));
    }

    // Build the disk layout (file_id assigned in appearance order)
    let mut disk = Vec::new();
    let mut max_file_id = 0;
    for (i, (file_len, free_len)) in files_free_pairs.iter().enumerate() {
        for _ in 0..*file_len {
            disk.push(Some(i));
        }

        max_file_id = i;

        for _ in 0..*free_len {
            disk.push(None);
        }
    }

    // A helper function to find the continuous blocks of a given file.
    fn file_blocks(disk: &[Option<usize>], file_id: usize) -> Vec<usize> {
        disk.iter()
            .enumerate()
            .filter_map(|(pos, &block)| {
                if block == Some(file_id) {
                    Some(pos)
                } else {
                    None
                }
            })
            .collect()
    }

    // A helper function to find a contiguous free segment of a given length
    // to the left of a certain position. We'll look from the start of the disk
    // up to (but not including) the start_pos, searching for a contiguous run of None.
    fn find_free_segment(disk: &[Option<usize>], length: usize, limit: usize) -> Option<usize> {
        if length == 0 {
            return None;
        }

        let mut start: Option<usize> = None;
        let mut count = 0;

        for i in 0..limit {
            if disk[i].is_none() {
                if start.is_none() {
                    start = Some(i);
                }
                count += 1;
                if count == length {
                    // Found a segment
                    return start;
                }
            } else {
                start = None;
                count = 0;
            }
        }

        None
    }

    // Move files in order of decreasing file ID
    for fid in (0..=max_file_id).rev() {
        // Identify the file's current blocks
        let blocks = file_blocks(&disk, fid);
        if blocks.is_empty() {
            continue; // This file might have length zero or something unexpected
        }
        let file_len = blocks.len();
        let file_start = blocks[0];

        // Find a free segment to the left of file_start that fits file_len
        if let Some(free_start) = find_free_segment(&disk, file_len, file_start) {
            // Move the file there
            // First, clear the old file location
            for &pos in &blocks {
                disk[pos] = None;
            }

            // Place the file starting at free_start
            for offset in 0..file_len {
                disk[free_start + offset] = Some(fid);
            }
        } else {
            // No suitable free segment found; file stays put.
        }
    }

    // Compute the checksum: sum of position * file_id for all file blocks
    let mut checksum = 0usize;
    for (pos, block) in disk.iter().enumerate() {
        if let Some(fid) = block {
            checksum += pos * fid;
        }
    }

    println!("{}", checksum);
}
