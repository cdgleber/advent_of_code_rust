use std::usize;

use crate::util::grid::*;
use crate::util::heap::*;
use crate::util::point::*;

pub fn run_day16(input: &str) {
    let grid = Grid::parse(input);
    let start = grid.find(b'S').unwrap();
    let end = grid.find(b'E').unwrap();
    let distances = find_path(&grid, start);

    println!("{:?}", distances[start]);
    println!("{:?}", distances[end]);
}

fn find_path(grid: &Grid<u8>, start: Point) -> Grid<usize> {
    let mut dist = grid.same_size_with(usize::MAX);
    let mut seen = grid.same_size_with(false);
    dist[start] = 0usize;
    let direction = UP;
    let mut queue = MinHeap::new();
    let to_index = |pt: Point| (grid.width * pt.y + pt.x) as usize;
    queue.push(0, to_index(start));

    let valid_moves = |p: Point, dir: Point| {
        let clockwise = (p + dir.clockwise(), 1000usize, dir.clockwise());
        let counter_clockwise = (p + dir.counter_clockwise(), 1000usize, dir.counter_clockwise());
        let straight = (p + dir, 1usize, dir.clone());

        let queue_valid_move = |pt: Point| {
            if grid.contains(pt) { grid[pt] != b'#' } else { false }
        };

        [straight, clockwise, counter_clockwise]
            .iter()
            .filter(|possible_move| queue_valid_move(possible_move.0))
            .map(|p| p.clone())
            .collect::<Vec<_>>()
    };

    while let Some((min_value, index)) = queue.pop() {
        let p = grid.to_point(index).unwrap();
        seen[p] = true;
        for edge in valid_moves(p, direction) {
            if seen[edge.0] {
                continue;
            }

            let new_distance = dist[p] + edge.1;
            if new_distance < dist[edge.0] {
                dist[edge.0] = new_distance;
                queue.push(new_distance, to_index(edge.0));
                println!("{:?} {}", edge.0, new_distance);
            }
        }
    }

    dist
}

pub const TESTD16SM: &str = "####
#.E#
#.##
#.##
#.##
#.##
#S.#
####";

pub const TESTD16: &str =
    "###############
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
