use std::fs;

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");

    // Split the input into two sections
    let parts: Vec<&str> = input.split("\n\n").collect();

    // Parse ordering rules
    let ordering_rules_str = parts[0];
    let mut ordering_rules = Vec::new();
    for line in ordering_rules_str.lines() {
        let nums: Vec<&str> = line.trim().split('|').collect();
        if nums.len() == 2 {
            let x = nums[0].parse::<u32>().unwrap();
            let y = nums[1].parse::<u32>().unwrap();
            ordering_rules.push((x, y));
        }
    }

    // Parse updates
    let updates_str = parts[1];
    let mut total = 0;
    for line in updates_str.lines() {
        let pages: Vec<u32> = line
            .trim()
            .split(',')
            .map(|s| s.trim().parse::<u32>().unwrap())
            .collect();

        // Map page number to its position in the update
        let mut position = std::collections::HashMap::new();
        for (idx, &page) in pages.iter().enumerate() {
            position.insert(page, idx);
        }

        // Check if the update is in the correct order
        let mut valid = true;
        for &(x, y) in &ordering_rules {
            if let (Some(&pos_x), Some(&pos_y)) = (position.get(&x), position.get(&y)) {
                if pos_x >= pos_y {
                    valid = false;
                    break;
                }
            }
        }

        // If valid, add the middle page number to the total
        if valid {
            let middle_index = pages.len() / 2;
            total += pages[middle_index];
        }
    }

    println!("{}", total);
}
