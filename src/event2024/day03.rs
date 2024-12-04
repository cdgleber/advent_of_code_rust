pub fn run_day03(input: &str) {
    let answer = parse(input);
    println!("day 03 part 1: {}\nday 03 part 2: {}", answer.0, answer.1);
}

pub fn parse(input: &str) -> (u32, u32) {
    let memory: Vec<char> = input.chars().collect();
    let mut index = 0;
    let mut enabled = true;
    let mut part_one = 0;
    let mut part_two = 0;

    let mul_needle = ['m', 'u', 'l', '('];
    let do_needle = ['d', 'o', '(', ')'];
    let dont_needle = ['d', 'o', 'n', '\'', 't', '(', ')'];

    while index < memory.len() {
        // look for the starts that we care about 'm'ul and 'd'o
        if memory[index] != 'm' && memory[index] != 'd' {
            index += 1;
            continue;
        }

        // Check possible prefixes
        if memory[index..].starts_with(&mul_needle) {
            index += 4;
        } else if memory[index..].starts_with(&do_needle) {
            index += 4;
            enabled = true;
            continue;
        } else if memory[index..].starts_with(&dont_needle) {
            index += 7;
            enabled = false;
            continue;
        } else {
            index += 1;
            continue;
        }

        // First number
        let mut first = 0;

        while memory[index].is_ascii_digit() {
            let digit = memory[index].to_digit(10).unwrap();
            first = 10 * first + digit;
            index += 1;
        }

        // First delimiter
        if memory[index] != ',' {
            continue;
        }
        index += 1;

        // Second number
        let mut second = 0;

        while memory[index].is_ascii_digit() {
            let digit = memory[index].to_digit(10).unwrap();
            second = 10 * second + digit;
            index += 1;
        }

        // Second delimiter
        if memory[index] != ')' {
            continue;
        }
        index += 1;

        // Multiply
        let product = first * second;
        // println!("{} {} {}, {enabled}", first, second, product);
        part_one += product;
        if enabled {
            part_two += product;
        }
    }

    (part_one, part_two)
}

const TESTD3: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const TESTD3P2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
