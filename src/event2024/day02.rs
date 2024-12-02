pub fn run_day02(input: &str) {
    let reports: Vec<Vec<usize>> = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    println!("{:?}", reports);

    let safe_reports = reports
        .iter()
        .map(|report| {
            println!("{:?}", report);

            let mut slices = report.windows(2).map(|s| {
                let diff = s[0].abs_diff(s[1]);
                let incr = s[0] <= s[1];
                println!("{:?}, {}, {}", s, diff, incr);

                (diff < 3, incr)
            });

            let safe = slices.all(|b| ); //look for all true, safe and increasing

            if safe {
                return safe;
            } else {
                return slices.all(|b| b == false); //look for all false, safe and decreasing
            }
        })
        .collect::<Vec<_>>();

    println!("{:?}", safe_reports);
}
