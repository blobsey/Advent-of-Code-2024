use std::{collections::HashSet, hash::Hash};

fn main() {
    let input = std::fs::read_to_string("input/day06.txt")
        .expect("Error reading file");
    
    part1(&input);
    part2(&input);
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}

#[derive(Debug, PartialEq, Hash, Clone, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Hash, Clone, Eq)]
struct Guard {
    position: (usize, usize),
    direction: Direction,
}

impl Guard {
    fn new(position: (usize, usize), direction: Direction) -> Self {
        Guard {
            position,
            direction
        }
    }

    fn next_pos(&self) -> (usize, usize) {
        let (row, col) = self.position;
        match self.direction {
            Direction::Up => (row - 1, col),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col - 1),
            Direction::Right => (row, col + 1),
        }
    }

    fn rotate_cw(&mut self) {
        self.direction = get_cw_rotation(&self.direction);
    }
    
    fn step(&mut self, grid: &Vec<Vec<char>>) {
        let (row_next, col_next) = self.next_pos();
        if grid[row_next][col_next] == '#' {
            self.rotate_cw();
        }
        else {
            self.position = self.next_pos();
        }
    }

    fn is_oob(&self, grid: &Vec<Vec<char>>) -> bool {
        let (row, col) = self.position;
        grid[row][col] == '%'
    }

}

fn get_cw_rotation(direction: &Direction) -> Direction {
    return match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn part1(input: &str) {
    // Parse grid
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| {
            line.chars().collect()
        })
        .collect();

    // Pad grid with '%' to make edge-cases simpler
    let rows = grid.len();
    let cols = grid[0].len();
    let mut grid_padded: Vec<Vec<char>> = vec![vec!['%'; cols + 2]; rows + 2];
    for i in 0..rows {
        for j in 0..cols {
            grid_padded[i + 1][j + 1] = grid[i][j];
        }
    }

    // Find guard start position
    let start_pos = grid_padded.iter()
        .enumerate()
        .find_map(|(row, line)| {
            line.iter()
                .position(|&c| c == '^')
                .map(|col| (row, col))
        })
        .unwrap();
    let mut guard = Guard::new(start_pos, Direction::Up);
    grid_padded[start_pos.0][start_pos.1] = 'X';

    // Step through grid
    let mut solution = 0;
    while !guard.is_oob(&grid_padded) {
        grid_padded[guard.position.0][guard.position.1] = 'X';
        guard.step(&grid_padded);
        // print_grid(&grid_padded);
        // std::io::stdin().read(&mut [0; 1]).unwrap();
    }

    for i in 0..rows {
        for j in 0..cols {
            if grid_padded[i + 1][j + 1] == 'X' {
                solution += 1;
            }
        }
    }

    println!("The answer to part 1 is: {}", solution);
}

fn part2(input: &str) {
    // Parse grid
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| {
            line.chars().collect()
        })
        .collect();

    // Pad grid with '%' to make edge-cases simpler
    let rows = grid.len();
    let cols = grid[0].len();
    let mut grid_padded: Vec<Vec<char>> = vec![vec!['%'; cols + 2]; rows + 2];
    for i in 0..rows {
        for j in 0..cols {
            grid_padded[i + 1][j + 1] = grid[i][j];
        }
    }

    let start_pos = grid_padded.iter()
    .enumerate()
    .find_map(|(row, line)| {
        line.iter()
            .position(|&c| c == '^')
            .map(|col| (row, col))
    })
    .unwrap();
    let mut guard = Guard::new(start_pos, Direction::Up);
    
    let mut solutions: HashSet<(usize, usize)> = HashSet::new();
    let mut nonsolutions: HashSet<(usize, usize)> = HashSet::new();
    loop {

        let mut guard_clone = guard.clone();
        let next_pos = guard.next_pos();
        let next_pos_cell = grid_padded[next_pos.0][next_pos.1];

        // If the next cell is already a wall, or if it's a known non-solution, skip
        if next_pos_cell == '#' || nonsolutions.contains(&next_pos) {
            guard.step(&grid_padded);
            continue;
        }

        // If at the boundary, we're done
        if next_pos_cell == '%' {
            break;
        }

        grid_padded[next_pos.0][next_pos.1] = '#'; // Temporarily place obstacle here

        let mut seen: HashSet<Guard> = HashSet::new();  
        loop {
            if guard_clone.is_oob(&grid_padded) {
                nonsolutions.insert(next_pos);
                break;
            }

            if seen.contains(&guard_clone) {  
                solutions.insert(next_pos);
                break;
            }
            seen.insert(guard_clone.clone());

            guard_clone.step(&grid_padded);
        }

        grid_padded[next_pos.0][next_pos.1] = '.'; // Undo the temporary change
        guard.step(&grid_padded);
    }

    println!("The answer to part 2 is: {}", solutions.len());
}
