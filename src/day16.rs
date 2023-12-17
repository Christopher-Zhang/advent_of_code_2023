
#[derive(PartialEq, Debug, Clone, Copy)]
enum Direction {
    N,
    S,
    E,
    W
}
type CharGrid = Vec<Vec<char>>;
type DirGrid = Vec<Vec<Vec<Direction>>>;
type DirRow = Vec<Vec<Direction>>;
pub async fn advent(data: String) -> usize {
    let mut answer = 0;
    let mut grid: CharGrid = Vec::new();
    let mut dir_grid: DirGrid = Vec::new();

    for line in data.lines() {
        let mut row: Vec<char> = Vec::new();
        let mut dir_row: DirRow = Vec::new();
        for c in line.chars() {
            row.push(c);
            dir_row.push(Vec::<Direction>::new());
        }
        grid.push(row);
        dir_grid.push(dir_row);
    }
    _print_grid(&grid);
    light(0,0,Direction::E, &grid, &mut dir_grid);
    _print_dir_grid(&dir_grid);
    for row in dir_grid {
        for cell in row {
            if !cell.is_empty() {
                answer += 1;
            }
        }
    }
    return answer;
}

fn light(x: i32, y: i32, dir: Direction, grid: &CharGrid, dir_grid: &mut DirGrid) {
    if x < 0 || y < 0 || y as usize >= grid.len() || x as usize >= grid[0].len() {
        return;
    }
    let i = x as usize;
    let j = y as usize;
    let mut x = x;
    let mut y = y;
    if dir_grid[j][i].iter().any(|d| d == &dir) {
        return;
    }
    // _print_dir_grid(&dir_grid);
    dir_grid[j][i].push(dir.clone());
    
    let c = grid[j][i];
    let mut next_dir = dir.clone();
    if c == '|' && (dir == Direction::E || dir == Direction::W) {
        light(x, y+1, Direction::S, grid, dir_grid);
        light(x, y-1, Direction::N, grid, dir_grid);
    }
    else if c == '-' && (dir == Direction::N || dir == Direction::S) {
        light(x+1, y, Direction::E, grid, dir_grid);
        light(x-1, y, Direction::W, grid, dir_grid);
    }
    else {
        if c == '/' {
            if dir == Direction::E {
                next_dir = Direction::N;
            }
            if dir == Direction::W {
                next_dir = Direction::S;
            }
            if dir == Direction::S {
                next_dir = Direction::W;
            }
            if dir == Direction::N {
                next_dir = Direction::E;
            }
        }
        else if c == '\\' {
            if dir == Direction::E {
                next_dir = Direction::S;
            }
            if dir == Direction::W {
                next_dir = Direction::N;
            }
            if dir == Direction::S {
                next_dir = Direction::E;
            }
            if dir == Direction::N {
                next_dir = Direction::W;
            }
        }
        match next_dir {
            Direction::E => x += 1,
            Direction::N => y -= 1,
            Direction::S => y += 1,
            Direction::W => x -= 1
        };
        light(x,y,next_dir,grid,dir_grid);
    }
}
pub async fn advent_2(data: String) -> usize {
    let mut answer = 0;
    let mut grid: CharGrid = Vec::new();
    let mut dir_grid: DirGrid = Vec::new();

    for line in data.lines() {
        let mut row: Vec<char> = Vec::new();
        let mut dir_row: DirRow = Vec::new();
        for c in line.chars() {
            row.push(c);
            dir_row.push(Vec::<Direction>::new());
        }
        grid.push(row);
        dir_grid.push(dir_row);
    }

    for y in 0..grid.len() {
        let mut dir_grid = build_dir_grid(&grid);
        light(0, y as i32, Direction::E, &grid, &mut dir_grid);
        answer = std::cmp::max(answer, check_grid(&dir_grid));
        let mut dir_grid = build_dir_grid(&grid);
        light((grid[0].len()-1) as i32, y as i32, Direction::W, &grid, &mut dir_grid);
        answer = std::cmp::max(answer, check_grid(&dir_grid));
    }

    for x in 0..grid[0].len() {
        let mut dir_grid = build_dir_grid(&grid);
        light(x as i32, 0, Direction::S, &grid, &mut dir_grid);
        answer = std::cmp::max(answer, check_grid(&dir_grid));
        let mut dir_grid = build_dir_grid(&grid);
        light(x as i32, (grid.len()-1) as i32, Direction::N, &grid, &mut dir_grid);
        answer = std::cmp::max(answer, check_grid(&dir_grid));
    }
    return answer;
}

fn _print_grid(grid: &CharGrid) -> () {
    println!();
    for row in grid {
        let mut st = "".to_string();
        for c in row {
            st.push(c.clone());
        }
        println!("{}", st);
    }
}
fn _print_dir_grid(grid: &DirGrid) -> () {
    println!();
    for row in grid {
        let mut st = "".to_string();
        for dirs in row {
            if dirs.is_empty() {
                st.push('.');
                // st.push(' ');
            }
            else {
                st.push('#');
            }
            // st.push(char::from_digit(dirs.len() as u32, 10).unwrap());
            // st.push(' ');
        }
        println!("{}", st);
    }
}

fn build_dir_grid(grid: &CharGrid) -> DirGrid {
    let mut ret: DirGrid = Vec::new();
    for row in grid {
        let mut dir_row: DirRow = Vec::new();
        for _ in row {
            dir_row.push(Vec::<Direction>::new());
        }
        ret.push(dir_row);
    }
    ret
}

fn check_grid(dir_grid: &DirGrid) -> usize {
    let mut current = 0;
    for row in dir_grid {
        for cell in row {
            if !cell.is_empty() {
                current += 1;
            }
        }
    }
    current
}