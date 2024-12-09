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

    // We know the pattern is file_length, free_length, file_length, free_length, ..., possibly ending in a file_length.
    // Let's extract these.
    let mut files_free_pairs = Vec::new();

    let mut i = 0;
    while i < digits.len() {
        // File length
        let file_len = digits[i];
        i += 1;

        // The next segment should be free length, unless we are at the end and it must be a file length (odd count)
        let mut free_len = 0;
        if i < digits.len() {
            free_len = digits[i];
            i += 1;
        }
        files_free_pairs.push((file_len, free_len));
    }

    // Next, build the actual disk layout.
    // We'll assign file IDs in the order files appear.
    let mut disk = Vec::new();
    for (i, (file_len, free_len)) in files_free_pairs.iter().enumerate() {
        if *file_len > 0 {
            for _ in 0..*file_len {
                disk.push(Some(i));
            }
        }
        if *free_len > 0 {
            for _ in 0..*free_len {
                disk.push(None);
            }
        }
    }

    // Now perform the compaction.
    // The process: while there is a gap (None) somewhere before the last file block, move a block from the end.
    loop {
        // Find leftmost free block
        let free_pos = match disk.iter().position(|&x| x.is_none()) {
            Some(pos) => pos,
            None => break, // no free space, we're done
        };

        // Find the rightmost file block
        let right_file_pos = match disk.iter().rposition(|&x| x.is_some()) {
            Some(pos) => pos,
            None => break, // no file blocks to move
        };

        if right_file_pos <= free_pos {
            // All files are to the left of this free position, so we can't fill it
            break;
        }

        // Move the file block from right_file_pos to free_pos
        let file_id = disk[right_file_pos].take();
        disk[free_pos] = file_id;
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
