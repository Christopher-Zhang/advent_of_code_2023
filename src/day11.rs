use std::cmp::{max, min};

type Grid = Vec<Vec<char>>;
pub async fn advent(data: String) -> usize {
    let mut answer = 0;
    let mut grid: Grid = Vec::new();
    for line in data.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }
    let mut height = grid.len();
    let mut width = grid[0].len();

    let mut x = 0;
    while x < width {
        if grid.iter().all(|row| {
            row[x] != '#'
        }) {
            grid.iter_mut().for_each(|row| {
                row.insert(x, '.');
            });
            width += 1;
            x += 1;
        }
        x += 1;
    }
    let mut y = 0;
    while y < height {
        if grid[y].iter().all(|c| c != &'#') {
            let new_row = vec!['.'; width];
            grid.insert(y, new_row);
            height += 1;
            y += 1;
        }
        y += 1;
    }

    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for (y,row) in grid.iter().enumerate() {
        for (x,c) in row.iter().enumerate() {
            if c == &'#' {
                galaxies.push((x,y));
            }
        }
    }

    for (i, galaxy) in galaxies.iter().enumerate() {
        if i == galaxies.len()-1 {
            break;
        }
        for j in i+1..galaxies.len() {
            answer += get_distance(galaxy, &galaxies[j]);
        }
    }

    // print_grid(&grid);
    return answer;
}

fn get_distance(a: &(usize, usize), b: &(usize, usize)) -> usize {
    return max(a.0,b.0) - min(a.0,b.0) + max(a.1,b.1) - min(a.1,b.1);
}
pub async fn advent_2(data: String) -> usize {
    let mut answer = 0;
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    let mut width = 0;
    let mut height = 0;
    for (y,line) in data.lines().enumerate() {
        for (x,c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((x,y));
            }
            width = max(x+1, width);
        }
        height = max(y+1, height);
    }

    for y in (0..height).rev() {
        if galaxies.iter().all(|g| g.1 != y) {
            galaxies.iter_mut().for_each(|g| {
                if g.1 > y {
                    g.1 += 999_999;
                }
            });
        }
    }
    for x in (0..width).rev() {
        if galaxies.iter().all(|g| g.0 != x) {
            galaxies.iter_mut().for_each(|g| {
                if g.0 > x {
                    g.0 += 999_999;
                }
            });
        }
    }

    for (i, galaxy) in galaxies.iter().enumerate() {
        if i == galaxies.len()-1 {
            break;
        }
        for j in i+1..galaxies.len() {
            answer += get_distance(galaxy, &galaxies[j]);
        }
    }

    
    return answer;
}


fn print_grid(grid: &Grid) -> () {
    for row in grid {
        let mut st = "".to_string();
        for c in row {
            st.push(c.clone());
        }
        println!("{}", st);
    }
}