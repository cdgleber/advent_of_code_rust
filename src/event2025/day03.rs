pub fn solve() {
    TEST.lines().for_each(|l| {
        println!("{}", find_joltage(l.as_bytes()));
    });
}

fn find_joltage(s: &[u8]) -> usize {
    //find max bytes
    let (max_i, max) = s.iter().enumerate().max_by_key(|(i, n)| **n).unwrap();
    let next_max = s[max_i + 1..].iter().max().unwrap();
    //convert to u8
    println!("{} {}", max, next_max);
    let joltage = (max - b'0') as usize * 10 + (next_max - b'0') as usize;
    joltage
}

const TEST: &str = "987654321111111
811111111111119
234234234234278
818181911112111";
