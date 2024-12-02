fn main() {
    let input = std::fs::read_to_string("input/day02.txt")
        .expect("Error reading file");
    
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut result = 0;
    for line in input.lines() {
        // Parse the line into vector of integers
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Couldn't parse input"))
            .collect();

        result += if is_safe(&levels) { 1 } else { 0 };
    }

    println!("The answer to part 1 is: {}", result);
}

fn part2(input: &str) {
    let mut result = 0;
    for line in input.lines() {
        // Parse the line into vector of integers
        let levels: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().expect("Couldn't parse input"))
                .collect();

        if is_safe(&levels) {
            result += 1;
            continue;
        }

        for i in 0..levels.len() {
            let mut levels_remove_i: Vec<i32> = levels.clone();
            levels_remove_i.remove(i);
            if is_safe(&levels_remove_i) {
                result += 1;
                break;
            }
        }
    }

    println!("The answer to part 2 is: {}", result);
}

fn is_safe(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        return true;
    }

    let mut safe_decreasing = true;
    let mut safe_increasing = true;

    for pair in levels.windows(2) {
        let (num1, num2) = (pair[0], pair[1]);
        let diff = (num1 - num2).abs();
        
        // Check if "unsafe"
        if diff > 3 || diff < 1{
            return false;
        }

        if num1 < num2 {
            safe_increasing = false;
        }

        if num1 > num2 {
            safe_decreasing = false;
        }

        if !safe_increasing && !safe_decreasing {
            return false;
        }
    }

    return true;
}