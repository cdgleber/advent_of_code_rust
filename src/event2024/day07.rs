use crate::util::parse::ParseOps;
use aoc_macro::repeat_ops;
use itertools::{iproduct, Itertools};
use std::collections::HashSet;

pub fn test_mcp() {
    let possible_ops = [Operator::Add, Operator::Mul];
    let v = iproduct!(possible_ops, possible_ops, possible_ops,).collect_vec();
    println!("{:?}", v);

    // let v = iproduct!().collect_vec();
    println!("{:?}", v);
}

pub fn run_day07_p1(input: &str) {
    let equations: Vec<_> = input
        .lines()
        .map(|l| {
            let (left, right) = l.split_once(": ").unwrap();
            let numbers: Vec<usize> = right.iter_unsigned().collect();
            let left: usize = left.parse().unwrap();
            (left, numbers)
        })
        .filter(|(l, n)| check_left_within_range(*l, n))
        .collect();

    // println!("{:?}", equations);

    let possible_ops = [Operator::Add, Operator::Mul];
    let mut total_calibration_result = 0usize;
    let mut functional_test_values: HashSet<usize> = HashSet::new();

    for (left, numbers) in equations {
        let num_operators = numbers.len() - 1;

        //iterate adding instances of Mul to the numbers in order
        //start with sum (all Add) as it is the lowest number
        let operations = vec![Operator::Add; num_operators];
        if left == run_equation(&numbers, &operations) {
            total_calibration_result += 1;
        }

        //tested with 1 position as Mul
        for starting_muls in 1..operations.len() {
            let mut test_operations = operations.clone();
            for _ in 0..starting_muls {}

            for mul_i in 0..operations.len() {
                test_operations[mul_i] = Operator::Mul;
                // println!("{:?}", numbers);
                // println!("{:?}", operations);
                let result = run_equation(&numbers, &operations);
                if left == result {
                    total_calibration_result += 1;
                    functional_test_values.insert(left);
                }
                // println!("{} {}", left, result);
            }
        }
    }

    println!("{}", total_calibration_result);
    println!("{:?}", functional_test_values);
    println!("{}", functional_test_values.iter().sum::<usize>());
}

fn check_left_within_range(left: usize, numbers: &Vec<usize>) -> bool {
    if left < numbers.iter().sum() || left > numbers.iter().product() {
        return false;
    }
    true
}

fn run_equation(numbers: &Vec<usize>, operations: &Vec<Operator>) -> usize {
    let mut carry_value = numbers[0];
    let mut i = 1usize;
    for op in operations {
        // println!("{} {}", carry_value, numbers[i]);
        carry_value = execute_operator(&op, &carry_value, &numbers[i]);
        i += 1;
    }
    carry_value
}

fn execute_operator(op: &Operator, prev: &usize, next: &usize) -> usize {
    match op {
        Operator::Add => prev + next,
        Operator::Mul => prev * next,
    }
}

#[derive(Debug, Clone, Copy)]
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
1144: 4 17 714 402 8 1
107: 5 2 10 7";
