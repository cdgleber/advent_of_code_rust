use std::collections::HashMap;

pub fn run_day11(input: &str) {
    let mut rock_line: Vec<usize> = input
        .split_whitespace()
        .map(|d| d.parse::<usize>().unwrap())
        .collect();

    //part one
    // for _ in 0..25 {
    //     rock_line = apply_rules(rock_line);
    // }
    // println!("{:?}", rock_line.len());

    let rock_blinks_memo: HashMap<(usize, usize), usize> = HashMap::new();
    for _ in 0..25 {

        rock_line = apply_rules(rock_line);
    }
    println!("{:?}", rock_line.len());

}

fn apply_rules(rock_line: Vec<usize>) -> Vec<usize> {
    let mut temp_rock_line = Vec::<usize>::new();
    for number in rock_line {
        if number == 0 {
            temp_rock_line.push(1usize);
        } else if number == 1 {
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
