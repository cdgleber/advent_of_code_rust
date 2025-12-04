use std::collections::{ BTreeSet, HashSet };

pub fn solve() {
    let input = include_str!("input/day03.txt");

    //PART II
    // let mut total = 0usize;
    // TEST.lines().for_each(|l| {
    //     total += find_joltage_p1(l.as_bytes());
    // });
    // println!("{}", total);

    // PART II
    let result = input
        .lines()
        .map(|bank| {
            let mut batteries: Vec<u8> = Vec::with_capacity(12);

            let mut current_index = 0;
            for i in 0..12 {
                let (index, first_max) = &bank
                    .as_bytes()
                    [current_index..bank.len() - 11 + i].iter()
                    .enumerate()
                    .reduce(|acc, next| {
                        //find the earlist max
                        if next.1 > acc.1 {
                            next
                        } else {
                            acc
                        }
                    })
                    .unwrap();

                batteries.push(**first_max);
                current_index = current_index + index + 1;
            }
            convert_to_usize(&batteries)
        })
        .sum::<usize>();

    println!("{}", result);
}

fn convert_to_usize(s: &[u8]) -> usize {
    assert!(s.len() == 12);
    let mut mul = 100_000_000_000usize;
    let mut product = 0usize;
    for n in s {
        product += ((*n - b'0') as usize) * mul;
        mul = mul / 10;
    }
    product
}

fn find_joltage_p1(s: &[u8]) -> usize {
    //find max bytes
    let (max_i, max) = s
        .iter()
        .enumerate()
        .max_by_key(|(i, n)| **n)
        .unwrap();
    let right_max: Option<u8> = if max_i + 1 < s.len() {
        Some(*s[max_i + 1..].iter().max().unwrap())
    } else {
        None
    };
    let left_max: Option<u8> = if max_i > 0 {
        Some(*s[..max_i].iter().max().unwrap())
    } else {
        None
    };
    //convert to u8
    // println!("{:?} {:?} {:?}", left_max, max, right_max);
    let joltage = if left_max.is_some() && right_max.is_some() {
        let left = ((left_max.unwrap() - b'0') as usize) * 10 + ((max - b'0') as usize);
        let right = ((max - b'0') as usize) * 10 + ((right_max.unwrap() - b'0') as usize);
        left.max(right)
    } else if left_max.is_some() {
        ((left_max.unwrap() - b'0') as usize) * 10 + ((max - b'0') as usize)
    } else {
        ((max - b'0') as usize) * 10 + ((right_max.unwrap() - b'0') as usize)
    };

    joltage
}

const TEST: &str = "987654321111111
811111111111119
234234234234278
818181911112111";
