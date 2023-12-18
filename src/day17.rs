use std::collections::{VecDeque, HashSet};
use priority_queue::PriorityQueue;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum Direction {
    N,
    S,
    E,
    W,
    None
}
type Grid = Vec<Vec<usize>>;
pub async fn advent(data: String) -> usize {
    let mut answer = 0;
    let mut grid: Grid = Vec::new();
    for line in data.lines() {
        let mut row: Vec<usize> = Vec::new();
        for c in line.chars() {
            row.push(char::to_digit(c, 10).unwrap() as usize);
        }
        grid.push(row);
    }


    return answer;
}

// struct Node {
//     x: i32,
//     y: i32,
//     dist: usize
// }
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}
#[derive(Hash, PartialEq, Eq, Clone)]
struct Node {
    point: Point,
    steps: usize,
    dir: Direction
}

fn djikstra(start: Point, target: Point, grid: Grid) -> usize {
    let mut queue: PriorityQueue<Node, usize> = PriorityQueue::new();
    queue.push(Node{point: start, steps: 0, dir: Direction::None}, 0);

    let mut seen: HashSet<Node> = HashSet::new();
    let mut distance = usize::MAX;
    while !queue.is_empty() {
        let (node, dist) = queue.pop().unwrap();
        let x = node.point.x;
        let y = node.point.y;
        if x < 0 || y < 0 || x as usize >= grid[0].len() || y as usize >= grid.len() {
            continue;
        }
        if node.point == target {
            distance = std::cmp::min(distance, dist);
            continue;
        }
        
        if !seen.contains(&node) {
            seen.insert(node.clone());
            let dirs = get_next_dirs(node.dir, node.steps == 3);
            for dir in dirs {
                let next_point = get_next_point(node.point, dir);
                let steps = node.steps;
                if node.dir == dir {

                }
                // let new_node = Node {
                //     point: next_point,
                //     steps
                // }
            }
        }
    }
    0
}

fn get_next_point(point: Point, dir: Direction) -> Point {
    let x = point.x;
    let y = point.y;
    match dir {
        Direction::E => {
            return Point {x: x+1, y};
        },
        Direction::N => {
            return Point {x, y: y-1};
        },
        Direction::S => {
            return Point {x, y: y+1};
        },
        Direction::W => {
            return Point {x: x-1, y};
        },
        Direction::None => {
            panic!("oops!");
        }
    };
}

fn get_next_dirs(dir: Direction, need_turn: bool) -> Vec<Direction> {
    let mut dirs: Vec<Direction> = Vec::new();
    match dir {
        Direction::E => {
            dirs.push(Direction::S);
            dirs.push(Direction::N);
            if !need_turn {
                dirs.push(Direction::E);
            }
        },
        Direction::N => {
            dirs.push(Direction::E);
            dirs.push(Direction::W);
            if !need_turn {
                dirs.push(Direction::N);
            }
        },
        Direction::S => {
            dirs.push(Direction::E);
            dirs.push(Direction::W);
            if !need_turn {
                dirs.push(Direction::S);
            }
        },
        Direction::W => {
            dirs.push(Direction::N);
            dirs.push(Direction::S);
            if !need_turn {
                dirs.push(Direction::W);
            }
        },
        Direction::None => {
            dirs.push(Direction::E);
            dirs.push(Direction::S);
        }
    }

    dirs
}

pub async fn advent_2(data: String) -> usize {
    let mut answer = 0;

    return answer;
}