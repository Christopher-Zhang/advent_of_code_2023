use itertools::Itertools;
use std::collections::HashMap;
use regex::Regex;

pub async fn advent(data: String) -> usize {
    let (mut bricks, mut space) = parse(data);
    solve(&mut bricks, &mut space)
}

fn solve(bricks: &mut Vec<Brick>, space: &mut Space) -> usize {
    let mut count = 0;
    let mut did_move = true;
    while !did_move {
        for brick_num in 0..bricks.len() {
            if !fall(brick_num, bricks, space) {
                did_move = false;
            }
            // did_move |= fall(brick_num, bricks, space);
        }
    }

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

fn parse(data: String) -> (Vec<Brick>, Space) {
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
                    space.insert(Point{x,y,z}, bricks.len());
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

    println!("{:?}", &bricks);

    (bricks, space)
    
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
                    space.remove(&Point{x,y,z});
                    space.insert(Point{x,y,z: z-1}, brick_num);
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
pub async fn advent_2(data: String) -> usize {
    let mut answer = 0;

    return answer;
}