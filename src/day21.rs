use std::{collections::{HashSet, VecDeque, HashMap}, mem};
use cached::proc_macro::cached;
use itertools::Itertools;
type Grid = Vec<Vec<char>>;
type Row = Vec<char>;
type Point = (usize, usize);
pub async fn advent(data: String) -> usize {
    let answer;
    let mut seen: HashSet<Point> = HashSet::new();
    let (grid, start) = parse(data);
    answer = solve(&grid, &mut seen, start, 64);
    // print_seen(&grid, &seen);
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

// #[cached(key="String", convert=r##"{ format!("{},{}", point.0, point.1) }"##)]
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
    if y >= grid.len() || x >= grid[0].len() || grid[y][x] == '#' || seen.contains(&(x,y)){
        return false;
    }
    true
}
// fn is_valid2(x: i64, y: i64, grid: &Grid) -> bool {
//     let mut x = x % grid[0].len() as i64;
//     let mut y = y % grid.len() as i64;
//     if x < 0 {
//         x = grid[0].len() as i64 + x;
//     }
//     if y < 0 {
//         y = grid.len() as i64 + y;
//     }
//     let x = x as usize;
//     let y = y as usize;
//     if grid[y][x] == '#' {//|| seen.contains(&(x,y)){
//         return false;
//     }
//     true
// }
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

