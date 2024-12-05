use std::collections::{HashMap, VecDeque};
use std::fs;

/// Performs a topological sort on the provided ordering rules.
///
/// # Arguments
///
/// * `ordering_rules` - A reference to a vector of tuples `(u32, u32)` representing the ordering rules.
///
/// # Returns
///
/// If a valid topological order cannot be established (e.g., due to cycles in the graph),
/// the returned vector may be incomplete.
fn topological_sort(ordering_rules: &Vec<(u32, u32)>) -> Vec<u32> {
    // Build a graph for the pages in the update
    let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut in_degree: HashMap<u32, usize> = HashMap::new();

    // Add edges based on ordering rules
    for &(x, y) in ordering_rules {
        in_degree.entry(x).or_insert(0);
        graph.entry(x).or_default().push(y);
        *in_degree.entry(y).or_insert(0) += 1;
    }

    let mut queue: VecDeque<u32> = VecDeque::new();
    for (&v, &deg) in in_degree.iter() {
        if deg == 0 {
            queue.push_back(v);
        }
    }

    let mut sorted = Vec::new();
    while let Some(v) = queue.pop_front() {
        sorted.push(v);
        if let Some(neighbors) = graph.get(&v) {
            for &neighbor in neighbors {
                let deg = in_degree.get_mut(&neighbor).unwrap();
                *deg -= 1;
                if *deg == 0 {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    sorted
}

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

        // Filter the ordering_rules to only include those relevant for the current pages
        let filtered_rules: Vec<(u32, u32)> = ordering_rules
            .iter()
            .filter(|(x, y)| pages.contains(x) && pages.contains(y))
            .cloned()
            .collect();

        // Sort the pages based on the ordering rules
        let sorted_pages = topological_sort(&filtered_rules);

        // Check if topological sort is possible (i.e., no cycles)
        if sorted_pages.len() != pages.len() {
            println!("Cycle detected in ordering rules for update: {:?}", pages);
            continue; // Skip this update if there's a cycle
        }

        // Compare the sorted pages with the original pages to check if the update was in correct order
        let is_valid = pages == sorted_pages;

        if !is_valid {
            // The update was incorrectly ordered; use the sorted_pages for the corrected order
            let middle_index = sorted_pages.len() / 2;
            total += sorted_pages[middle_index];
        }
    }

    println!("{}", total);
}
