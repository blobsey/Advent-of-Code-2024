fn main() {
    let input = std::fs::read_to_string("input/day01.txt")
        .expect("Error reading file");
    
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    // Parse input into two parallel vectors
    let mut col1: Vec<i32> = Vec::new();
    let mut col2: Vec<i32> = Vec::new();

    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse"))
            .collect();

        col1.push(nums[0]);
        col2.push(nums[1]);
    }

    col1.sort();
    col2.sort();

    let solution: i32 = col1.iter()
        .zip(col2.iter())
        .map(|(x, y)| (x - y).abs())
        .sum();

    println!("The solution to part 1 is: {}", solution);
}

fn part2(input: &str) {
    use std::collections::HashMap;
    let mut numbers: Vec<i32> = Vec::new();
    let mut frequencies: HashMap<i32, i32> = HashMap::new();

    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse"))
            .collect();

        numbers.push(nums[0]);
        *frequencies.entry(nums[1]).or_insert(0) += 1;
    }

    let solution: i32 = numbers.iter()
        .map(|x| x * frequencies.get(x).copied().unwrap_or(0))
        .sum();

    println!("The solution to part 2 is: {}", solution);
}