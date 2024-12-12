use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct EdgeFragment {
    // Orientation: 'H' for horizontal, 'V' for vertical
    // line: The line of the edge
    // orig_line: The line still in the region. Necessary since two regions share the same neighbor line.
    // pos: Position within the line
    orientation: char,
    line: i32,
    orig_line: usize,
    pos: usize,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = map.len();
    let cols = map[0].len();

    let mut visited = vec![vec![false; cols]; rows];
    let mut total_price = 0;

    for i in 0..rows {
        for j in 0..cols {
            if !visited[i][j] {
                let (plant_type, cells) = find_region(&map, &mut visited, (i, j));
                let edges = region_edges(&map, &cells, plant_type);
                let sides = merge_edges(&edges);
                let area = cells.len();
                println!(
                    "Price for plant type {} with an area of {} and {} sides for a total of {}",
                    plant_type,
                    area,
                    sides,
                    area * sides
                );
                total_price += area * sides;
            }
        }
    }

    println!("Total price of fencing all regions: {}", total_price);
}

/// Find all cells in the region starting from `start` by BFS.
/// Only horizontally and vertically adjacent cells of the same type are considered connected.
/// Diagonally adjacent cells are NOT considered connected.
fn find_region(
    map: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    start: (usize, usize),
) -> (char, Vec<(usize, usize)>) {
    let (sx, sy) = start;
    let plant_type = map[sx][sy];

    // Only consider orthogonal directions: up, down, left, right
    // No diagonals are included here.
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    visited[sx][sy] = true;
    let mut cells = Vec::new();
    cells.push((sx, sy));

    let mut queue = VecDeque::new();
    queue.push_back((sx, sy));

    while let Some((x, y)) = queue.pop_front() {
        for &(dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && nx < map.len() as isize && ny >= 0 && ny < map[0].len() as isize {
                let nx = nx as usize;
                let ny = ny as usize;
                // Check only same type; no diagonal check needed since we use only orth directions.
                if !visited[nx][ny] && map[nx][ny] == plant_type {
                    visited[nx][ny] = true;
                    queue.push_back((nx, ny));
                    cells.push((nx, ny));
                }
            }
        }
    }

    (plant_type, cells)
}

/// Given all cells of a region and its plant type, determine its boundary edges.
/// Each cell contributes up to 4 edges if not shared with a similar plant cell.
fn region_edges(
    map: &Vec<Vec<char>>,
    cells: &Vec<(usize, usize)>,
    plant_type: char,
) -> Vec<EdgeFragment> {
    let rows = map.len();
    let cols = map[0].len();

    let mut unique_edges = HashSet::new();

    for &(x, y) in cells {
        // top edge
        if x == 0 || map[x - 1][y] != plant_type {
            unique_edges.insert(EdgeFragment {
                orientation: 'H',
                line: x as i32 - 1,
                orig_line: x,
                pos: y,
            });
        }
        // bottom edge
        if x == rows - 1 || map[x + 1][y] != plant_type {
            unique_edges.insert(EdgeFragment {
                orientation: 'H',
                line: x as i32 + 1,
                orig_line: x,
                pos: y,
            });
        }
        // left edge
        if y == 0 || map[x][y - 1] != plant_type {
            unique_edges.insert(EdgeFragment {
                orientation: 'V',
                line: y as i32 - 1,
                orig_line: y,
                pos: x,
            });
        }
        // right edge
        if y == cols - 1 || map[x][y + 1] != plant_type {
            unique_edges.insert(EdgeFragment {
                orientation: 'V',
                line: y as i32 + 1,
                orig_line: y,
                pos: x,
            });
        }
    }

    unique_edges.into_iter().collect()
}

/// Merge all edge fragments of a region into continuous sides.
/// Each continuous horizontal or vertical line of fencing counts as a single side.
fn merge_edges(edges: &Vec<EdgeFragment>) -> usize {
    let mut horizontal_edges_map: HashMap<(i32, usize), Vec<usize>> = HashMap::new();
    let mut vertical_edges_map: HashMap<(i32, usize), Vec<usize>> = HashMap::new();

    // Separate edges by orientation and group them by their "line"
    for e in edges {
        if e.orientation == 'H' {
            horizontal_edges_map
                .entry((e.line, e.orig_line))
                .or_default()
                .push(e.pos);
        } else {
            vertical_edges_map
                .entry((e.line, e.orig_line))
                .or_default()
                .push(e.pos);
        }
    }

    // Merge line segments for horizontal and vertical edges
    let mut sides = 0;
    for (_, frags) in horizontal_edges_map {
        sides += merge_line_fragments(frags);
    }
    for (_, frags) in vertical_edges_map {
        sides += merge_line_fragments(frags);
    }

    sides
}

/// Merge line fragments into continuous lines.
fn merge_line_fragments(mut fragments: Vec<usize>) -> usize {
    fragments.sort();

    if fragments.is_empty() {
        return 0;
    }

    // We have at least one segment, let's merge them
    let mut count = 1;

    for (prev, frag) in fragments.iter().skip(1).enumerate() {
        if *frag - fragments[prev] > 1 {
            count += 1;
        }
    }

    count
}
