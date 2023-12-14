use std::str;

pub async fn advent(data: String) -> usize {
    let mut grid: Grid = Vec::new();
    for line in data.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }
    tilt_north(&mut grid);
    _print_grid(&grid);
    return count_load(&grid);
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    West,
    South,
    East,
}
impl Direction {
    const VALUES: [Self; 4] = [Self::North, Self::West, Self::South, Self::East];
}

type Grid = Vec<Vec<char>>;
pub async fn advent_2(data: String) -> usize {
    let target = 1_000_000_000 - 1;
    let mut grid: Grid = Vec::new();
    for line in data.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }
    _print_grid(&grid);
    let mut loads: Vec<usize> = Vec::new();
    let mut states: Vec<String> = Vec::new();
    for i in 0..1_000_000_000 {
        if (i+1) % 10000 == 0 {
            println!("Cycle {}", i+1);
            break;
        }
        for direction in Direction::VALUES {
            tilt(&mut grid, direction);
        }
        loads.push(count_load(&grid));
        states.push(grid.iter().map(|v| v.iter().collect::<String>()).collect::<String>());
    }
    let (cycle_start, cycle_end) = find_cycle(&states);
    println!("cycle_start {cycle_start} cycle_end {cycle_end}");
    // println!("loads[{target}] = {}", loads[target]);
    println!("getting index {}", target % (cycle_end - cycle_start));
    return loads[(target-cycle_start) % (cycle_end - cycle_start) + cycle_start];
}
fn find_cycle (states: &Vec<String>) -> (usize, usize) {
    for i in 0..states.len() {
        for j in i+1..states.len() {
            if states[i] == states[j] {
                return (i, j);
            }
        }
    }
    return (0,0);
}

fn _print_grid(grid: &Grid) -> () {
    println!();
    for row in grid {
        let mut st = "".to_string();
        for c in row {
            st.push(c.clone());
        }
        println!("{}", st);
    }
}
fn tilt (grid: &mut Grid, dir: Direction) {
    match dir {
        Direction::North => tilt_north(grid),
        Direction::West => tilt_west(grid),
        Direction::South => tilt_south(grid),
        Direction::East => tilt_east(grid),
    };
}

fn tilt_north (grid: &mut Grid) {
    for x in 0..grid[0].len() {
        let mut y = 0;
        let mut wall = 0;
        while y < grid.len() {
            let mut count = 0;
            while y < grid.len() && grid[y][x] != '#' {
                if grid[y][x] == 'O' {
                    grid[y][x] = '.';
                    count += 1;
                }
                y += 1;
            }
            for i in wall..wall+count {
                grid[i][x] = 'O';
            }
            y += 1;
            wall = y;
        }
    }
}
fn tilt_south (grid: &mut Grid) {
    for x in 0..grid[0].len() {
        let mut y = grid.len() - 1;
        let mut wall = grid.len();
        loop {
            let mut count = 0;
            while grid[y][x] != '#' {
                if grid[y][x] == 'O' {
                    grid[y][x] = '.';
                    count += 1;
                }
                if y == 0 {
                    break;
                }
                y -= 1;
            }
            for i in wall - count .. wall {
                grid[i][x] = 'O';
            }
            wall = y;
            if y == 0 {
                break;
            }
            y -= 1;
        }
    }
}
fn tilt_west (grid: &mut Grid) {
    for y in 0..grid.len() {
        let mut x = 0;
        let mut wall = 0;
        while x < grid[0].len() {
            let mut count = 0;
            while x < grid[0].len() && grid[y][x] != '#' {
                if grid[y][x] == 'O' {
                    grid[y][x] = '.';
                    count += 1;
                }
                x += 1;
            }
            for i in wall..wall+count {
                grid[y][i] = 'O';
            }
            x += 1;
            wall = x;
        }
    }
}
fn tilt_east (grid: &mut Grid) {
    for y in 0..grid.len() {
        let mut x = grid.len() - 1;
        let mut wall = grid.len();
        loop {
            let mut count = 0;
            while grid[y][x] != '#' {
                if grid[y][x] == 'O' {
                    grid[y][x] = '.';
                    count += 1;
                }
                if x == 0 {
                    break;
                }
                x -= 1;
            }
            for i in wall - count .. wall {
                grid[y][i] = 'O';
            }
            if x == 0 {
                break;
            }
            wall = x;
            x -= 1;
        }
    }
}

fn count_load(grid: &Grid) -> usize {
    let mut load = 0;
    for (y, row) in grid.iter().enumerate() {
        for c in row {
            if c == &'O' {
                load += grid.len() - y;
            }
        }
    }
    return load;
}