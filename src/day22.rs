use itertools::Itertools;
use std::collections::HashMap;
use regex::Regex;

pub async fn advent(data: String) -> usize {
    let (mut bricks, mut space, xyz) = parse(data);
    // _display(&space, &xyz);
    solve(&mut bricks, &mut space, &xyz);
    let a = 10;
    let b = 2;
    println!("range between {} and {} is {:?}", a,b,range(a,b));
    let a = 1;
    let b = 2;
    println!("range between {} and {} is {:?}", a,b,range(a,b));
    let a = 10;
    let b = 10;
    println!("range between {} and {} is {:?}", a,b,range(a,b));
    // _display(&space, &xyz);
    0
}

fn solve(bricks: &mut Vec<Brick>, space: &mut Space, xyz: &Point) -> usize {
    let mut count = 0;
    let mut done = vec![true; bricks.len()];
    while done.iter().any(|v| v == &true) {
        for brick_num in 0..bricks.len() {
            done[brick_num] = fall(brick_num, bricks, space);
            // did_move |= fall(brick_num, bricks, space);
        }
        for brick_num in (0..bricks.len()).rev() {
            done[brick_num] = fall(brick_num, bricks, space);
            // did_move |= fall(brick_num, bricks, space);
        }
    }
    _display(&space, &xyz);
    fall(6, bricks, space);    

    let mut supports = HashMap::<usize, Vec<usize>>::new();
    let mut is_supported_by = HashMap::<usize, Vec<usize>>::new();

    for (brick_num, brick) in bricks.iter().enumerate() {
        for x in range(brick.start.x, brick.end.x) {
            for y in range(brick.start.y, brick.end.y) {
                for z in range(brick.start.z, brick.end.z) {
                    if let Some(b) = space.get(&Point{x,y,z:z+1}) {
                        match supports.get_mut(&brick_num) {
                            Some(v) => v.push(b.clone()),
                            None => {supports.insert(brick_num, vec![b.clone()]);}
                        };
                        match is_supported_by.get_mut(&b) {
                            Some(v) => v.push(brick_num),
                            None => {is_supported_by.insert(b.clone(), vec![brick_num]);}
                        };
                    }
                }
            }
        }
    }
    for (brick_num, brick) in bricks.iter().enumerate() {
        for carry in supports.get(&brick_num).unwrap_or(&vec![]) {
            if let Some(v) = is_supported_by.get(&carry) {
                if v.len() > 1 {
                    count += 1;
                }
            }
        }
    }

    count
}

type Space = HashMap<Point, usize>;
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
struct Point {
    x: isize,
    y: isize,
    z: isize
}
lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d+),(\d+),(\d+)").unwrap();
}
impl Point {
    fn from_string(st: &str) -> Option<Point> {
        // let re = Regex::new(r"(\d+),(\d+),(\d+)").unwrap();
        // return Some(Point{x:0,y:0,z:0});
        match RE.captures(st) {
            Some(caps) => Some(Point{x: caps[1].parse().unwrap(), y: caps[2].parse().unwrap(), z: caps[3].parse().unwrap()}),
            None => None
        }
    }
}
#[derive(Debug)]
struct Brick {
    start: Point,
    end: Point
}

fn parse(data: String) -> (Vec<Brick>, Space, Point) {
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut z: isize = 0;
    data.split("\n").for_each(|line| line.split('~').for_each(|point| {
        let xyz = point.split(',').collect_vec(); 
        x = std::cmp::max(x, xyz[0].parse().unwrap());
        y = std::cmp::max(y, xyz[1].parse().unwrap());
        z = std::cmp::max(z, xyz[2].parse().unwrap());
    }));
    // for _ in 0..x {
    //     let mut grid = Vec::<Vec<usize>>::new();
    //     for _ in 0..y {
    //         let mut dim = Vec::<usize>::new();
    //         for _ in 0..z {
    //             dim.push(usize::MAX);
    //         }
    //         grid.push(dim);
    //     }

    // }
    let mut space: Space = HashMap::new();
    let mut bricks = Vec::<Brick>::new();
    data.split("\n").enumerate().for_each(|(brick_num, line)| {
        let points = line.split('~').collect_vec();
        let start = Point::from_string(points[0]).unwrap();
        let end = Point::from_string(points[1]).unwrap();
        let brick = Brick{
            start,
            end,
        };
        for x in range(brick.start.x, brick.end.x) {
            for y in range(brick.start.y, brick.end.y) {
                for z in range(brick.start.z, brick.end.z) {
                    space.insert(Point{x,y,z}, brick_num);
                }
            }
        }
        bricks.push(brick);
    });
    bricks.sort_by(|a,b| {
        let za = std::cmp::min(a.start.z, a.end.z);
        let zb = std::cmp::min(b.start.z, b.end.z);
        return za.cmp(&zb);
    });

    // println!("{:?}", &bricks);
    println!("{} bricks", bricks.len());

    (bricks, space, Point{x,y,z})
    
}

fn fall(brick_num: usize, bricks: &mut Vec<Brick>, space: &mut Space) -> bool {
    let mut did_move = false;
    let brick = &bricks[brick_num];
    let mut can_move = true;
    'outer: for x in range(brick.start.x, brick.end.x) {
        for y in range(brick.start.y, brick.end.y) {
            for z in range(brick.start.z, brick.end.z) {
                if z == 0 {
                    return false;
                }
                if let Some(n) = space.get(&Point{x,y,z: z-1}) {
                    can_move = false;
                    break 'outer;
                }
            }
        }
    }
    if can_move {
        did_move = true;
        for x in range(brick.start.x, brick.end.x) {
            for y in range(brick.start.y, brick.end.y) {
                for z in range(brick.start.z, brick.end.z) {
                    // println!("trying {x},{y},{z}");
                    if let Some(v) = space.remove(&Point{x,y,z}) {
                        println!("removed {:?}", v);
                        space.insert(Point{x,y,z: z-1}, brick_num);
                    }
                }
            }
        }
        bricks[brick_num].start.z -= 1;
        bricks[brick_num].end.z -= 1;
    }

    did_move
}

fn range(a: isize, b: isize) -> Vec<isize> {
    // let a = a as usize;
    // let b = b as usize;
    if a > b {
        return (a..=b).rev().collect_vec();
    }
    (a..=b).collect_vec()
}

fn _display(space: &Space, xyz: &Point) {
    println!("x axis:");
    for z in (0..=xyz.z).rev() {
        print!("{z}: ");
        for x in 0..=xyz.x {
            let mut tile = ".".to_string();
            for y in 0..=xyz.y {
                if let Some(v) = space.get(&Point{x,y,z}) {
                    tile = v.to_string();
                    break;
                }
            }
            print!("{tile}");
        }
        println!();
    }

    println!("y axis:");
    for z in (0..=xyz.z).rev() {
        print!("{z}: ");
        for y in 0..=xyz.y {
            let mut tile = ".".to_string();
            for x in 0..=xyz.x {
                if let Some(v) = space.get(&Point{x,y,z}) {
                    tile = v.to_string();
                    break;
                }
            }
            print!("{tile}");
        }
        println!();
    }
}
pub async fn advent_2(data: String) -> usize {
    let mut answer = 0;

    return answer;
}