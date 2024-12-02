use std::collections::HashMap;

pub fn day01(input: &str) {
    let mut lefts = Vec::new();
    let mut rights = Vec::new();

    input.lines().for_each(|l| {
        let mut sides = l.split_whitespace();
        let left = sides.next().unwrap().parse::<usize>().unwrap();
        let right = sides.next().unwrap().parse::<usize>().unwrap();

        lefts.push(left);
        rights.push(right);
    });

    lefts.sort();
    rights.sort();

    let mut total_distance: usize = 0;
    let num_locations = lefts.len();

    for i in 0..num_locations {
        total_distance += lefts[i].abs_diff(rights[i]);
    }

    println!("day 01 part 1: {}", total_distance);

    let left_counter = count_digits(&lefts);
    let right_counter = count_digits(&rights);

    let mut similarity_score = 0usize;

    for left in &lefts {
        let right_count = right_counter.get(&left).unwrap_or(&0usize);
        similarity_score += left * right_count;
    }

    println!("day 01 part 2: {}", similarity_score);
}

fn count_digits(digits: &Vec<usize>) -> HashMap<usize, usize> {
    let mut temp = HashMap::<usize, usize>::new();

    for digit in digits {
        temp.entry(*digit).and_modify(|d| *d += 1).or_insert(1);
    }

    temp
}

const TEST: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn part_one_test() {
//         assert_eq!(day_01(TEST), 31);
//     }
// }
