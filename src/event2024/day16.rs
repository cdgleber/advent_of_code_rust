use crate::util::grid::*;
use crate::util::heap::*;
use crate::util::point::*;

pub fn run_day16(input: &str) {
    let grid = Grid::parse(input);
    let seen = grid.same_size_with(b'-');
    let start = grid.find(b'S').unwrap();
    let end = grid.find(b'E').unwrap();
    let starting_dir = UP;
    let total_score = find_path(&grid, start, starting_dir, end, 0usize);
}

fn find_path(grid: &Grid<u8>, p: Point, dir: Point, end: Point, total_score: usize) -> usize {
    let mut queue = MinHeap::new();
    let clockwise = (p + dir.clockwise(), 1000usize);
    let counter_clockwise = (p + dir.counter_clockwise(), 1000usize);
    let straight = (p, 1usize);

    let to_index = |pt: Point| (grid.width * pt.y + pt.x) as usize;
    let mut queue_valid_move = |pt: (Point, usize)| {
        if grid.contains(pt.0) {
            if grid[pt.0] != b'#' {
                queue.push(pt.1, to_index(pt.0));
            }
        }
    };

    [straight, clockwise, counter_clockwise].map(|possible_move| queue_valid_move(possible_move));

    while let Some(p) = queue.pop() {
        println!("{:?}", p);
    }

    total_score
}

pub const TESTD16: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
