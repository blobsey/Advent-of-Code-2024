use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/day07.txt")
        .expect("Error reading file");
    
    part1(&input);
    part2(&input);
}

#[derive(Debug, Clone)]
struct Equation {
    test_value: u64,
    numbers: Vec<u64>,
}

impl Equation {
    fn new(test_value: u64, numbers: Vec<u64>) -> Self {
        Equation {
            test_value,
            numbers,
        }
    }
}

#[derive(Debug, Clone)]
enum Operator {
    Multiply,
    Add,
}

#[derive(Debug, Clone)]
enum Operator_p2 {
    Multiply,
    Add,
    Concatenate,
}

fn part1(input: &str) {
    let equations: Vec<Equation> = input
        .lines()
        .map(|line| {
            let line_split = line.split_once(':').unwrap();
            let test_value = line_split.0.parse::<u64>().unwrap();

            let numbers: Vec<u64> = line_split.1
                .trim()
                .split(' ')
                .map(|number_str| number_str.parse::<u64>().unwrap())
                .collect();

            Equation::new(test_value, numbers)
        })
        .collect();

    let mut solution = 0;
    for equation in equations.iter() {
        solution += evaluate_p1(equation);
    }

    println!("The answer to part 1 is: {}", solution);
}

fn evaluate_p1(equation: &Equation) -> u64 {
    let operators = vec![Operator::Multiply, Operator::Add];
    let num_positions = equation.numbers.len() - 1;

    for permutation in std::iter::repeat(operators)
        .take(num_positions)
        .multi_cartesian_product() {
            let mut running_total = equation.numbers[0];
            for (i, operator) in permutation.iter().enumerate() {
                match operator {
                    Operator::Add => running_total += equation.numbers[i + 1],
                    Operator::Multiply => running_total *= equation.numbers[i + 1],
                }
            }

            if running_total == equation.test_value {
                return running_total;
            }
    }

    return 0;
}

fn part2(input: &str) {
    let equations: Vec<Equation> = input
        .lines()
        .map(|line| {
            let line_split = line.split_once(':').unwrap();
            let test_value = line_split.0.parse::<u64>().unwrap();

            let numbers: Vec<u64> = line_split.1
                .trim()
                .split(' ')
                .map(|number_str| number_str.parse::<u64>().unwrap())
                .collect();

            Equation::new(test_value, numbers)
        })
        .collect();

    let mut solution = 0;
    for equation in equations.iter() {
        solution += evaluate_p2(equation);
    }

    println!("The answer to part 1 is: {}", solution);
}

fn evaluate_p2(equation: &Equation) -> u64 {
    let operators = vec![
        Operator_p2::Multiply, 
        Operator_p2::Add,
        Operator_p2::Concatenate,
    ];
    let num_positions = equation.numbers.len() - 1;

    for permutation in std::iter::repeat(operators)
        .take(num_positions)
        .multi_cartesian_product() {
            let mut running_total = equation.numbers[0];
            for (i, operator) in permutation.iter().enumerate() {
                match operator {
                    Operator_p2::Add => running_total += equation.numbers[i + 1],
                    Operator_p2::Multiply => running_total *= equation.numbers[i + 1],
                    Operator_p2::Concatenate => {
                        let (operand1, operand2) = (running_total, equation.numbers[i + 1]);

                        // https://math.stackexchange.com/questions/1384917/relation-between-number-of-digits-of-a-number-and-its-logarithm
                        let num_digits = (operand2 as f64).log10().floor() as u32 + 1;
                        
                        running_total = operand1 * 10_u64.pow(num_digits) + operand2;
                    }
                }
            }

            if running_total == equation.test_value {
                return running_total;
            }
    }

    return 0;
}