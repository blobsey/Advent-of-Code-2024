use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("input/day13.txt")
    .expect("Error reading file");
    
    part1(&input);
    part2(&input);
}

#[derive(Debug, Clone)]
struct ClawMachine {
    a1: f64,
    a2: f64,
    b1: f64,
    b2: f64,
    k1: f64,
    k2: f64,
}

impl ClawMachine {
    fn new(a1: f64, a2: f64, b1: f64, b2: f64, k1: f64, k2: f64) -> Self {
        ClawMachine {
            a1,
            a2,
            b1,
            b2,
            k1,
            k2,
        }
    }
    
    fn from_str(input: &str) -> Option<Self> {
        let re = Regex::new(r"(?x)
            Button\ A:\ X\+(\d+),\ Y\+(\d+)\n   # Parse a1 and a2
            Button\ B:\ X\+(\d+),\ Y\+(\d+)\n   # Parse b1 and b2
            Prize:\ X=(\d+),\ Y=(\d+)           # Parse k1 and k2
        ").ok()?;
        
        if let Some(caps) = re.captures(input) {
            Some(ClawMachine::new(
                caps[1].parse().ok()?,
                caps[2].parse().ok()?,
                caps[3].parse().ok()?,
                caps[4].parse().ok()?,
                caps[5].parse().ok()?,
                caps[6].parse().ok()?,
            ))
        } else {
            None
        }
    }

    fn from_str_p2(input: &str) -> Option<Self> {
        Self::from_str(input).map(|mut c| {
            c.k1 += 10_000_000_000_000.0;
            c.k2 += 10_000_000_000_000.0;
            c
        })
    }
}

fn cramers_rule(c: ClawMachine) -> Option<(u64, u64)> {
    let determinant = c.a1 * c.b2 - c.a2 * c.b1;

    if determinant == 0_f64 {
        return None;
    }

    let determinant_x = c.k1 * c.b2 - c.k2 * c.b1;
    let determinant_y = c.a1 * c.k2 - c.a2 * c.k1;
    let x = determinant_x / determinant;
    let y = determinant_y / determinant;
    if x.fract().abs() < 1e-10 && y.fract().abs() < 1e-10 && x > -1e-10 && y > -1e-10 {
        Some((x as u64, y as u64))
    }
    else {
        None
    }
}

fn part1(input: &str) {
    let input_sections = input.split("\n\n");
    
    let systems: Vec<ClawMachine> = input_sections
    .map(|section| {
        ClawMachine::from_str(section).expect(&format!("Failed to parse: {section}"))
    })
    .collect();

    let solution: u64 = systems
        .iter()
        .filter_map(|c| {
            cramers_rule(c.clone())
        })
        .map(|(a, b)| {
            a*3 + b*1
        })
        .sum();

    println!("The answer to part 1 is: {solution}");
}

fn part2(input: &str) {
    let input_sections = input.split("\n\n");
    
    let systems: Vec<ClawMachine> = input_sections
    .map(|section| {
        ClawMachine::from_str_p2(section).expect(&format!("Failed to parse: {section}"))
    })
    .collect();

    let solution: u64 = systems
        .iter()
        .filter_map(|c| {
            cramers_rule(c.clone())
        })
        .map(|(a, b)| {
            a*3 + b*1
        })
        .sum();

    println!("The answer to part 2 is: {solution}");
}