use crate::util::point::*;

pub fn run_day13(input: &str) {
    let answers: Vec<_> = input
        .split("\n\n")
        .map(|claw_machine| {
            let (a, b, c) = parse(claw_machine);
            find_answer(a, b, c)
        })
        .collect();

    let min_tokens: i32 = answers
        .iter()
        .map(|cm| {
            let m = cm.iter().map(|(presses, _, _)| find_price(presses)).min();

            match m {
                Some(x) => x,
                None => 0i32,
            }
        })
        .sum();

    println!("{:?}", min_tokens);
}

fn parse(input: &str) -> (Point, Point, Point) {
    let mut lines = input.lines();
    let a_line = lines.next().unwrap().replace("Button A: X+", "");
    let (a_x, a_y) = a_line.split_once(", Y+").unwrap();
    let b_line = lines.next().unwrap().replace("Button B: X+", "");
    let (b_x, b_y) = b_line.split_once(", Y+").unwrap();
    let prize_line = lines.next().unwrap().replace("Prize: X=", "");
    let (p_x, p_y) = prize_line.split_once(", Y=").unwrap();

    let a = Point::new(a_x.parse::<i32>().unwrap(), a_y.parse::<i32>().unwrap());
    let b = Point::new(b_x.parse::<i32>().unwrap(), b_y.parse::<i32>().unwrap());
    let p = Point::new(p_x.parse::<i32>().unwrap(), p_y.parse::<i32>().unwrap());

    (a, b, p)
}

fn find_answer(a: Point, b: Point, p: Point) -> Vec<((i32, i32), Point, bool)> {
    let mut press_matrix = Vec::new();
    for b_press in 0..100 {
        for a_press in 0..100 {
            let new_loc = ORIGIN + a * a_press + b * b_press;
            press_matrix.push(((a_press, b_press), new_loc, new_loc == p));
        }
    }

    let answer: Vec<((i32, i32), Point, bool)> = press_matrix
        .iter()
        .filter(|((_, _), _, prize)| *prize)
        .map(|i| i.clone())
        .collect();

    answer
}

fn find_price(presses: &(i32, i32)) -> i32 {
    presses.0 * 3 + presses.1
}

pub const TESTD13: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
