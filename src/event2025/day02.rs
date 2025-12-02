use crate::util::{iter::ChunkOps, parse::ParseOps};

pub fn solve() {
    let input = include_str!("input/day02.txt");
    let mut sum_invalid: usize = 0usize;
    for range_str in input.split(',') {
        let mut range_iter = range_str.iter_unsigned::<usize>();
        let min = range_iter.next().unwrap();
        let max = range_iter.next().unwrap();
        for n in min..=max {
            if check_value_p2(n) {
                sum_invalid += n;
            }
        }
    }
    println!("{}", sum_invalid);
}

//for part one, split in half and compare the sides
fn check_value_p1(n: usize) -> bool {
    let n_str = format!("{n}");
    if n_str.len().is_multiple_of(2) {
        let split_at = n_str.len() / 2;
        let (left, right) = n_str.split_at(split_at);
        if left == right {
            true
        } else {
            false
        }
    } else {
        false
    }
}

//for part two, chars from chuck size =1 to =half, look for repeating
//not efficient, may return to optimize later
fn check_value_p2(n: usize) -> bool {
    let n_str = format!("{n}");
    let half_point = n_str.len() / 2;
    let mut return_bool = false;
    for chuck_size in 1..=half_point {
        let needle: String = n_str.chars().take(chuck_size).collect();
        let all_match = n_str
            .chars()
            .collect::<Vec<char>>()
            .chunks(chuck_size)
            .all(|ch| {
                let match_ch: String = ch.iter().collect();
                needle == match_ch
            });

        if all_match {
            return_bool = true;
        }
    }
    return_bool
}

const TEST: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";
