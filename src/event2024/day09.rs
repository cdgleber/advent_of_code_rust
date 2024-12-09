use std::collections::{ HashMap, VecDeque };

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

    let comp_disk: VecDeque<Option<u16>> = part_one_compression(disk);
    let answer = get_answer(&comp_disk);
    println!("Part 1: {}", answer);

    //Changed setup to a struct rather than options.

    let mut disk: VecDeque<File> = VecDeque::new();
    let mut file_id = 0u16;
    for (i, byte) in input.bytes().enumerate() {
        let count = byte.to_decimal();
        let temp_file: File;
        if i % 2 == 0 {
            temp_file = File { id: file_id, ty: FileOption::Some, length: count };
            file_id += 1;
        } else {
            temp_file = File { id: file_id, ty: FileOption::None, length: count };
        }
        disk.push_back(temp_file);
    }

    let comp_disk_p2: VecDeque<File> = part_two_compression(disk);
    let answer = get_answer_p2(&comp_disk_p2);
    println!("Part 2: {}", answer);
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct File {
    id: u16,
    ty: FileOption,
    length: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum FileOption {
    Some,
    None,
}

fn part_one_compression(mut disk: VecDeque<Option<u16>>) -> VecDeque<Option<u16>> {
    let comp_disk_max_size = disk
        .iter()
        .filter(|b| b.is_some())
        .count();
    let mut comp_disk: VecDeque<Option<u16>> = VecDeque::with_capacity(comp_disk_max_size);
    let mut disk_index = 0usize;

    while disk_index < comp_disk_max_size {
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
        disk_index += 1;
    }

    comp_disk
}

fn part_two_compression(disk: VecDeque<File>) -> VecDeque<File> {
    let mut comp_disk: VecDeque<File> = disk.clone();
    let mut curs = disk.len() - 1;

    while curs > 0 {
        match disk[curs].ty {
            FileOption::Some => {
                let insert_pos = comp_disk
                    .iter()
                    .position(|f| f.length >= disk[curs].length && f.ty == FileOption::None);
                match insert_pos {
                    Some(pos) => {
                        let file_to_move = disk[curs].clone();
                        let file_to_move_none = File {
                            id: file_to_move.id,
                            ty: FileOption::None,
                            length: file_to_move.length,
                        };
                        comp_disk[pos].length -= file_to_move.length;
                        comp_disk.insert(pos, file_to_move);
                        let remove_file_index =
                            comp_disk.len() -
                            1 -
                            comp_disk
                                .iter()
                                .rev()
                                .position(|f| f == &disk[curs])
                                .unwrap();
                        comp_disk.remove(remove_file_index);
                        comp_disk.insert(remove_file_index, file_to_move_none);
                        // print_diskp2(&comp_disk);
                    }
                    None => (),
                }
            }
            FileOption::None => (),
        }
        curs -= 1;
    }

    comp_disk
}

fn get_answer(disk: &VecDeque<Option<u16>>) -> usize {
    disk.iter()
        .enumerate()
        .map(|(i, f)| {
            let r = match f {
                Some(file_id) => (*file_id as usize) * i,
                None => 0usize,
            };
            r
        })
        .sum::<usize>()
}

fn get_answer_p2(disk: &VecDeque<File>) -> usize {
    let mut temp_vecdeque: VecDeque<Option<u16>> = VecDeque::new();
    for e in disk {
        for _ in 0..e.length {
            match e.ty {
                FileOption::Some => temp_vecdeque.push_back(Some(e.id)),
                FileOption::None => temp_vecdeque.push_back(None),
            }
        }
    }

    get_answer(&temp_vecdeque)
}

fn print_disk(v: &VecDeque<Option<u16>>) {
    for e in v {
        match e {
            Some(n) => print!("{n}"),
            None => print!("."),
        }
    }
    print!("\n");
}

fn print_diskp2(v: &VecDeque<File>) {
    for e in v {
        for _ in 0..e.length {
            match e.ty {
                FileOption::Some => print!("{}", e.id),
                FileOption::None => print!("."),
            }
        }
    }
    print!("\n");
}

pub const TESTD9: &str = "12345";
pub const TESTD9P1: &str = "2333133121414131402";
