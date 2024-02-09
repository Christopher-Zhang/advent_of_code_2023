use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use regex::Regex;

pub async fn advent(data: String) -> usize {
    let (mut bricks, mut space, xyz) = parse(data);
    // _display(&space, &xyz);
    solve(&mut bricks, &mut space, &xyz)
}

fn solve(bricks: &mut Vec<Brick>, space: &mut Space, xyz: &Point) -> usize {
    let mut count = 0;
    let mut moved = true;
    while moved {
        moved = false;
        for brick_num in 0..bricks.len() {
            moved |= fall(brick_num, bricks, space);
        }
    }
    // _display(space, xyz);
    let mut supports = HashMap::<usize, HashSet<usize>>::new();
    let mut is_supported_by = HashMap::<usize, HashSet<usize>>::new();

    for (brick_num, brick) in bricks.iter().enumerate() {
        for x in range(brick.start.x, brick.end.x) {
            for y in range(brick.start.y, brick.end.y) {
                for z in range(brick.start.z, brick.end.z) {
                    if let Some(b) = space.get(&Point{x,y,z:z+1}) {
                        if b == &brick_num {
                            continue;
                        }
                        match supports.get_mut(&brick_num) {
                            Some(v) => {v.insert(b.clone());},
                            None => {
                                let mut set = HashSet::<usize>::new();
                                set.insert(b.clone());
                                supports.insert(brick_num, set);
                            }
                        };
                        match is_supported_by.get_mut(&b) {
                            Some(v) => {v.insert(brick_num);},
                            None => {
                                let mut set = HashSet::<usize>::new();
                                set.insert(brick_num);
                                is_supported_by.insert(b.clone(), set);
                            }
                        };
                    }
                }
            }
        }
    }
    // dbg!(&supports);
    // dbg!(&is_supported_by);
    for (brick_num, brick) in bricks.iter().enumerate() {
        if supports.get(&brick_num).unwrap_or(&HashSet::<usize>::new()).iter().all(|carry| {
            if let Some(v) = is_supported_by.get(&carry) {
                return v.len() > 1;
            }
            false
        }) {
            // println!("brick {brick_num} is safe to destroy");
            count += 1;
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
        bricks.push(brick);
    });
    bricks.sort_by(|a,b| {
        let za = std::cmp::max(a.start.z, a.end.z);
        let zb = std::cmp::max(b.start.z, b.end.z);
        za.cmp(&zb)
    });
    for brick_num in 0..bricks.len() {
        let brick = &bricks[brick_num];
        for x in range(brick.start.x, brick.end.x) {
            for y in range(brick.start.y, brick.end.y) {
                for z in range(brick.start.z, brick.end.z) {
                    space.insert(Point{x,y,z}, brick_num);
                }
            }
        }
    }

    // println!("{:?}", &bricks);
    println!("{} bricks", bricks.len());

    (bricks, space, Point{x,y,z})
    
}

fn fall(brick_num: usize, bricks: &mut Vec<Brick>, space: &mut Space) -> bool {
    let brick = &bricks[brick_num];
    for x in range(brick.start.x, brick.end.x) {
        for y in range(brick.start.y, brick.end.y) {
            for z in range(brick.start.z, brick.end.z) {
                if z == 0 {
                    return false;
                }
                if let Some(n) = space.get(&Point{x,y,z: z-1}) {
                    if *n != brick_num {
                        return false;
                    }
                }
            }
        }
    }

    for x in range(brick.start.x, brick.end.x) {
        for y in range(brick.start.y, brick.end.y) {
            for z in range(brick.start.z, brick.end.z) {
                // println!("trying {x},{y},{z}");
                if let Some(v) = space.remove(&Point{x,y,z}) {
                    // println!("removing {v} for {brick_num} at {x},{y},{z}");
                    // assert_eq!(v, brick_num);
                    space.insert(Point{x,y,z: z-1}, brick_num);
                }
            }
        }
    }
    bricks[brick_num].start.z -= 1;
    bricks[brick_num].end.z -= 1;

    true
}

fn range(a: isize, b: isize) -> Vec<isize> {
    // let a = a as usize;
    // let b = b as usize;
    if a > b {
        return (b..=a).collect_vec();
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
            print!("\t{tile}");
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
            print!("\t{tile}");
        }
        println!();
    }
}

