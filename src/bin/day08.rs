use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("input/day08.txt")
        .expect("Error reading file");
    
    part1(&input);
    part2(&input);
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}

fn part1(input: &str) {
    let grid: Vec<Vec<char>> = input.lines()
        .map(|line| {
            line.chars().collect()
        })
        .collect();

    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            if ch != '.' {
                antennas.entry(ch).or_default().push((i, j));
            }
        }
    }

    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    for positions in antennas.values() {
        for i in 0..positions.len() {
            for j in 0..positions.len() {
                if i == j {
                    continue;
                }
                
                let antenna1 = positions[i];
                let antenna2 = positions[j];

                let d_row = antenna1.0 as i16 - antenna2.0 as i16;
                let d_col = antenna1.1 as i16 - antenna2.1 as i16;

                let antinode_row = antenna2.0 as i16 - d_row;
                let antinode_col = antenna2.1 as i16 - d_col;

                if antinode_row >= 0 &&
                    antinode_row < grid.len() as i16 &&
                    antinode_col >= 0 &&
                    antinode_col < grid[0].len() as i16 {
                        antinodes.insert((antinode_row as usize, antinode_col as usize));
                }
            }
        }
    }

    println!("The answer to part 1 is: {}", antinodes.len());
}

fn part2(input: &str) {
    let grid: Vec<Vec<char>> = input.lines()
        .map(|line| {
            line.chars().collect()
        })
        .collect();

    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            if ch != '.' {
                antennas.entry(ch).or_default().push((i, j));
            }
        }
    }

    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    for positions in antennas.values() {
        for i in 0..positions.len() {
            for j in 0..positions.len() {
                if i == j {
                    continue;
                }
                
                let antenna1 = positions[i];
                let antenna2 = positions[j];

                let d_row = antenna1.0 as i16 - antenna2.0 as i16;
                let d_col = antenna1.1 as i16 - antenna2.1 as i16;

                let mut temp_pos = (antenna1.0 as i16, antenna1.1 as i16);
                while !is_oob(&grid, temp_pos) {
                    antinodes.insert((temp_pos.0 as usize, temp_pos.1 as usize));
                    temp_pos.0 += d_row;
                    temp_pos.1 += d_col;
                }

                let mut temp_pos = (antenna1.0 as i16, antenna1.1 as i16);
                loop {
                    temp_pos.0 -= d_row;
                    temp_pos.1 -= d_col;
                    if !is_oob(&grid, temp_pos) {
                        break;
                    }
                    antinodes.insert((temp_pos.0 as usize, temp_pos.1 as usize));
                }
            }
        }
    }

    println!("The answer to part 2 is: {}", antinodes.len());
}

fn is_oob(grid: &Vec<Vec<char>>, pos: (i16, i16)) -> bool {
    pos.0 >= grid.len() as i16 ||
    pos.0 < 0 ||
    pos.1 >= grid[0].len() as i16 ||
    pos.1 < 0
}