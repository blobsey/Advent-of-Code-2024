use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = std::fs::read_to_string("input/day10.txt")
        .expect("Error reading file");
    
    part1(&input);
    part2(&input);
}

struct Step {
    origin: (usize, usize),
    position: (usize, usize),
}

impl Step {
    fn new(origin: (usize, usize), position: (usize, usize)) -> Self {
        Step {
            origin,
            position,
        }
    }
    fn generate_possible_next_steps(&self) -> Vec<(usize, usize)> {
        let (row, col) = self.position;
        vec![
            (row - 1, col),
            (row, col + 1),
            (row + 1, col),
            (row, col - 1),
        ]
    }
}

fn part1(input: &str) {
    let temp_grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    // Pad the grid with '#' for easy bounds checking
    let mut grid = vec![vec!['#'; temp_grid[0].len() + 2]; temp_grid.len() + 2];
    for (i, row) in temp_grid.iter().enumerate() {
        for (j, &height) in row.iter().enumerate() {
            grid[i + 1][j + 1] = height;
        }
    }
    
    // Traverse each possible trail via BFS
    let mut bfs: VecDeque<Step> = VecDeque::new();
    let mut scores: HashMap<(usize, usize), u32> = HashMap::new();
    let mut seen: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, &height) in row.iter().enumerate() {
            if height == '0' {
                bfs.push_back(Step::new((i, j), (i, j)));
                scores.insert((i, j), 0);
                seen.insert((i, j), HashSet::new());
            }
        }
    }

    while !bfs.is_empty() {
        let current_step = bfs.pop_front().unwrap();

        let seen_for_trail = seen.get_mut(&current_step.origin).unwrap();
        if seen_for_trail.contains(&current_step.position) {
            continue;
        }
        seen_for_trail.insert(current_step.position);

        let (current_row, current_col) = current_step.position;
        let current_height = grid[current_row][current_col];
        if current_height == '9' {
            scores.entry(current_step.origin).and_modify(|score| *score += 1);
            continue;
        }

        for next_pos in current_step.generate_possible_next_steps() {
            let (next_row, next_col) = next_pos;
            let next_height = grid[next_row][next_col];
            if next_height != '#' && next_height as u8 == current_height as u8 + 1 {
                bfs.push_back(Step::new(current_step.origin, next_pos));
            }
        }
    }

    let solution: u32 = scores.iter().map(|(_key, value)| value).sum();
    println!("The answer to part 1 is: {}", solution);
}

fn part2(input: &str) {
    let temp_grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    // Pad the grid with '#' for easy bounds checking
    let mut grid = vec![vec!['#'; temp_grid[0].len() + 2]; temp_grid.len() + 2];
    for (i, row) in temp_grid.iter().enumerate() {
        for (j, &height) in row.iter().enumerate() {
            grid[i + 1][j + 1] = height;
        }
    }
    
    // Traverse each possible trail via BFS
    let mut bfs: VecDeque<Step> = VecDeque::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, &height) in row.iter().enumerate() {
            if height == '0' {
                bfs.push_back(Step::new((i, j), (i, j)));
            }
        }
    }

    let mut solution = 0;
    while !bfs.is_empty() {
        let current_step = bfs.pop_front().unwrap();

        let (current_row, current_col) = current_step.position;
        let current_height = grid[current_row][current_col];
        if current_height == '9' {
            solution += 1;
            continue;
        }

        for next_pos in current_step.generate_possible_next_steps() {
            let (next_row, next_col) = next_pos;
            let next_height = grid[next_row][next_col];
            if next_height != '#' && next_height as u8 == current_height as u8 + 1 {
                bfs.push_back(Step::new(current_step.origin, next_pos));
            }
        }
    }

    println!("The answer to part 2 is: {}", solution);
}