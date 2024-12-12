use std::fs;

fn main() {
    // Read the input map from the input.txt file
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = map.len();
    let cols = map[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut total_price = 0;

    // Directions for exploring neighbors (up, down, left, right)
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    // Iterate through each cell of the map
    for i in 0..rows {
        for j in 0..cols {
            if !visited[i][j] {
                let plant_type = map[i][j];
                let (area, perimeter) = dfs(&map, &mut visited, i, j, plant_type, &directions);
                total_price += area * perimeter;
            }
        }
    }

    println!("Total price of fencing all regions: {}", total_price);
}

// Helper function for DFS to calculate area and perimeter
fn dfs(
    map: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    x: usize,
    y: usize,
    plant_type: char,
    directions: &[(isize, isize)],
) -> (usize, usize) {
    let mut stack = vec![(x, y)];
    let mut area = 0;
    let mut perimeter = 0;

    while let Some((cx, cy)) = stack.pop() {
        if visited[cx][cy] {
            continue;
        }

        visited[cx][cy] = true;
        area += 1;

        for &(dx, dy) in directions {
            let nx = cx as isize + dx;
            let ny = cy as isize + dy;

            if nx >= 0 && nx < map.len() as isize && ny >= 0 && ny < map[0].len() as isize {
                let nx = nx as usize;
                let ny = ny as usize;

                if map[nx][ny] == plant_type && !visited[nx][ny] {
                    stack.push((nx, ny));
                } else if map[nx][ny] != plant_type {
                    perimeter += 1;
                }
            } else {
                // Edge of the map contributes to the perimeter
                perimeter += 1;
            }
        }
    }

    (area, perimeter)
}
