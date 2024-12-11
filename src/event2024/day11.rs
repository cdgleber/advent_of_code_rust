use std::collections::LinkedList;

pub fn run_day11(input: &str) {
    let mut rock_line: LinkedList<usize> = input
        .split_whitespace()
        .map(|d| d.parse::<usize>().unwrap())
        .collect();

    // println!("{:?}", rock_line);
    for _ in 0..75 {
        // part 1 = 25, part 2 = 75
        // println!("{:?}", t);
        rock_line = apply_rules(rock_line);
    }
    println!("{:?}", rock_line.len());
}

fn apply_rules(rock_line: LinkedList<usize>) -> LinkedList<usize> {
    let mut temp_rock_line = LinkedList::<usize>::new();
    for number in rock_line {
        if number == 0 {
            temp_rock_line.push_back(1usize)
        } else if number == 1 {
            temp_rock_line.push_back(2024usize)
        } else {
            let log_number = number.ilog10() + 1;
            if log_number % 2 == 0 {
                let divisor = 10usize.pow(log_number / 2);
                let left_number = number / divisor;
                temp_rock_line.push_back(left_number);
                let right_number = number - (left_number * divisor);
                temp_rock_line.push_back(right_number);
            } else {
                temp_rock_line.push_back(number * 2024usize)
            }
        }
    }
    temp_rock_line
}

pub const TESTD11: &str = "125 17";
