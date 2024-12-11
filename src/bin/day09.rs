fn main() {
    let input = std::fs::read_to_string("input/day09.txt")
        .expect("Error reading file");
    
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let input_clean = input.trim();
    let mut digits: Vec<usize> = input_clean
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    
    let num_digits = digits.len();
    let max_id = (num_digits - 1) / 2;

    let mut position = 0;
    let mut id_left = 0 as u64;
    let mut id_right = max_id as u64;
    let mut right_index = if num_digits % 2 == 1 {num_digits - 1} else {num_digits - 2};
    let mut solution: u64 = 0;

    for i in 0..num_digits {
        if i > right_index {
            break;
        }
        else if i % 2 == 0 { // Directly use left value
            for _ in 0..digits[i] {
                // println!("Adding {} * {}, remaining: (From {} left)", position, id_left,  i);
                solution += position * id_left;
                position += 1;
            }
            id_left += 1;
        }
        else { // Need to figure out how many of right_id to include
            'outer: for _ in 0..digits[i] {

                while digits[right_index] == 0 {
                    right_index -= 2;
                    id_right -= 1;
                    if i > right_index {
                        break 'outer;
                    }
                }
                solution += position * id_right;
                digits[right_index] -= 1;
                position += 1;
            }
        }
    }

    println!("\nThe answer to part 1 is: {}", solution);
}

#[derive(Debug)]
struct Block {
    start: usize,
    length: usize,
    id: i64,
}

impl Block {
    fn new(start: usize, length: usize, id: i64) -> Self {
        Block {
            start,
            length,
            id,
        }
    }
}

fn part2(input: &str) {
    let mut disk: Vec<i64> = Vec::new();
    let mut files: Vec<Block> = Vec::new();
    let mut spaces: Vec<Block> = Vec::new();

    let mut current_id = 0;
    for (i, ch) in input.trim().chars().enumerate() {
        let block_size = ch.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            files.push(Block::new(disk.len(), block_size, current_id));
            for _ in 0..block_size {
                disk.push(current_id);
            }
            current_id += 1;
        }
        else {
            spaces.push(Block::new(disk.len(), block_size, -1));
            for _ in 0..block_size {
                disk.push(-1);
            }
        }
    }

    for file in files.iter().skip(1).rev() {
        for space in spaces.iter_mut() {
            if file.length <= space.length  && file.start > space.start {
                // Remove file from old location
                for i in file.start..file.start + file.length {
                    disk[i] = -1;
                }

                // Fill in new location
                let delta = space.start + file.length;
                for i in space.start..delta {
                    disk[i] = file.id;
                }

                // Update space
                space.start = delta;
                space.length -= file.length;

                break;
            }
        }
    }

    // let disk_str: String = disk.iter()
    //     .map(|&x| if x == -1 { ".".to_string() } else { x.to_string() })
    //     .collect::<Vec<String>>()
    //     .join("");
    // println!("Disk: {}", disk_str);

    let mut solution = 0;
    for (i, &id) in disk.iter().enumerate() {
        if id != -1 {
            solution += i as i64 * id;
        }
    }

    println!("The answer to part 2 is: {}", solution);
}