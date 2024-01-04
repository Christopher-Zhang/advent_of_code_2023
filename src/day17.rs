use std::{collections::{VecDeque, HashMap}};
use std::slice::Iter;
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum Direction {
    N,
    S,
    E,
    W,
    None
}
impl Direction {
    fn val(&self) -> i64 {
        match self {
            Direction::E => 1,
            Direction::W => -1,
            Direction::N => 2,
            Direction::S => -2,
            Direction::None => 0
        }
    }
    fn go(&self, point: &Point) -> Point {
        match self {
            Direction::E => Point{x: point.x + 1, y: point.y},
            Direction::W => Point{x: point.x - 1, y: point.y},
            Direction::N => Point{x: point.x, y: point.y - 1},
            Direction::S => Point{x: point.x, y: point.y + 1},
            Direction::None => point.clone()
        }
    }
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 5] = [Direction::N, Direction::S, Direction::E, Direction::W, Direction::None];
        DIRECTIONS.iter()
    }
}
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
struct Point {
    x: i64,
    y: i64
}
type Grid = Vec<Vec<usize>>;
pub async fn advent(data: String) -> usize {
    let grid = parse(data);
    bfs(&grid, Point{x:0, y:0}, Point{x: grid[0].len() as i64 - 1, y: grid.len() as i64 - 1})
}

fn parse(data: String) -> Grid {
    let mut grid: Grid = Vec::new();
    data.lines().for_each(|line| {
        let mut row = Vec::<usize>::new();
        line.chars().for_each(|c| row.push(c.to_digit(10).unwrap() as usize));
        grid.push(row);
    });
    grid
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
struct State {
    point: Point,
    dir: Direction,
    steps: i64,
}
impl Into<(Point, Direction, i64)> for State {
    fn into(self) -> (Point, Direction, i64) {
        (self.point, self.dir, self.steps)
    }
}
fn bfs(grid: &Grid, start: Point, end: Point) -> usize {
    let mut min_dist = usize::MAX;
    let mut q = VecDeque::<(State, usize)>::new();
    let mut seen = HashMap::<State, usize>::new();
    let initial_state = State {
        point: start,
        dir: Direction::E,
        steps: 0,
    };
    q.push_back((initial_state, 0));
    seen.insert(initial_state, 0);
    while !q.is_empty() {
        let (state, dist) = q.pop_front().unwrap();
        // println!("{:?}", state);
        let (point, dir, steps): (Point, Direction, i64) = state.into();
        if point == end {
            min_dist = std::cmp::min(min_dist, dist);
            continue;
        }
        for (next_point, next_dir) in get_neighbors(grid, state) {
            let next_steps: i64;
            if next_dir == dir {
                next_steps = steps + 1;
            }
            else {
                next_steps = 0;
            }
            let next_state = State {
                point: next_point,
                dir: next_dir,
                steps: next_steps
            };
            let next_dist = dist + grid[next_point.y as usize][next_point.x as usize];
            if next_point == end {
                min_dist = std::cmp::min(min_dist, next_dist);
                continue;
            }
            if let Some(prev_dist) = seen.get(&next_state) {
                if &next_dist < prev_dist {
                    seen.insert(next_state, next_dist);
                }
                else {
                    continue;
                }
            }
            else {
                seen.insert(next_state, next_dist);
            }
            q.push_back((next_state, next_dist));
        }
    }
    min_dist
}
fn bfs2(grid: &Grid, start: Point, end: Point) -> usize {
    let mut min_dist = usize::MAX;
    let mut q = VecDeque::<(State, usize)>::new();
    let mut seen = HashMap::<State, usize>::new();
    let initial_state = State {
        point: start,
        dir: Direction::E,
        steps: 0,
    };
    q.push_back((initial_state, 0));
    seen.insert(initial_state, 0);
    while !q.is_empty() {
        let (state, dist) = q.pop_front().unwrap();
        // println!("{:?}", state);
        let (point, dir, steps): (Point, Direction, i64) = state.into();
        // if point == end {
        //     min_dist = std::cmp::min(min_dist, dist);
        //     continue;
        // }
        for (next_point, next_dir) in get_neighbors2(grid, state) {
            let next_steps: i64;
            if next_dir == dir {
                next_steps = steps + 1;
            }
            else {
                next_steps = 0;
            }
            let next_state = State {
                point: next_point,
                dir: next_dir,
                steps: next_steps
            };
            let next_dist = dist + grid[next_point.y as usize][next_point.x as usize];
            if next_point == end {
                if next_steps >= 4 {
                    min_dist = std::cmp::min(min_dist, next_dist);
                }
                continue;
            }
            if let Some(prev_dist) = seen.get(&next_state) {
                if &next_dist < prev_dist {
                    seen.insert(next_state, next_dist);
                }
                else {
                    continue;
                }
            }
            else {
                seen.insert(next_state, next_dist);
            }
            q.push_back((next_state, next_dist));
        }
    }
    min_dist
}

fn get_neighbors(grid: &Grid, state: State) -> Vec<(Point, Direction)> {
    let mut ret = Vec::<(Point, Direction)>::new();
    for dir in Direction::iterator() {
        if dir.val() == -state.dir.val() || (dir == &state.dir && state.steps == 2) || dir.val() == 0 {
            continue;
        }
        let next = dir.go(&state.point);
        if is_valid_point(next, grid) {
            ret.push((next, dir.clone()));
        }
    }
    ret
}

fn get_neighbors2(grid: &Grid, state: State) -> Vec<(Point, Direction)> {
    let mut ret = Vec::<(Point, Direction)>::new();
    for dir in Direction::iterator() {
        if (state.steps < 3 && dir != &state.dir) || dir.val() == -state.dir.val() || (dir == &state.dir && state.steps == 9) || dir.val() == 0 {
            continue;
        }
        let next = dir.go(&state.point);
        if is_valid_point(next, grid) {
            ret.push((next, dir.clone()));
        }
    }
    ret
}

fn is_valid_point(point: Point, grid: &Grid) -> bool {
    let x = point.x;
    let y = point.y;
    x >= 0 && y >= 0 && (y as usize) < grid.len() && (x as usize) < grid[0].len()
}

pub async fn advent_2(data: String) -> usize {
    let grid = parse(data);
    bfs2(&grid, Point{x:0, y:0}, Point{x: grid[0].len() as i64 - 1, y: grid.len() as i64 - 1})
}