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
    // println!("{:?}", file_id);

    let mut comp_disk: VecDeque<Option<u16>> = VecDeque::with_capacity(disk.len());
    let mut disk_index = 0usize;
    while disk_index < disk.len() {
        match disk[disk_index] {
            Some(file_id) => {
                comp_disk.push_back(Some(file_id));
            }
            None => {
                let mut move_file: Option<u16>;
                loop {
                    move_file = disk.pop_back().unwrap();
                    if move_file.is_some() {
                        break;
                    }
                }
                comp_disk.push_back(move_file);
            }
        }
        // print_disk(&disk);
        // print_disk(&comp_disk);
        disk_index += 1;
    }

    // println!("{:?}", comp_desk);

    let answer = comp_disk
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let r = match f {
                Some(file_id) => *file_id as usize * i,
                None => 0usize,
            };

            // println!("{} {:?} {}", i, f, r);
            r
        })
        .sum::<usize>();

    println!("{}", answer);
}

fn print_disk(v: &VecDeque<Option<u16>>) {
    for e in v {
        match e {
            Some(n) => print!("{n}"),
            None => print!("."),
        };
    }
    print!("\n");
}

pub const TESTD9: &str = "12345";
pub const TESTD9P1: &str = "2333133121414131402";
