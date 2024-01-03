use z3::{*, ast::Ast};
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
                    count += 1;
                }
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
    let t1 = (x - h1.position.x) / h1.velocity.x;
    let t2 = (x - h2.position.x) / h2.velocity.x;
    Ok(t1 >= 0. && t2 >= 0. && x >= bounds.0 && x <= bounds.1 && y >= bounds.0 && y <= bounds.1)
}
pub async fn advent(data: String) -> usize {
    let hailstones = parse1(data);
    return solve1((200_000_000_000_000., 400_000_000_000_000.), &hailstones);
    // return solve1((7., 27.), &hailstones); // example data
}


fn solve2(hailstones: Vec<Hailstone>) -> usize {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let x1 = ast::Int::from_i64(&ctx, hailstones[0].velocity.x as i64);
    let y1 = ast::Int::from_i64(&ctx, hailstones[0].velocity.y as i64);
    let z1 = ast::Int::from_i64(&ctx, hailstones[0].velocity.z as i64);
    let a1 = ast::Int::from_i64(&ctx, hailstones[0].position.x as i64);
    let b1 = ast::Int::from_i64(&ctx, hailstones[0].position.y as i64);
    let c1 = ast::Int::from_i64(&ctx, hailstones[0].position.z as i64);
    let x2 = ast::Int::from_i64(&ctx, hailstones[1].velocity.x as i64);
    let y2 = ast::Int::from_i64(&ctx, hailstones[1].velocity.y as i64);
    let z2 = ast::Int::from_i64(&ctx, hailstones[1].velocity.z as i64);
    let a2 = ast::Int::from_i64(&ctx, hailstones[1].position.x as i64);
    let b2 = ast::Int::from_i64(&ctx, hailstones[1].position.y as i64);
    let c2 = ast::Int::from_i64(&ctx, hailstones[1].position.z as i64);
    let x3 = ast::Int::from_i64(&ctx, hailstones[2].velocity.x as i64);
    let y3 = ast::Int::from_i64(&ctx, hailstones[2].velocity.y as i64);
    let z3 = ast::Int::from_i64(&ctx, hailstones[2].velocity.z as i64);
    let a3 = ast::Int::from_i64(&ctx, hailstones[2].position.x as i64);
    let b3 = ast::Int::from_i64(&ctx, hailstones[2].position.y as i64);
    let c3 = ast::Int::from_i64(&ctx, hailstones[2].position.z as i64);

    let x = ast::Int::new_const(&ctx, "x");
    let y = ast::Int::new_const(&ctx, "y");
    let z = ast::Int::new_const(&ctx, "z");
    let a = ast::Int::new_const(&ctx, "a");
    let b = ast::Int::new_const(&ctx, "b");
    let c = ast::Int::new_const(&ctx, "c");
    let t1 = ast::Int::new_const(&ctx, "t1");
    let t2 = ast::Int::new_const(&ctx, "t2");
    let t3 = ast::Int::new_const(&ctx, "t3");

    let e1 = &t1 * x1 + a1 - &x * &t1 - &a;
    let e2 = &t1 * y1 + b1 - &y * &t1 - &b;
    let e3 = &t1 * z1 + c1 - &z * &t1 - &c;
    let e4 = &t2 * x2 + a2 - &x * &t2 - &a;
    let e5 = &t2 * y2 + b2 - &y * &t2 - &b;
    let e6 = &t2 * z2 + c2 - &z * &t2 - &c;
    let e7 = &t3 * x3 + a3 - &x * &t3 - &a;
    let e8 = &t3 * y3 + b3 - &y * &t3 - &b;
    let e9 = &t3 * z3 + c3 - &z * &t3 - &c;

    let zero = ast::Int::from_i64(&ctx, 0);
    solver.assert(&e1._eq(&zero));
    solver.assert(&e2._eq(&zero));
    solver.assert(&e3._eq(&zero));
    solver.assert(&e4._eq(&zero));
    solver.assert(&e5._eq(&zero));
    solver.assert(&e6._eq(&zero));
    solver.assert(&e7._eq(&zero));
    solver.assert(&e8._eq(&zero));
    solver.assert(&e9._eq(&zero));
    solver.assert(&t1.gt(&zero));
    solver.assert(&t2.gt(&zero));
    solver.assert(&t3.gt(&zero));

    match solver.check() {
        SatResult::Sat => {
            let model = solver.get_model().unwrap();

            // let solution_x = model.eval(&x, true).unwrap().as_i64().unwrap();
            // let solution_y = model.eval(&y, true).unwrap().as_i64().unwrap();
            // let solution_z = model.eval(&z, true).unwrap().as_i64().unwrap();
            let solution_a = model.eval(&a, true).unwrap().as_i64().unwrap();
            let solution_b = model.eval(&b, true).unwrap().as_i64().unwrap();
            let solution_c = model.eval(&c, true).unwrap().as_i64().unwrap();

            println!("Solution found:");
            println!("a = {:?}", solution_a);
            println!("b = {:?}", solution_b);
            println!("c = {:?}", solution_c);
            return (solution_a + solution_b + solution_c) as usize;
        }
        _ => {
            println!("No solution found.");
        }
    }
    0
}

pub async fn advent_2(data: String) -> usize {
    let hailstones = parse1(data);
    return solve2(hailstones);
}