use std::collections::{HashSet, VecDeque, HashMap};
use cached::proc_macro::cached;
use itertools::Itertools;
type Grid = Vec<Vec<char>>;
type Row = Vec<char>;
type Point = (usize, usize);
type IPoint = (i64, i64);
static DIRECTIONS: [(i64,i64); 4] = [(0,1),(0,-1),(1,0),(-1,0)];

pub async fn advent(data: String) -> usize {
    
    let (grid, start) = parse(data);
    solve(&grid, start, 64)
}

fn solve(grid: &Grid, start: Point, steps: usize) -> usize {
    let mut count = 0;
    let mut queue: VecDeque<Point> = VecDeque::new();
    let mut seen: HashSet<Point> = HashSet::new();
    queue.push_back(start);
    for current_step in 0..steps+1 {
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

            let neighbors = get_neighbors(grid, current, &seen);
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

fn get_neighbors(grid: &Grid, point: Point, seen: &HashSet<Point>) -> Vec<Point> {
    let x = point.0 as i64;
    let y = point.1 as i64;
    DIRECTIONS.iter().filter_map(|dir| {
        if is_valid(x+dir.0, y+dir.1, grid, seen) {
            return Some(((x+dir.0) as usize, (y+dir.1) as usize));
        }
        None
    }).collect_vec()
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

fn _print_seen(grid: &Grid, seen: &HashSet<Point>) -> () {
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
    let (mut grid, _, width, height) = parse2(data.clone());
    let mut odds = 0;
    let mut evens = 0;
    let mut memmap: MemMap = HashMap::new();
    for point in corners_and_edges(width, height).iter() {
        let mut mem: Mem = vec![0];
        (odds, evens) = fill_memmap(&mut grid, point.clone(), &mut mem);
        memmap.insert(*point, mem);
    }
    // println!("{odds} odds and {evens} evens");
    // println!("{width} width and {height} height");
    solve2(odds, evens, width, height, 26501365, &memmap)

    // println!("{}", count(&mut grid, (0,0), &mut mem));
    //// verify that verify() is correct against the example daya
    // let tests = [(6, 16), (10,50), (50, 1594), (100, 6536)];
    // let (grid, start, width, height) = parse2(data);
    // for (steps, test) in tests {
    //     let attempt = solve_check(&grid, start, steps);
    //     assert_eq!(test, attempt, "VERIFY SOLVE_CHECK: Reference {test} != attempt {attempt} with steps {steps}");
    // }

    //// verify that my solve 2 is correct
    // for n in (width)..(5*width) {
    //     // let mut seen = HashSet::<Point>::new();
    //     let a = solve_check(&grid, start, n);
    //     let b = solve2(odds, evens, width, height, n, &memmap);
    //     assert_eq!(a, b, "VERIFY SOLVE2: Reference {a} not equal to attempt {b} when num_steps = {n}");
    // }
}

#[cached(key="String", convert=r##"{ format!("{}", steps) }"##)]
fn _verify(grid: &Grid, start: Point, steps: usize) -> usize {
    let mut queue: VecDeque<(IPoint, usize)> = VecDeque::new();
    let mut odds = 0;
    let mut evens = 0;
    let mut seen: HashMap<IPoint, usize> = HashMap::new();
    queue.push_back(((start.0 as i64, start.1 as i64), 0));
    seen.insert((start.0 as i64, start.1 as i64), 0);
    while !queue.is_empty() {
        let (current, step) = queue.pop_front().unwrap();
        if step > steps {
            break;
        }
        if step % 2 == 0 {
            evens += 1;
        }
        else {
            odds += 1;
        }
        for neighbor in get_neighbors_infinite(grid, current, &seen) {
            queue.push_back((neighbor, step + 1));
            seen.insert(neighbor, (step + 1) % 2);
        }
    }
    match steps % 2 {
        0 => evens,
        _ => odds
    }
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

    for stride in 1..strides {
        let x = stride + 1;
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
    let sides = vec![(0,height/2), (width/2,0), (width/2, height-1), (width-1, height/2)];
    let corners = vec![(0,0), (0,height-1), (width-1,0), (width-1, height-1)];
    let mut total_corners = 0;
    let mut total_edges = 0;
    if leftover > 0 {
        for _ in 1..=strides {
            for corner in corners.iter() {
                total_corners += memmap.get(&corner).unwrap()[leftover-1];
            }
        }
    }
    for _ in 1..strides {
        for corner in corners.iter() {
            let map = memmap.get(&corner).unwrap();
            total_corners += map[(leftover + width - 1) % map.len()];
        }
    }
    for side in sides {
        total_edges += memmap.get(&side).unwrap()[leftover + width/2];
        if leftover > width/2 {
            total_edges += memmap.get(&side).unwrap()[leftover - width/2 - 1];
        }
    }
    leftovers += total_corners + total_edges;
    total + leftovers
}

fn parse2(data: String) -> (Grid, Point, usize, usize) {
    let mut start: Point = (0,0);
    let mut grid = data.lines().enumerate().map(|(y,line)| {
        if let Some(x) = line.chars().position(|c| c == 'S') {
            start = (x,y).clone();
        }
        line.chars().collect_vec()
    }).collect_vec();
    grid[start.1][start.0] = '.'; // clear 'S"
    let (width, height) = (grid[0].len(), grid.len());
    (grid, start, width, height)
}

fn fill_memmap(grid: &mut Grid, start: Point, mem: &mut Mem) -> (usize, usize) {
    let mut seen = HashSet::<Point>::new();
    let mut q = VecDeque::<(Point, usize)>::new();
    q.push_back((start, 0));
    seen.insert(start);
    let mut odds = 0;
    let mut evens = 0;
    while !q.is_empty() {
        let (current, step) = q.pop_front().unwrap();
        let push = match step % 2 {
            0 => {
                odds += 1;
                odds
            },
            _ => {
                evens += 1;
                evens
            }
        };
        if step >= mem.len() {
            mem.push(push);
        }
        else {
            mem[step] = push;
        }

        for neighbor in get_neighbors(grid, current, &seen) {
            if !seen.contains(&neighbor) {
                q.push_back((neighbor, step+1));
                seen.insert(neighbor);
            }
        }
    }
    mem.push(mem[mem.len()-1]);
    (odds, evens)
}

fn corners_and_edges(width: usize, height: usize) -> Vec<Point> {
    vec![(0,0), (0,height/2), (0,height-1), (width/2,0), (width-1,0), (width-1, height-1), (width/2, height-1), (width-1, height/2)]
}

fn get_neighbors_infinite(grid: &Grid, point: IPoint, seen: &HashMap<IPoint, usize>) -> Vec<IPoint> {
    let mut neighbors: Vec<IPoint> = Vec::new();
    for dir in DIRECTIONS {
        if is_valid_infinite(point.0+dir.0, point.1+dir.1, grid, seen) {
            neighbors.push(((point.0+dir.0), (point.1+dir.1)));
        }
    }
    neighbors
}

fn is_valid_infinite(xi: i64, yi: i64, grid: &Grid, seen: &HashMap<IPoint, usize>) -> bool {
    let mut x = xi % grid[0].len() as i64;
    let mut y = yi % grid.len() as i64;
    if x < 0 {
        x += grid[0].len() as i64;
    }
    if y < 0 {
        y += grid.len() as i64;
    }
    let xu = x as usize;
    let yu = y as usize;
    if grid[yu][xu] == '#' || seen.contains_key(&(xi,yi)){
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