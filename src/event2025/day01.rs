use crate::util::parse::ParseOps;

pub fn solve() {
    let input = include_str!("input/day01.txt");
    let mut reading = 50i32;
    let mut count_zeros = 0i32;
    let mut started_at_zero = false;
    for line in input.lines() {
        let dir = line.chars().nth(0).unwrap().clone();
        let mut ticks_iter = line.iter_signed::<i32>();
        let mut ticks = ticks_iter.next().unwrap();
        count_zeros += ticks.div_euclid(100);
        ticks = ticks % 100;
        let mut under_zero = false;
        reading = match dir {
            'R' => reading.wrapping_add(ticks),
            'L' => {
                let mid = reading.wrapping_sub(ticks);
                if mid < 0 {
                    under_zero = true;
                    100 + mid
                } else {
                    mid
                }
            }
            _ => panic!(),
        };
        let over_100 = if reading > 99i32 {
            reading -= 100;
            true
        } else {
            false
        };

        //only the top conditional is needed for part 1
        if reading == 0 {
            count_zeros += 1;
            //added conditionals for part 2
        } else if over_100 && !started_at_zero {
            count_zeros += 1;
        } else if under_zero && !started_at_zero {
            count_zeros += 1;
        }
        started_at_zero = reading == 0;
    }

    println!("{}", count_zeros);
}

const I: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
L256";
