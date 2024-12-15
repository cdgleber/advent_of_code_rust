use crate::util::parse::*;
use crate::util::point::*;

pub fn run_day14(input: &str) {
    //robots outside the actual bathroom are in a space which is 101 tiles wide and 103 tiles tall
    //example, the robots are in a space which is only 11 tiles wide and 7 tiles tall.
    let bathroom_ht = 103i32;
    let bathroom_wt = 101i32;
    let seconds = 100i32;

    let robots: Vec<Robot> = input
        .lines()
        .map(|l| {
            let coords: Vec<i32> = l.iter_signed().collect();
            let mut end_x = (coords[0] + coords[2] * seconds) % bathroom_wt;
            if end_x < 0 {
                end_x += bathroom_wt;
            }
            let mut end_y = (coords[1] + coords[3] * seconds) % bathroom_ht;
            if end_y < 0 {
                end_y += bathroom_ht;
            }
            let robot: Robot = Robot {
                start: Point::new(coords[0], coords[1]),
                velocity: Point::new(coords[2], coords[3]),
                end: Point::new(end_x, end_y),
            };
            robot
        })
        .collect();
    // println!("{:?}", robots);

    // print_grid(bathroom_ht, bathroom_wt, &robots);

    println!("{}", calc_safety_factor(bathroom_ht, bathroom_wt, &robots));
}

fn print_grid(bathroom_ht: i32, bathroom_wt: i32, robots: &Vec<Robot>) {
    let mid_x = bathroom_wt / 2;
    let mid_y = bathroom_ht / 2;
    for y in 0..bathroom_ht {
        for x in 0..bathroom_wt {
            if y == mid_y {
                break;
            }
            if x == mid_x {
                print!(" ");
                continue;
            }
            let current_point = Point::new(x, y);
            let number = robots.iter().filter(|r| r.end == current_point).count();
            if number > 0 {
                print!("{number}");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

fn calc_safety_factor(bathroom_ht: i32, bathroom_wt: i32, robots: &Vec<Robot>) -> usize {
    let mid_x = bathroom_wt / 2;
    let mid_y = bathroom_ht / 2;
    let pos_pos = robots
        .iter()
        .filter(|r| r.end.x > mid_x && r.end.y > mid_y)
        .count();
    let neg_pos = robots
        .iter()
        .filter(|r| r.end.x < mid_x && r.end.y > mid_y)
        .count();
    let neg_neg = robots
        .iter()
        .filter(|r| r.end.x < mid_x && r.end.y < mid_y)
        .count();
    let pos_neg = robots
        .iter()
        .filter(|r| r.end.x > mid_x && r.end.y < mid_y)
        .count();

    println!("{} {} {} {}", pos_pos, neg_pos, neg_neg, pos_neg);

    pos_pos * neg_pos * neg_neg * pos_neg
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Robot {
    start: Point,
    velocity: Point,
    end: Point,
}

pub const TESTD14SM: &str = "p=0,4 v=3,-3;";

pub const TESTD14: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
