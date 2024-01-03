use std::collections::{HashSet, VecDeque};
use cached::proc_macro::cached;
type Grid = Vec<Vec<char>>;
type Row = Vec<char>;
type Point = (usize, usize);
pub async fn advent(data: String) -> usize {
    let answer;
    let mut seen: HashSet<Point> = HashSet::new();
    let (grid, start) = parse(data);
    answer = solve(&grid, &mut seen, start, 64);
    print_seen(&grid, &seen);
    return answer;
}

fn solve(grid: &Grid, seen: &mut HashSet<Point>, start: Point, steps: usize) -> usize {
    let mut count = 0;
    let mut queue: VecDeque<Point> = VecDeque::new();
    queue.push_back(start);
    for current_step in 0..steps+1 {
        // println!("step {current_step}");
        let mut next: VecDeque<Point> = VecDeque::new();
        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            if seen.contains(&current) {
                continue;
            }
            if current_step == steps {
                count += 1;
                seen.insert(current.clone());
            }

            let neighbors = get_valid_neighbors(grid, current, seen);
            for neighbor in neighbors {
                if !next.contains(&neighbor) {
                    next.push_back(neighbor);
                }
            }
        }
        queue = next;
    }
    count
}

#[cached(key="String", convert=r##"{ format!("{},{}", point.0, point.1) }"##)]
fn get_valid_neighbors(grid: &Grid, point: Point, seen: &HashSet<Point>) -> Vec<Point> {
    let mut neighbors: Vec<Point> = Vec::new();
    let directions: Vec<(i64, i64)> = vec![(0,1),(0,-1),(1,0),(-1,0)];

    let x = point.0 as i64;
    let y = point.1 as i64;

    for dir in directions {
        if is_valid(x+dir.0, y+dir.1, grid, seen) {
            neighbors.push(((x+dir.0) as usize, (y+dir.1) as usize));
        }
    }

    neighbors
}

fn is_valid(x: i64, y: i64, grid: &Grid, seen: &HashSet<Point>) -> bool {
    if x < 0 || y < 0 {
        return false;
    }
    let x = x as usize;
    let y = y as usize;
    if y >= grid.len() || x >= grid[0].len() || grid[y][x] == '#' {//|| seen.contains(&(x,y)){
        return false;
    }
    true
}
fn is_valid2(x: i64, y: i64, grid: &Grid) -> bool {
    let mut x = x % grid[0].len() as i64;
    let mut y = y % grid.len() as i64;
    if x < 0 {
        x = grid[0].len() as i64 + x;
    }
    if y < 0 {
        y = grid.len() as i64 + y;
    }
    let x = x as usize;
    let y = y as usize;
    if grid[y][x] == '#' {//|| seen.contains(&(x,y)){
        return false;
    }
    true
}
fn parse(data: String) -> (Grid, Point) {
    let mut grid: Grid = Vec::new();
    let mut start: Point = (0,0);
    for (y,line) in data.lines().enumerate() {
        let mut row: Row = Vec::new();
        for (x,c) in line.chars().enumerate() {
            if c == 'S' {
                start = (x,y);
            }
            row.push(c);
        }
        grid.push(row);
    }
    (grid, start)
}

fn print_seen(grid: &Grid, seen: &HashSet<Point>) -> () {
    for (y,row) in grid.iter().enumerate() {
        let mut st = "".to_string();
        for (x,c) in row.iter().enumerate() {
            if seen.contains(&(x,y)) {
                st.push('O');
            }
            else {
                st.push(c.clone());
            }
        }
        println!("{}", st);
    }
}

pub async fn advent_2(data: String) -> usize {
    let answer;
    println!("{}",-1 % 10);
    let mut seen: HashSet<Point> = HashSet::new();
    let (grid, start) = parse(data);
    answer = solve(&grid, &mut seen, start, 200);
    print_seen(&grid, &seen);
    return answer;
}