use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = std::fs::read_to_string("input/day12.txt")
        .expect("Error reading file");
    
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let unpadded_grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let rows = unpadded_grid.len() + 2;
    let cols = unpadded_grid.get(0).expect("Grid is empty").len() + 2;
    let mut grid = vec![vec!['#'; cols]; rows];
    for (i, row) in unpadded_grid.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            grid[i + 1][j + 1] = ch;
        }
    }

    let mut areas: HashMap<(usize, usize), u32> = HashMap::new();
    let mut perimeters: HashMap<(usize, usize), u32> = HashMap::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            let origin = (i, j);
            if seen.contains(&origin) { 
                continue;
            }

            let mut bfs: VecDeque<(usize, usize)> = VecDeque::new();
            let plant = grid[i][j];
            bfs.push_back(origin);
            while !bfs.is_empty() {
                let position = bfs.pop_front().expect("Popped empty queue");
                if seen.contains(&position) {
                    continue;
                }

                let mut temp_perimeter = 4;
                for neighbor_position in get_neighbors(position) {
                    let (neighbor_row, neighbor_col) = neighbor_position;
                    if grid[neighbor_row][neighbor_col] == plant {
                        temp_perimeter -= 1;
                        if !seen.contains(&neighbor_position) {
                            bfs.push_back(neighbor_position);
                        }
                    }
                }

                *perimeters.entry(origin).or_default() += temp_perimeter;
                *areas.entry(origin).or_default() += 1;
                seen.insert(position);
            }
        }
    }

    let solution: u32 = areas.iter()
        .map(|(origin, &area)| {
            let perimeter = perimeters
                .get(origin)
                .expect(&format!("Origin {origin:?} missing!"));
            perimeter * area
        })
        .sum();

    println!("The answer to part 1 is: {solution}");
}

fn get_neighbors(position: (usize, usize)) -> Vec<(usize, usize)> {
    vec![
        (position.0 - 1, position.1),
        (position.0, position.1 + 1),
        (position.0 + 1, position.1),
        (position.0, position.1 - 1),
    ]
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}

fn part2(input: &str) {
    // Observation 1: Number of corners == number of sides
    // Observation 2: Can detect corners by zooming the graph 2x and looking
    // at neighbors + diagonals. The possible cases for corners are:                                
    // XX..  XX..  .... 
    // XX..  XX..  .... 
    // XXXX  ..XX  XX.. 
    // XXXX  ..XX  XX..           
    // i.e. when a square has 7, 4, or 3 neighbors which are X, we know it's a corner
    // All other cases have different number of neighbors, source: trust me bro

    let unpadded_grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let unpadded_rows = unpadded_grid.len();
    let unpadded_cols = unpadded_grid.get(0).expect("Grid is empty").len();

    let mut zoomed_grid = vec![vec!['#'; unpadded_cols * 2]; unpadded_rows * 2];
    for i in (0..unpadded_rows * 2).step_by(2) {
        for j in (0..unpadded_cols * 2).step_by(2) {
            let ch = unpadded_grid[i/2][j/2];
            zoomed_grid[i][j] = ch;
            zoomed_grid[i][j + 1] = ch;
            zoomed_grid[i + 1][j] = ch;
            zoomed_grid[i + 1][j + 1] = ch;
        }
    }

    let rows = zoomed_grid.len() + 2;
    let cols = zoomed_grid.get(0).expect("Grid is empty").len() + 2;

    let mut grid = vec![vec!['#'; cols]; rows];
    for (i, row) in zoomed_grid.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            grid[i + 1][j + 1] = ch;
        }
    }

    let mut corners: HashMap<(usize, usize), u32> = HashMap::new();
    let mut areas: HashMap<(usize, usize), u32> = HashMap::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            let origin = (i, j);
            if seen.contains(&origin) { 
                continue;
            }

            let mut bfs: VecDeque<(usize, usize)> = VecDeque::new();
            let plant = grid[i][j];
            bfs.push_back(origin);
            while !bfs.is_empty() {
                let position = bfs.pop_front().expect("Popped empty queue");
                if seen.contains(&position) {
                    continue;
                }

                let neighbors = get_neighbors(position);
                
                for &neighbor_position in &neighbors {
                    let (neighbor_row, neighbor_col) = neighbor_position;
                    if grid[neighbor_row][neighbor_col] == plant {
                        if !seen.contains(&neighbor_position) {
                            bfs.push_back(neighbor_position);
                        }
                    }
                }

                let like_neighbors = get_neighbors_diagonals(position)
                    .iter()
                    .filter(|&&neighbor_pos| grid[neighbor_pos.0][neighbor_pos.1] == plant)
                    .count();

                if matches!(like_neighbors, 3 | 4 | 7) {
                    *corners.entry(origin).or_default() += 1;
                }

                *areas.entry(origin).or_default() += 1;
                seen.insert(position);
            }
        }
    }

    let solution: u32 = areas.iter()
    .map(|(origin, &area)| {
        let corner_count = corners
            .get(origin)
            .expect(&format!("Origin {origin:?} missing!"));
        corner_count * area / 4
    })
    .sum();

    println!("The answer to part 2 is: {solution}");
}

fn get_neighbors_diagonals(position: (usize, usize)) -> Vec<(usize, usize)> {
    vec![
        (position.0 - 1, position.1),
        (position.0, position.1 + 1),
        (position.0 + 1, position.1),
        (position.0, position.1 - 1),
        (position.0 - 1, position.1 - 1),
        (position.0 - 1, position.1 + 1),
        (position.0 + 1, position.1 - 1),
        (position.0 + 1, position.1 + 1),
    ]
}