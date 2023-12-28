use std::collections::{HashMap};
use std::i64;
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Hole {
    point: Point,
    // dug: bool,
    color: String
}
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Point {
    x: i64,
    y: i64,
}

type Grid = HashMap<Point, Hole>;
pub async fn advent(data: String) -> usize {
    let mut answer = 0;
    let mut grid: Grid = HashMap::new();

    let mut x: i64 = 0;
    let mut y: i64 = 0;

    let mut min_x = x.clone();
    let mut max_x = x.clone();
    let mut min_y = x.clone();
    let mut max_y = x.clone();

    for line in data.lines() {
        let words = line.split(' ');
        let words: Vec<&str> = words.collect();
        // let dist = words[1].parse::<i64>().unwrap();
        let dist = 1;
        match words[0] {
            "R" => {
                for _ in 0..dist {
                    x += 1;
                    let point = Point{x,y};
                    grid.insert(point.clone(), Hole {point, color: words[2].to_string()});
                }
                max_x = std::cmp::max(max_x, x);
            },
            "L" => {
                for _ in 0..dist {
                    x -= 1;
                    let point = Point{x,y};
                    grid.insert(point.clone(), Hole {point, color: words[2].to_string()});
                }
                min_x = std::cmp::min(min_x, x);
            },
            "U" => {
                for _ in 0..dist {
                    y -= 1;
                    let point = Point{x,y};
                    grid.insert(point.clone(), Hole {point, color: words[2].to_string()});
                }
                min_y = std::cmp::min(min_y, y);
            },
            "D" => {
                for _ in 0..dist {
                    y += 1;
                    let point = Point{x,y};
                    grid.insert(point.clone(), Hole {point, color: words[2].to_string()});
                }
                max_y = std::cmp::max(max_y, y);
            },
            _ => {
                panic!("unexpected input");
            }
        };
    }
    println!("x {min_x} to {max_x}\ny {min_y} to {max_y}");
    // _print_grid(&grid, min_x, max_x, min_y, max_y);
    flood_fill(Point{x: 20, y: 20}, &mut grid);
    // 663
    _print_grid(&grid, min_x, max_x, min_y, max_y);
    for y in min_y..max_y+1 {
        for x in min_x..max_x+1 {
            if let Some(_) = grid.get(&Point{x,y}) {
                answer += 1;
            }
        }
    }
    return answer;
}

fn flood_fill(pt: Point, grid: &mut Grid) {
    if let Some(_) = grid.get(&pt) {
        return;
    }

    grid.insert(pt.clone(), Hole{point: pt, color: "".to_string()});
    let x = pt.x;
    let y = pt.y;
    flood_fill(Point {x: x-1, y}, grid);
    flood_fill(Point {x: x+1, y}, grid);
    flood_fill(Point {x, y: y+1}, grid);
    flood_fill(Point {x, y: y-1}, grid);
}
fn _print_grid(grid: &Grid, min_x: i64, max_x: i64, min_y: i64, max_y: i64) -> () {
    println!();
    for y in min_y..max_y+1 {
        let mut st = "".to_string();
        for x in min_x..max_x+1 {
            match grid.get(&Point { x,y }) {
                Some(_) => st.push('#'),
                None => st.push('.')
            };
        }
        println!("{}", st);
    }
}
pub async fn advent_2(data: String) -> usize {
    let mut answer = 0;
    let mut grid: Grid = HashMap::new();

    let mut x: i64 = 0;
    let mut y: i64 = 0;

    let mut min_x = x.clone();
    let mut max_x = x.clone();
    let mut min_y = x.clone();
    let mut max_y = x.clone();
    let mut vertices: Vec<Point> = Vec::new();
    // vertices.push(Point{x,y});
    let mut add = 0;

    for line in data.lines() {
        let words = line.split(' ');
        let words: Vec<&str> = words.collect();
        // let dist = words[1].parse::<i64>().unwrap();
        let dist = i64::from_str_radix(&words[2][2..words[2].len()-2], 16).unwrap();
        let dir: i64 = words[2][words[2].len()-2..words[2].len()-1].parse().unwrap();
        // println!("{}, dist {}", line, dist);
        match dir {
            0 => {
                // for _ in 0..dist {
                //     x += 1;
                //     let point = Point{x,y};
                //     grid.insert(point.clone(), Hole {point, color: words[2].to_string()});
                // }
                x += dist;
                add += dist;
                vertices.push(Point{x,y});
                max_x = std::cmp::max(max_x, x);
            },
            2 => {
                // for _ in 0..dist {
                //     x -= 1;
                //     let point = Point{x,y};
                //     grid.insert(point.clone(), Hole {point, color: words[2].to_string()});
                // }
                x -= dist;
                vertices.push(Point{x,y});
                min_x = std::cmp::min(min_x, x);
            },
            3 => {
                // for _ in 0..dist {
                //     y -= 1;
                //     let point = Point{x,y};
                //     grid.insert(point.clone(), Hole {point, color: words[2].to_string()});
                // }
                y -= dist;
                vertices.push(Point{x,y});
                min_y = std::cmp::min(min_y, y);
            },
            1 => {
                // for _ in 0..dist {
                //     y += 1;
                //     let point = Point{x,y};
                //     grid.insert(point.clone(), Hole {point, color: words[2].to_string()});
                // }
                y += dist;
                add += dist;
                vertices.push(Point{x,y});
                max_y = std::cmp::max(max_y, y);
            },
            _ => {
                panic!("unexpected input");
            }
        };
    }
    let mut left = 0;
    let mut right = 0;
    for i in 0..vertices.len() {
        let a = i;
        let b = (i + 1) % vertices.len();
        let p1 = vertices[a];
        let p2 = vertices[b];

        left += p1.x * p2.y;
        right += p1.y * p2.x;
    }

    let left = left as i64;
    let right = right as i64;
    let area = i64::abs(left - right);
    println!("area: {area}, add {add}");
    println!("x {min_x} to {max_x}\ny {min_y} to {max_y}");
    println!("Number of vertices: {}", vertices.len());
    // println!("First vertex {:?}, last {:?}", vertices[0], vertices[vertices.len()-1]);
    // _print_grid(&grid, min_x, max_x, min_y, max_y);
    // flood_fill(Point{x: 100, y: 100}, &mut grid);
    // _print_grid(&grid, min_x, max_x, min_y, max_y);
    // for y in min_y..max_y+1 {
    //     for x in min_x..max_x+1 {
    //         if let Some(_) = grid.get(&Point{x,y}) {
    //             answer += 1;
    //         }
    //     }
    // }
    return (area / 2 + add + 1) as usize;
    // return answer;
}

// 82712675214003 too low
// 82712746433310
// 0,0 0,2
// 2,0 2,2