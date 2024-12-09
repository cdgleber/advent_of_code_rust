use std::collections::VecDeque;

use crate::util::parse::*;

pub fn run_day09(input: &str) {
    let mut disk: VecDeque<Option<u16>> = VecDeque::new();
    let mut file_id = 0u16;
    for (i, byte) in input.bytes().enumerate() {
        let count = byte.to_decimal();
        if i % 2 == 0 {
            for _ in 0..count {
                disk.push_back(Some(file_id));
            }
            file_id += 1;
        } else {
            for _ in 0..count {
                disk.push_back(None);
            }
        }
    }

    // println!("{:?}", disk);

    let mut comp_desk: VecDeque<Option<u16>> = VecDeque::with_capacity(disk.len());
    let mut disk_index = 0usize;
    while disk_index < disk.len() {
        match disk[disk_index] {
            Some(file_id) => {
                comp_desk.push_back(Some(file_id));
            }
            None => {
                let move_file = disk.pop_back().unwrap();
                comp_desk.push_back(move_file);
            }
        }
        disk_index += 1;
    }

    // println!("{:?}", comp_desk);

    let answer = comp_desk
        .iter()
        .enumerate()
        .map(|(i, f)| {
            println!("{} {:?}", i, f);
            match f {
                Some(file_id) => *file_id as usize * i,
                None => 0usize,
            }
        })
        .sum::<usize>();

    println!("{}", answer);
}

pub const TESTD9: &str = "12345";
pub const TESTD9P1: &str = "2333133121414131402";
