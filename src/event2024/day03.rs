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

    while index < memory.len() {
        // look for the starts that we care about 'm'ul and 'd'o
        if memory[index] != 'm' && memory[index] != 'd' {
            index += 1;
            continue;
        }

        // Check possible prefixes
        if memory[index..].starts_with(&['m', 'u', 'l', '(']) {
            index += 4;
        } else if memory[index..].starts_with(&['d', 'o', '(', ')']) {
            index += 4;
            enabled = true;
            continue;
        } else if memory[index..].starts_with(&['d', 'o', 'n', '\'', 't', '(', ')']) {
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
        println!("{} {} {}, {enabled}", first, second, product);
        part_one += product;
        if enabled {
            part_two += product;
        }
    }

    (part_one, part_two)
}
