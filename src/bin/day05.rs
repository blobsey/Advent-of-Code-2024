use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input/day05.txt")
        .expect("Error reading file");
    
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut input_sections = input.split("\n\n");

    let mut rules: HashMap<u8, HashSet<u8>> = HashMap::new();
    let rules_input = input_sections.next().unwrap();
    rules_input
        .lines()
        .for_each(|line| {
            let mut nums = line
                .split("|")
                .map(|num| {
                    num.parse::<u8>().unwrap()
                });

            let left = nums.next().unwrap();
            let right = nums.next().unwrap();

            rules.entry(left).or_default().insert(right);
        });

    let updates_input = input_sections.next().unwrap();
    let updates: Vec<Vec<u8>> = updates_input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();

    let mut solution: u16 = 0;
    for update in updates {
        if is_correct_order_p1(&update, &rules) {
            solution += update[update.len() / 2] as u16; 
        }
    }

    println!("The answer to part 1 is: {}", solution);
}

fn is_correct_order_p1(update: &Vec<u8>, rules: &HashMap<u8, HashSet<u8>>) -> bool {
    // Convert update into a set
    let all_pages_set: HashSet<u8> = update.iter().cloned().collect();

    // Find relevant rules
    let rules_culled: HashMap<u8, HashSet<u8>> = update
        .iter()
        .map(|&page| {
            let relevant_rules: HashSet<u8> = rules
                .get(&page).unwrap_or(&HashSet::new())
                .intersection(&all_pages_set).cloned().collect();
            (page, relevant_rules)
        })
        .collect();
    
    let mut seen: HashSet<u8> = HashSet::new();
    for &page in update.iter().rev() {
        if rules_culled.get(&page).unwrap() != &seen {
            return false;
        }
        seen.insert(page);
    }

    return true;
}

fn part2(input: &str) {
    let mut input_sections = input.split("\n\n");

    let mut rules: HashMap<u8, HashSet<u8>> = HashMap::new();
    let rules_input = input_sections.next().unwrap();
    rules_input
        .lines()
        .for_each(|line| {
            let mut nums = line
                .split("|")
                .map(|num| {
                    num.parse::<u8>().unwrap()
                });

            let left = nums.next().unwrap();
            let right = nums.next().unwrap();

            rules.entry(left).or_default().insert(right);
        });

    let updates_input = input_sections.next().unwrap();
    let updates: Vec<Vec<u8>> = updates_input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();

    let mut solution: u16 = 0;
    for update in updates {
        // Find the correct order
        let mut correct_update = vec![0u8; update.len()];
        let all_pages_set = update.iter().cloned().collect();

        let rules_culled: HashMap<u8, HashSet<u8>> = update
        .iter()
        .map(|&page| {
            let relevant_rules: HashSet<u8> = rules
                .get(&page).unwrap_or(&HashSet::new())
                .intersection(&all_pages_set).cloned().collect();
            (page, relevant_rules)
        })
        .collect();

        for &page in update.iter() {
            let correct_index = rules_culled.len() - rules_culled.get(&page).unwrap_or(&HashSet::new()).len() - 1;
            correct_update[correct_index] = page;
        }

        if update != correct_update {
            solution += correct_update[correct_update.len() / 2] as u16;
        }
    }

    println!("The answer to part 2 is: {}", solution);
}
