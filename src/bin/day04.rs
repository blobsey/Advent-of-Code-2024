fn main() {
    let input = std::fs::read_to_string("input/day04.txt")
        .expect("Error reading file");
    
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    
    let mut solution = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            solution += count_xmas(&grid, row, col);
        }
    }
    println!("The answer to part 1 is: {}", solution);
}

fn count_xmas(grid: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let mut count = 0;
    for row_direction in -1..=1 {
        for col_direction in -1..=1 {
            if row_direction == 0 && col_direction == 0 {
                continue;
            }

            let mut is_xmas = true;
            for (i, ch) in "XMAS".chars().enumerate() {
                let neighbor_row = (i as i32 * row_direction) + row as i32;
                let neighbor_col = (i as i32 * col_direction) + col as i32;

                // Check if negative or OOB
                if neighbor_row < 0 || 
                   neighbor_col < 0 || 
                   neighbor_row >= grid.len() as i32 || 
                   neighbor_col >= grid[0].len() as i32 ||
                   grid[neighbor_row as usize][neighbor_col as usize] != ch {
                        is_xmas = false;
                        break;
                    }
            }
            if is_xmas {
                count += 1;
            }
        }
    }
    return count;
}

fn part2(input: &str) {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut memo = vec![vec![0 as usize; grid[0].len()]; grid.len()];

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            memo_mas(&grid, &mut memo, row, col);
        }
    }

    let mut solution = 0;
    for row in memo.iter() {
        for &count in row.iter() {
            if count >= 2 {
                solution += 1;
            }
        }
    }

    println!("The answer to part 2 is: {}", solution);
}

fn memo_mas(grid: &Vec<Vec<char>>, memo: &mut Vec<Vec<usize>>, row: usize, col: usize) {
    for row_direction in [-1, 1] {
        for col_direction in [-1, 1] {
            let mut is_mas = true;
            for (i, ch) in "MAS".chars().enumerate() {
                let neighbor_row = (i as i32 * row_direction) + row as i32;
                let neighbor_col = (i as i32 * col_direction) + col as i32;

                // Check if negative or OOB
                if neighbor_row < 0 || 
                   neighbor_col < 0 || 
                   neighbor_row >= grid.len() as i32 || 
                   neighbor_col >= grid[0].len() as i32 ||
                   grid[neighbor_row as usize][neighbor_col as usize] != ch {
                    is_mas = false;
                    break;
                }
            }
            
            if is_mas {
                let middle_row = (row as i32 + row_direction) as usize;
                let middle_col = (col as i32 + col_direction) as usize;
                memo[middle_row][middle_col] += 1;
            }
        }
    }
}
