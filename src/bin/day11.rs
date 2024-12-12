use std::collections::{HashMap, VecDeque};


fn main() {
    let input = std::fs::read_to_string("input/day11.txt")
        .expect("Error reading file");
    
    part1(&input);
    part2(&input);
}

#[allow(dead_code)]
fn print_stones(stones: &Vec<u128>) {
    println!("{}", stones.iter()
    .map(|n| n.to_string())
    .collect::<Vec<String>>()
    .join(" "));
}

fn num_digits(engraving: u128) -> u32 {
    // https://math.stackexchange.com/questions/1384917/relation-between-number-of-digits-of-a-number-and-its-logarithm
    (engraving as f64).log10().floor() as u32 + 1
}

fn split_stone(index: usize, stones: &mut Vec<u128>) {
    let engraving = stones[index];
    let divisor = 10_u128.pow(num_digits(engraving) / 2);
    let left_stone = engraving / divisor;
    let right_stone = engraving % divisor;

    stones[index] = right_stone;
    stones.insert(index, left_stone);
}

fn simulate(stones: &mut Vec<u128>) {
    let mut index = 0;
    while index < stones.len() {
        let engraving = stones[index];
        if engraving == 0 {
            stones[index] = 1;
            index += 1;
        }
        else if num_digits(engraving) % 2 == 0 {
            split_stone(index, stones);
            index += 2;  // Skip the next stone since we just split one into two
        }
        else {
            stones[index] *= 2024;
            index += 1;
        }
    }
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct Node {
    value: u128,
    height: u128,
}

impl Node {
    fn new(value: u128, height: u128) -> Self {
        Node {
            value,
            height,
        }
    }
}

fn calculate_stones(stones: Vec<u128>, blinks: u128) -> u128 {
    let mut memo: HashMap<Node, u128> = HashMap::new();
    let mut solution = 0;
    for &root_stone in stones.iter() {
        let mut dfs: VecDeque<Node> = VecDeque::new();
        dfs.push_back(Node::new(root_stone, blinks));
        while !dfs.is_empty() {
            let node = dfs.pop_back().expect("Somehow dfs is empty");
            let height = node.height;
            if node.height == 0 {
                memo.insert(node, 1);
                continue;
            }

            let mut stones_subset = vec![node.value];
            simulate(&mut stones_subset);

            let new_nodes: Vec<Node> = stones_subset.iter()
                .map(|&stone| Node::new(stone, height - 1))
                .filter(|new_node| !memo.contains_key(new_node))
                .collect();

            if new_nodes.is_empty() {
                // All children are memoized, so this stone is just the sum of all of them
                let sum: u128 = stones_subset.iter()
                    .map(|&stone| memo[&Node::new(stone, height - 1)])
                    .sum();
                memo.insert(node, sum);
            } else {
                // Not all children are memoized, so need to figure out their value and revisit later
                dfs.push_back(node);
                dfs.extend(new_nodes);
            }
        }
        solution += memo[&Node::new(root_stone, blinks)];
    }
    return solution;
}


fn part1(input: &str) {
    let stones: Vec<u128> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number while parsing"))
        .collect();

    println!("The answer to part 1 is: {}", calculate_stones(stones, 25));
}

fn part2(input: &str) {
    let stones: Vec<u128> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number while parsing"))
        .collect();

    println!("The answer to part 2 is: {}", calculate_stones(stones, 75));
}