#[derive(Clone)]
struct Brick2 {
    name: usize,
    start: Point,
    end: Point,
    above: HashSet<usize>,
    below: HashSet<usize>
}
fn parse2(data: String) -> (Vec<Brick2>, Space, Point) {
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut z: isize = 0;
    data.split("\n").for_each(|line| line.split('~').for_each(|point| {
        let xyz = point.split(',').collect_vec(); 
        x = std::cmp::max(x, xyz[0].parse().unwrap());
        y = std::cmp::max(y, xyz[1].parse().unwrap());
        z = std::cmp::max(z, xyz[2].parse().unwrap());
    }));
    let mut space: Space = HashMap::new();
    let mut bricks = Vec::<Brick2>::new();
    data.split("\n").enumerate().for_each(|(brick_num, line)| {
        let points = line.split('~').collect_vec();
        let start = Point::from_string(points[0]).unwrap();
        let end = Point::from_string(points[1]).unwrap();
        let brick = Brick2{
            name: brick_num,
            start,
            end,
            above: HashSet::<usize>::new(),
            below: HashSet::<usize>::new()
        };
        bricks.push(brick);
    });
    bricks.sort_by(|a,b| {
        let za = std::cmp::min(a.start.z, a.end.z);
        let zb = std::cmp::min(b.start.z, b.end.z);
        za.cmp(&zb)
    });

    for (brick_num, brick) in bricks.iter().enumerate() {
        for x in range(brick.start.x, brick.end.x) {
            for y in range(brick.start.y, brick.end.y) {
                for z in range(brick.start.z, brick.end.z) {
                    space.insert(Point{x,y,z}, brick_num);
                }
            }
        }
    }

    println!("{} bricks", bricks.len());

    (bricks, space, Point{x,y,z})
}
fn fall2(brick_num: usize, bricks: &mut Vec<Brick2>, space: &mut Space) -> bool {
    let brick = &bricks[brick_num];
    for x in range(brick.start.x, brick.end.x) {
        for y in range(brick.start.y, brick.end.y) {
            for z in range(brick.start.z, brick.end.z) {
                if z == 0 {
                    return false;
                }
                if let Some(n) = space.get(&Point{x,y,z: z-1}) {
                    if *n != brick_num {
                        return false;
                    }
                }
            }
        }
    }

    for x in range(brick.start.x, brick.end.x) {
        for y in range(brick.start.y, brick.end.y) {
            for z in range(brick.start.z, brick.end.z) {
                // println!("trying {x},{y},{z}");
                if let Some(v) = space.remove(&Point{x,y,z}) {
                    // println!("removing {v} for {brick_num} at {x},{y},{z}");
                    // assert_eq!(v, brick_num);
                    space.insert(Point{x,y,z: z-1}, brick_num);
                }
            }
        }
    }
    bricks[brick_num].start.z -= 1;
    bricks[brick_num].end.z -= 1;

    true
}
fn traverse(bricks: &mut Vec<Brick2>, current_brick: usize) -> usize {
    // println!("\tchecking {current_brick}");
    let mut q = VecDeque::<usize>::new();
    let mut fallen = HashSet::<usize>::new();
    for b in bricks[current_brick].above.clone().iter() {
        bricks[*b].below.remove(&current_brick);
        q.push_back(*b);
    }
    while !q.is_empty() {
        let current_brick = q.pop_front().unwrap();

        if fallen.contains(&current_brick) || !bricks[current_brick].below.is_empty() {
            continue;
        }

        fallen.insert(current_brick);
        for b in bricks[current_brick].above.clone().iter() {
            bricks[*b].below.remove(&current_brick);
            q.push_back(*b);
        }
    }

    fallen.len()
}
fn solve2(bricks: &mut Vec<Brick2>, space: &mut Space, xyz: &Point) -> usize {
    // fall
    let mut moved = true;
    while moved {
        moved = false;
        for brick_num in 0..bricks.len() {
            moved |= fall2(brick_num, bricks, space);
        }
    }

    // figure out the relationship in space
    for brick_num in 0..bricks.len() {
        for x in range(bricks[brick_num].start.x, bricks[brick_num].end.x) {
            for y in range(bricks[brick_num].start.y, bricks[brick_num].end.y) {
                for z in range(bricks[brick_num].start.z, bricks[brick_num].end.z) {
                    if let Some(b) = space.get(&Point{x,y,z:z+1}) {
                        if b == &brick_num {
                            continue;
                        }
                        bricks[brick_num].above.insert(*b);
                        bricks[*b].below.insert(brick_num);
                    }
                }
            }
        }
    }
    let mut count = 0;
    for brick_num in 0..bricks.len() {
        // println!("checking {brick_num}");
        count += traverse(&mut bricks.clone(), brick_num);
        // println!("Count is now {count}");
    }
    count
}
pub async fn advent_2(data: String) -> usize {
    let (mut bricks, mut space, xyz) = parse2(data);
    solve2(&mut bricks, &mut space, &xyz)
}