type MemMap = HashMap<Point, Vec<usize>>;
type Mem = Vec<usize>;
pub async fn advent_2(data: String) -> usize {
    let (mut grid, start, width, height) = parse2(data);
    
    let (odds, evens) = odds_and_evens(&mut grid, start);
    let poi = points_of_interest(width, height);
    // let mem = Vec::<Vec<usize>>::new();
    let mut memmap: MemMap = HashMap::new();
    // let mut mem: Mem = vec![0];
    for point in poi.iter() {
            let mut mem: Mem = vec![1];
        count(&mut grid, start, &mut mem);
        memmap.insert(*point, mem);
    }
    // _print(&grid, &poi);
    // dbg!(memmap);
    println!("{odds} odds and {evens} evens");
    println!("{width} width and {height} height");
    // println!("{}", count(&mut grid, (0,0), &mut mem));
    for n in (width + 1)..(5*width) {
        let mut seen = HashSet::<Point>::new();
        let a = solve_check(&grid, &mut seen, start, n);
        let b = solve2(odds, evens, width, height, n, &memmap);
        assert_eq!(a, b, "Reference {a} not equal to attempt {b} when num_steps = {n}");
    }
    // solve2(odds, evens, width, height, 26501365, &memmap)
    0
}
fn solve_check(grid: &Grid, seen: &mut HashSet<Point>, start: Point, steps: usize) -> usize {
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

            let neighbors = get_valid_neighbors2(grid, current, seen);
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
fn solve2(odds: usize, evens: usize, width: usize, height: usize, total_steps: usize, memmap: &MemMap) -> usize {
    let mut total = 0;
    let strides = total_steps / width; 
    let leftover = total_steps % width;
    let mut even = total_steps % 2 == 0;
    if even {
        total += evens;
    }
    else {
        total += odds;
    }

    for x in 1..=strides {
        even = !even;
        if even {
            total += (4 * x - 4) * evens;
        }
        else {
            total += (4 * x - 4) * odds;
        }
    }

    // deal with leftovers
    let mut leftovers = 0;
    // [(0,0), (0,height/2), (0,height-1), (width/2,0), (width-1,0), (width-1, height-1), (width/2, height-1), (width-1, height/2)]
    let sides = vec![(0,height/2), (width/2,0), (width/2, height-1), (width-1, height/2)];
    let corners = vec![(0,0), (0,height-1), (width-1,0), (width-1, height-1)];
    if leftover > 0 {
        for side in sides {
            leftovers += memmap.get(&side).unwrap()[width + leftover - 3];
        }
        for corner in corners {
            leftovers += memmap.get(&corner).unwrap()[width / 2 + leftover];
        }
    }

    total + leftovers
}

fn parse2(data: String) -> (Grid, Point, usize, usize) {
    let mut start: Point = (0,0);
    let grid = data.lines().enumerate().map(|(y,line)| {
        if let Some(x) = line.chars().position(|c| c == 'S') {
            start = (x,y).clone();
        }
        line.chars().collect_vec()
    }).collect_vec();
    let (width, height) = (grid[0].len(), grid.len());
    (grid, start, width, height)
}
fn odds_and_evens(grid: &mut Grid, start: Point) -> (usize, usize) {
    let mut odds = 0;
    let mut evens = 0;
    let mut q = VecDeque::<(Point, usize)>::new();
    q.push_back((start, 0));
    while !q.is_empty() {
        let (current, step) = q.pop_front().unwrap();
        let x = current.0;
        let y = current.1;
        if !is_valid2(x as i64, y as i64, grid) {
            continue;
        }
        let marker: char;
        if step % 2 == 0 {
            marker = 'E';
            evens += 1;
        }
        else {
            marker = 'O';
            odds += 1;
        }

        grid[y][x] = marker;

        for neighbor in get_neighbors(grid, current) {
            q.push_back((neighbor, step+1));
        }
    }
    (odds, evens)
}

fn count (grid: &mut Grid, start: Point, mem: &mut Mem) -> usize {
    let mut seen = HashSet::<Point>::new();
    let mut q = VecDeque::<(Point, usize)>::new();
    q.push_back((start, 0));
    seen.insert(start);

    let mut max = 0;
    while !q.is_empty() {
        let (current, step) = q.pop_front().unwrap();
        if step >= mem.len() {
            mem.push(mem[mem.len()-1] + 1);
        }
        else {
            mem[step] += 1;
        }
        max = std::cmp::max(max, step);
        for neighbor in get_valid_neighbors(grid, current, &seen) {
            seen.insert(neighbor);
            q.push_back((neighbor, step+1));
        }
    }
    max
}

fn points_of_interest(width: usize, height: usize) -> Vec<Point> {
    vec![(0,0), (0,height/2), (0,height-1), (width/2,0), (width-1,0), (width-1, height-1), (width/2, height-1), (width-1, height/2)]
}

// #[cached(key="String", convert=r##"{ format!("{},{}", point.0, point.1) }"##)]
fn get_neighbors(grid: &Grid, point: Point) -> Vec<Point> {
    let mut neighbors: Vec<Point> = Vec::new();
    let directions: Vec<(i64, i64)> = vec![(0,1),(0,-1),(1,0),(-1,0)];

    let x = point.0 as i64;
    let y = point.1 as i64;

    for dir in directions {
        if is_valid2(x+dir.0, y+dir.1, grid) {
            neighbors.push(((x+dir.0) as usize, (y+dir.1) as usize));
        }
    }

    neighbors
}
fn get_valid_neighbors2(grid: &Grid, point: Point, seen: &HashSet<Point>) -> Vec<Point> {
    let mut neighbors: Vec<Point> = Vec::new();
    let directions: Vec<(i64, i64)> = vec![(0,1),(0,-1),(1,0),(-1,0)];

    let x = point.0 as i64;
    let y = point.1 as i64;

    for dir in directions {
        if is_valid3(x+dir.0, y+dir.1, grid, seen) {
            neighbors.push(((x+dir.0) as usize, (y+dir.1) as usize));
        }
    }

    neighbors
}

fn is_valid3(x: i64, y: i64, grid: &Grid, seen: &HashSet<Point>) -> bool {
    let mut x = x % grid[0].len() as i64;
    let mut y = y % grid.len() as i64;
    if x < 0 {
        x += grid[0].len() as i64;
    }
    if y < 0 {
        y += grid.len() as i64;
    }
    let x = x as usize;
    let y = y as usize;
    if grid[y][x] == '#' || seen.contains(&(x,y)){
        return false;
    }
    true
}
fn is_valid2(x: i64, y: i64, grid: &Grid) -> bool {
    if x < 0 || y < 0 {
        return false;
    }
    let x = x as usize;
    let y = y as usize;
    let invalid = ['#', 'E', 'O'];
    if y >= grid.len() || x >= grid[0].len() || invalid.contains(&grid[y][x]) {
        return false;
    }
    true
}

fn _print(grid: &Grid, points: &Vec<Point>) -> () {
    for (y,row) in grid.iter().enumerate() {
        let mut st = "".to_string();
        for (x,c) in row.iter().enumerate() {
            if points.contains(&(x,y)) {
                st.push('*');
            }
            else {
                st.push(c.clone());
            }
        }
        println!("{}", st);
    }
}