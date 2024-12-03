use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("input/day03.txt")
        .expect("Error reading file");
    
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    // Capture two 1-3 digit numbers X and Y in the pattern mul(X,Y)
    // mul\( <-- match the mul(
    // (\d{1,3}),(\d{1,3}) <-- match 1-3 digit number, comma, then another 1-3 digit number
    // \) <-- match closing parenthesis
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();


    let solution: u32 = pattern
        .captures_iter(input)
        .map(|capture| {
            let x = capture[1].parse::<u32>().unwrap();
            let y = capture[2].parse::<u32>().unwrap();
            x * y
        })
        .sum();

    println!("The answer to part 1 is: {}", solution);
}

fn part2(input: &str) {
    let pattern = Regex::new(r#"(?x) 
        mul\((\d{1,3}),(\d{1,3})\)  # Match mul(X,Y)
        |                           # OR
        do\(\)                      # Match "do()" exactly
        |                           # OR
        don't\(\)                   # Match "don't()" exactly
    "#).unwrap();

    let mut is_enabled = true;
    let mut solution = 0;
    for capture in pattern.captures_iter(input) {
        match &capture[0] {
            "do()" => { is_enabled = true; }
            "don't()" => { is_enabled = false; }
            _ => {
                if is_enabled {
                    let x = capture[1].parse::<u32>().unwrap();
                    let y = capture[2].parse::<u32>().unwrap();
                    solution += x * y;
                }
            }
        }
    }

    println!("The answer to part 2 is: {}", solution);
}
