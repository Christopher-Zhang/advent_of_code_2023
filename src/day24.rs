use ndarray::prelude::*;
use ndarray_linalg::Solve;
use z3::{Config, Context, Solver}; 
use z3d::{dec, exp};
use anyhow::{Result, anyhow};

extern crate z3;

#[derive(PartialEq, Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
    z: f64
}
type Velocity = Point;
#[derive(PartialEq, Debug, Clone, Copy)]
struct Hailstone {
    position: Point,
    velocity: Velocity
}

fn parse1(data: String) -> Vec<Hailstone> {
    let mut hailstones: Vec<Hailstone> = Vec::new();

    for line in data.lines() {
        let sides: Vec<&str> = line.split(" @ ").collect();
        let coords: Vec<&str> = sides[0].split(", ").collect();
        let vel_coords: Vec<&str> = sides[1].split(", ").collect();
        // println!("{:?}", coords);
        // println!("{:?}", vel_coords);
        let position = Point {
            x: coords[0].trim().parse::<f64>().unwrap(),
            y: coords[1].trim().parse::<f64>().unwrap(),
            z: coords[2].trim().parse::<f64>().unwrap(),
        };

        let velocity = Velocity {
            x: vel_coords[0].trim().parse::<f64>().unwrap(),
            y: vel_coords[1].trim().parse::<f64>().unwrap(),
            z: vel_coords[2].trim().parse::<f64>().unwrap(),
        };

        hailstones.push( Hailstone { position, velocity } );
    }

    return hailstones;
}

fn solve1(bounds: (f64, f64), hailstones: &Vec<Hailstone>) -> usize {
    let mut count = 0;
    for i in 0..hailstones.len() {
        for j in i + 1..hailstones.len() {
            if let Ok(intersects) = intersect1(bounds, &hailstones[i], &hailstones[j]) {
                if intersects {
                    // println!("{:?} intersects {:?}", i, j);
                    count += 1;
                }
            }
            else {
                // println!("{:?} failed with {:?}", i, j);
            }
        }
    }
    return count;
}
fn intersect1(bounds: (f64, f64), h1: &Hailstone, h2: &Hailstone) -> Result<bool> {
    // x1t1 + b1 = x2t2 + b2
    // y1t1 + c1 = y2t2 + c2
    // x = mt + b
    // t = (x-b)/m
    // y = nt + c
    // y = n/m (x-b) + c
    // y = n/m * x + c - n/m * b
    let m1 = h1.velocity.y / h1.velocity.x;
    let b1 = h1.position.y - m1 * h1.position.x;
    let m2 = h2.velocity.y / h2.velocity.x;
    let b2 = h2.position.y - m2 * h2.position.x;
    if m1 == m2 && b1 != b2 {
        return Err(anyhow!("parallel lines"));
    }
    let x = (b1 - b2) / (m2 - m1);
    let y = m1 * x + b1;
    // println!("intersect at {x}, {y}");
    // println!("{:?}", bounds);
    let t1 = (x - h1.position.x) / h1.velocity.x;
    let t2 = (x - h2.position.x) / h2.velocity.x;
    Ok(t1 >= 0. && t2 >= 0. && x >= bounds.0 && x <= bounds.1 && y >= bounds.0 && y <= bounds.1)
    // let a: Array2<f64> = array![[h1.velocity.x, -h2.velocity.x], [h1.velocity.y, -h2.velocity.y]];
    // let b: Array1<f64> = array![h1.position.x - h2.position.x, h1.position.y - h2.position.y];
    // let x = a.solve(&b)?[0];
    
    // return Ok(x >= bounds.0 && x <= bounds.1);
    // Ok(true)
}
pub async fn advent(data: String) -> usize {
    let hailstones = parse1(data);
    return solve1((200_000_000_000_000., 400_000_000_000_000.), &hailstones);
    // return solve1((7., 27.), &hailstones);
}


fn solve2(hailstones: Vec<Hailstone>) -> usize {
    let mut a: Array2<f64> = Array2::zeros((hailstones.len() * 3,6));
    let mut b: Array1<f64> = Array1::zeros(7);
    // dbg!(a);
    // Xt + a = x
    // Yt + b = y
    // Zt + c = z
    // t = (z-c)/Z
    // X(z-c)/Z + a = x
    // Y(z-c)/Z + b = y
    // x y z a b c t1 t2 t3
    let ctx = &Context::new(&Config::default());   // we declare constants in a Context
    let solver = Solver::new(ctx); 
    
    // let x = dec!(x: real in ctx);
    // let y = dec!(y: real in ctx);
    // let z = dec!(z: real in ctx);
    // let a = dec!(a: real in ctx);
    // let b = dec!(b: real in ctx);
    // let c = dec!(c: real in ctx);
    // let t1 = dec!(t1: real in ctx);
    // let t2 = dec!(t2: real in ctx);
    // let t3 = dec!(t3: real in ctx);
    let x1 = ctx.from_f64(hailstones[0].velocity.x);
    let y1 = ctx.from_f64(hailstones[0].velocity.y);
    let z1 = ctx.from_f64(hailstones[0].velocity.z);
    let a1 = ctx.from_f64(hailstones[0].position.x);
    let b1 = ctx.from_f64(hailstones[0].position.y);
    let c1 = ctx.from_f64(hailstones[0].position.z);
    let x2 = ctx.from_f64(hailstones[1].velocity.x);
    let y2 = ctx.from_f64(hailstones[1].velocity.y);
    let z2 = ctx.from_f64(hailstones[1].velocity.z);
    let a2 = ctx.from_f64(hailstones[1].position.x);
    let b2 = ctx.from_f64(hailstones[1].position.y);
    let c2 = ctx.from_f64(hailstones[1].position.z);
    let x3 = ctx.from_f64(hailstones[2].velocity.x);
    let y3 = ctx.from_f64(hailstones[2].velocity.y);
    let z3 = ctx.from_f64(hailstones[2].velocity.z);
    let a3 = ctx.from_f64(hailstones[2].position.x);
    let b3 = ctx.from_f64(hailstones[2].position.y);
    let c3 = ctx.from_f64(hailstones[2].position.z);
    let x = ctx.named_real_const("x");
    let y = ctx.named_real_const("y");
    let z = ctx.named_real_const("z");
    let a = ctx.named_real_const("a");
    let b = ctx.named_real_const("b");
    let c = ctx.named_real_const("c");
    let t1 = ctx.named_real_const("t1");
    let t2 = ctx.named_real_const("t2");
    let t3 = ctx.named_real_const("t3");

    let e1 = x.mul(&t1).add();
    // let e2 = ;
    // let e3 = ;
    // let e4 = ;
    // let e5 = ;
    // let e6 = ;
    // let e7 = ;
    // let e8 = ;
    // let e9 = ;


    0
}


pub async fn advent_2(data: String) -> usize {
    let hailstones = parse1(data);
    return solve2(hailstones);
}