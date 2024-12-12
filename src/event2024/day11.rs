use std::collections::HashMap;

pub fn run_day11(input: &str) {
    let rock_line: Vec<usize> = input
        .split_whitespace()
        .map(|d| d.parse::<usize>().unwrap())
        .collect();

    let mut total_answer = 0usize;
    for rock in &rock_line {
        let answer = run_blinks(*rock, 25usize);
        println!("{:?}", answer);
        total_answer += answer;
    }
    println!("{:?}", total_answer);

    // I attempted counting and memoization but couldn't figure out the logic
    // below is another solution that answers how to use a 'counter' method

    // Allocate enough room to prevent reallocations.
    let mut stones = Vec::with_capacity(5000);
    // Maps number on stone to a much smaller contigous range of indices.
    let mut indices = HashMap::with_capacity(5000);
    // Numbers of any new stones generated during the previous blink.
    let mut todo = Vec::new();
    // Amount of each stone of a particular number.
    let mut current = Vec::new();

    // Initialize stones from input.
    for number in rock_line {
        if let Some(&index) = indices.get(&number) {
            current[index] += 1;
        } else {
            indices.insert(number, indices.len());
            todo.push(number);
            current.push(1);
        }
    }

    for _ in 0..75 {
        // If a stone number has already been seen then return its index,
        // otherwise queue it for processing during the next blink.
        let numbers = todo;
        todo = Vec::with_capacity(200);

        let mut index_of = |number| {
            let size = indices.len();
            *indices.entry(number).or_insert_with(|| {
                todo.push(number);
                size
            })
        };

        // Apply the transformation logic to stones added in the previous blink.
        for number in numbers {
            let (first, second) = if number == 0 {
                (index_of(1), usize::MAX)
            } else {
                let digits = number.ilog10() + 1;
                if digits % 2 == 0 {
                    let power = 10_usize.pow(digits / 2);
                    (index_of(number / power), index_of(number % power))
                } else {
                    (index_of(number * 2024), usize::MAX)
                }
            };

            stones.push((first, second));
        }

        // Add amount of each stone to either 1 or 2 stones in the next blink.
        let mut next = vec![0; indices.len()];

        for (&(first, second), amount) in stones.iter().zip(current) {
            next[first] += amount;
            if second != usize::MAX {
                next[second] += amount;
            }
        }

        current = next;
    }

    println!("{}", current.iter().sum::<usize>());
}

fn run_blinks(rock: usize, blinks: usize) -> usize {
    let mut total_length = 0usize;
    let mut temp_vec = vec![rock];
    for blink in 0..blinks {
        temp_vec = apply_rules(temp_vec, blink);
        // println!("{:?}", temp_vec);
        let to_add = temp_vec.len();
        total_length = to_add;
    }
    total_length
}

fn apply_rules(rock_line: Vec<usize>, blink: usize) -> Vec<usize> {
    let mut temp_rock_line = Vec::<usize>::new();
    for number in &rock_line {
        if *number == 0 {
            temp_rock_line.push(1usize);
        } else if *number == 1 {
            temp_rock_line.push(2024usize);
        } else {
            let log_number = number.ilog10() + 1;
            if log_number % 2 == 0 {
                let divisor = (10usize).pow(log_number / 2);
                let left_number = number / divisor;
                temp_rock_line.push(left_number);
                let right_number = number - left_number * divisor;
                temp_rock_line.push(right_number);
            } else {
                temp_rock_line.push(number * 2024usize);
            }
        }
    }
    temp_rock_line
}

pub const TESTD11: &str = "125 17";
pub const TESTD11P2: &str = "0 1";
