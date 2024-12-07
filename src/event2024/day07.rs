use crate::util::parse::ParseOps;

pub fn run_day07_p1(input: &str) {
    let equations: Vec<_> = input
        .lines()
        .map(|l| {
            let (left, right) = l.split_once(": ").unwrap();
            let numbers: Vec<u32> = right.iter_unsigned().collect();
            let left: u32 = left.parse().unwrap();
            (left, numbers)
        })
        .filter(|(l, n)| check_left_within_range(*l, n))
        .collect();

    println!("{:?}", equations);

    let mut total_calibration_result = 0u32;
    for (left, numbers) in equations {
        let num_operators = numbers.len() - 1;

        //iterate adding instances of mul to the numbers in order
        //start with sum (all add) as it is the lowest number
        let mut operations = vec![Operator::Add; num_operators];
        if left == run_equation(&numbers, &operations) {
            total_calibration_result += 1;
        }

        for mul_i in 0..operations.len() {
            operations[mul_i] = Operator::Mul;
            println!("{:?}", numbers);
            println!("{:?}", operations);
            let result = run_equation(&numbers, &operations);
            if left == result {
                total_calibration_result += 1;
            }
            println!("{} {}", left, result);
            operations[mul_i] = Operator::Add;
        }
    }
}

fn check_left_within_range(left: u32, numbers: &Vec<u32>) -> bool {
    if left < numbers.iter().sum() || left > numbers.iter().product() {
        return false;
    }
    true
}

fn run_equation(numbers: &Vec<u32>, operations: &Vec<Operator>) -> u32 {
    let mut carry_value = numbers[0];
    let mut i = 1usize;
    for op in operations {
        println!("{} {}", carry_value, numbers[i]);
        carry_value = execute_operator(&op, &carry_value, &numbers[i]);
        i += 1;
    }
    carry_value
}

fn execute_operator(op: &Operator, prev: &u32, next: &u32) -> u32 {
    match op {
        Operator::Add => prev + next,
        Operator::Mul => prev * next,
    }
}

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Mul,
}

pub const TESTD7: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
1144: 4 17 714 402 8 1";
