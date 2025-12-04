use std::collections::{BTreeSet, HashSet};

pub fn solve() {
    // let input = include_str!("input/day03.txt");
    // let mut total = 0usize;
    // TEST.lines().for_each(|l| {
    //     total += find_joltage_p1(l.as_bytes());
    // });
    // println!("{}", total);

    let test = "234234234234278";
    let indecies = vec![0usize, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let length = test.len();

    let n = 5;
    let m = 3;
    // let mut all = HashSet::new();
    let mut g = Gen::new();
    let mut test = 0;
    while !g.done() {
        let mut candidates: Vec<usize> = (0..n).collect();
        let mut combination = BTreeSet::new();
        for _ in 0..m {
            let idx = g.gen(candidates.len() as usize - 1);
            combination.insert(candidates.remove(idx as usize));
        }

        println!("{:?}", combination);
        // all.insert(combination);

        // test += 1;
        // if test > 10 {
        //     break;
        // }
    }
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
    let (max_i, max) = s.iter().enumerate().max_by_key(|(i, n)| **n).unwrap();
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

struct Gen {
    started: bool,
    v: Vec<(usize, usize)>,
    p: usize,
}

impl Gen {
    fn new() -> Gen {
        Gen {
            started: false,
            v: Vec::new(),
            p: 0,
        }
    }
    fn done(&mut self) -> bool {
        if !self.started {
            self.started = true;
            return false;
        }

        for i in (0..self.v.len()).rev() {
            if self.v[i].0 < self.v[i].1 {
                self.v[i].0 += 1;
                self.v.truncate(i + 1);
                self.p = 0;
                return false;
            }
        }

        true
    }
    fn gen(&mut self, bound: usize) -> usize {
        if self.p == self.v.len() {
            self.v.push((0, 0));
        }
        self.p += 1;
        self.v[self.p - 1].1 = bound;
        self.v[self.p - 1].0
    }
}

const TEST: &str = "987654321111111
811111111111119
234234234234278
818181911112111";
