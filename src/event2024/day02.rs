pub fn run_day02(input: &str) {
    let reports: Vec<Vec<usize>> = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    // println!("{:?}", reports);

    let part_one_safe_reports = reports
        .iter()
        .map(|report| {
            // println!("{:?}", report);
            part_one_safe(report)
        })
        .collect::<Vec<_>>();

    let answer: usize = part_one_safe_reports
        .iter()
        .map(|r| if *r { 1 } else { 0 })
        .sum();

    println!("day 02 part 1: {}", answer);

    let part_two_safe_reports = reports
        .iter()
        .map(|report| {
            // println!("{:?}", report);
            part_two_safe(report)
        })
        .collect::<Vec<_>>();

    let answer: usize = part_two_safe_reports
        .iter()
        .map(|r| if *r { 1 } else { 0 })
        .sum();

    println!("day 02 part 2: {}", answer);
}

fn iter_all_same<T: PartialEq>(arr: &[T]) -> bool {
    arr.windows(2).all(|w| w[0] == w[1])
}

fn part_one_safe(report: &Vec<usize>) -> bool {
    let slices: Vec<(bool, bool, bool)> = report
        .windows(2)
        .map(|s| {
            let diff: isize = (s[0] as isize) - (s[1] as isize);
            (diff <= 3 && diff >= -3, diff >= 0, diff <= 0)
        })
        .collect();

    if slices.iter().all(|s| s.0) {
        //all differences must be -3 to 3 then look for all incr or decr
        iter_all_same(&slices)
    } else {
        false
    }
}

fn part_two_safe(report: &Vec<usize>) -> bool {
    let length = report.len();
    let mut safe = part_one_safe(report);

    if !safe {
        for skip_index in 0..length {
            let mut temp_vec = Vec::new();
            for i in 0..length {
                if skip_index != i {
                    temp_vec.push(report[i]);
                }
            }

            let p1 = part_one_safe(&temp_vec);

            // println!("{:?}, {}", temp_vec, p1);

            if p1 {
                safe = true;
            }
        }
    }

    safe
}
