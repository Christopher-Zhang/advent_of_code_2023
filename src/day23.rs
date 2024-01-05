use itertools::Itertools;
use pathfinding::prelude::astar;
use bimap::BiMap;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum Tile {
    Wall,
    SlopeNorth,
    SlopeSouth,
    SlopeEast,
    SlopeWest,
    Path,
}


impl Tile {
    fn _to_char(&self) -> char {
        match *self {
            Tile::Wall => '#',
            Tile::SlopeEast => '>',
            Tile::SlopeWest => '<',
            Tile::SlopeNorth => '^',
            Tile::SlopeSouth => 'v',
            Tile::Path => '.',
        }
    }

    fn from_char(c: char) -> Tile {
        match c {
            '#' => Tile::Wall,
            '>' => Tile::SlopeEast,
            '<' => Tile::SlopeWest,
            '^' => Tile::SlopeNorth,
            'v' => Tile::SlopeSouth,
            '.' => Tile::Path,
            _ => panic!("unexpected char"),
        }
    }

    fn get_dir(&self) -> (i64, i64) {
        match *self {
            Tile::Wall => (0,0),
            Tile::SlopeNorth => (0,-1),
            Tile::SlopeSouth => (0,1),
            Tile::SlopeEast => (1,0),
            Tile::SlopeWest => (-1,0),
            Tile::Path => (0,0),
        }
    }
}
type Grid = Vec<Vec<Tile>>;
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
struct Point {
    x: i64,
    y: i64,
}
pub async fn advent(data: String) -> usize {
    let grid = parse(data);
    // _print(&grid);
    solve(&grid)
}

fn solve(grid: &Grid) -> usize {
    let start = Point{x: 1, y: 0};
    let end = Point{x: (grid[0].len()-2) as i64, y: (grid.len()-1) as i64};
    let mut path = Vec::<Point>::new();
    longest(start, end, 0, &mut path, grid)
}

fn longest(current: Point, end: Point, dist: usize, path: &mut Vec<Point>, grid: &Grid) -> usize {
    let mut ret = dist;
    if current == end {
        return ret
    }

    for neighbor in get_neighbors(current, path, grid) {
        path.push(neighbor);
        let branch = longest(neighbor, end, dist + 1, path, grid);
        ret = std::cmp::max(ret, branch);
        path.pop();
    }
    ret
}
fn get_neighbors(point: Point, path: &Vec<Point>, grid: &Grid) -> Vec<Point> {
    let mut neighbors = Vec::<Point>::new();
    let tile = grid[point.y as usize][point.x as usize];
    match tile {
        Tile::Wall => panic!("unexpected wall"),
        Tile::SlopeEast | Tile::SlopeNorth | Tile::SlopeSouth | Tile::SlopeWest => {
            let dir = tile.get_dir();
            let next_point = Point{x: point.x + dir.0, y: point.y + dir.1};
            if is_valid(next_point, path, grid) {
                neighbors.push(next_point);
            }
        },
        Tile::Path => {
            // down
            let dir = (0, 1);
            let next_point = Point{x: point.x + dir.0, y: point.y + dir.1};
            if is_valid(next_point, path, grid) {
                neighbors.push(next_point);
            }
            // up
            let dir = (0, -1);
            let next_point = Point{x: point.x + dir.0, y: point.y + dir.1};
            if is_valid(next_point, path, grid) {
                neighbors.push(next_point);
            }
            // left
            let dir = (-1, 0);
            let next_point = Point{x: point.x + dir.0, y: point.y + dir.1};
            if is_valid(next_point, path, grid) {
                neighbors.push(next_point);
            }
            // right
            let dir = (1, 0);
            let next_point = Point{x: point.x + dir.0, y: point.y + dir.1};
            if is_valid(next_point, path, grid) {
                neighbors.push(next_point);
            }
        }
    }

    neighbors
}

fn is_valid(point: Point, path: &Vec<Point>, grid: &Grid) -> bool {
    let x = point.x;
    let y = point.y;
    if x < 0 || y < 0 {
        return false;
    }
    let x = x as usize;
    let y = y as usize;
    if y >= grid.len() || x >= grid[0].len() || path.contains(&point) || grid[y][x] == Tile::Wall {
        return false;
    }
    true
}
fn parse(data: String) -> Grid {
    data.lines()
        .map(|line| line.chars().map(|c| Tile::from_char(c)).collect_vec())
        .collect_vec()
}

fn _print(grid: &Grid) {
    grid.iter().for_each(|row| {
        println!(
            "{}",
            row.iter().map(|tile| tile._to_char()).collect::<String>()
        )
    });
}
fn _print_path(grid: &Grid, path: &Vec<Point>) {
    grid.iter().enumerate().for_each(|(j,row)| {
        println!(
            "{}",
            row.iter().enumerate().map(|(i,tile)| {
                let x = i as i64;
                let y = j as i64;
                if path.contains(&Point{x,y}) {
                    return 'O';
                }
                tile._to_char()
            }).collect::<String>()
        )
    });
}

pub async fn advent_2(data: String) -> usize {
    let grid = parse(data);
    // _print(&grid);
    solve2(&grid)
}

type Graph = Vec<Vec<usize>>;
type Mapping = BiMap<Point, usize>;
fn parse_graph(data: String) -> (Graph, Mapping) {
    let mut map: Mapping = BiMap::new();
    let mut graph: Graph = Vec::new();
    let mut index = 0;
    data.lines().enumerate().for_each(|(j, line)| {
        line.chars().enumerate().for_each(|(i,c)| {
            if c != '#' {
                let x = i as i64;
                let y = j as i64;
                map.insert(Point{x,y}, index);
                index += 1;
            }
        })
    });

    for (point, index) in map.iter() {
        
    }

    (graph, map)
}

// fn solve_graph();

fn solve2(grid: &Grid) -> usize {
    let start = Point{x: 1, y: 0};
    let end = Point{x: (grid[0].len()-2) as i64, y: (grid.len()-1) as i64};
    let mut path = Vec::<Point>::new();
    longest2(start, end, 0, &mut path, grid)
}

fn longest2(current: Point, end: Point, dist: usize, path: &mut Vec<Point>, grid: &Grid) -> usize {
    let mut ret = 0;
    if current == end {
        return dist;
    }

    for neighbor in get_neighbors2(current, path, grid) {
        path.push(neighbor);
        let branch = longest2(neighbor, end, dist + 1, path, grid);
        ret = std::cmp::max(ret, branch);
        path.pop();
    }
    ret
}

fn get_neighbors2(point: Point, path: &Vec<Point>, grid: &Grid) -> Vec<Point> {
    let mut neighbors = Vec::<Point>::new();
    let tile = grid[point.y as usize][point.x as usize];
    match tile {
        Tile::Wall => panic!("unexpected wall"),
        Tile::SlopeEast | Tile::SlopeNorth | Tile::SlopeSouth | Tile::SlopeWest | Tile::Path => {
            // down
            let dir = (0, 1);
            let next_point = Point{x: point.x + dir.0, y: point.y + dir.1};
            if is_valid(next_point, path, grid) {
                neighbors.push(next_point);
            }
            // up
            let dir = (0, -1);
            let next_point = Point{x: point.x + dir.0, y: point.y + dir.1};
            if is_valid(next_point, path, grid) {
                neighbors.push(next_point);
            }
            // left
            let dir = (-1, 0);
            let next_point = Point{x: point.x + dir.0, y: point.y + dir.1};
            if is_valid(next_point, path, grid) {
                neighbors.push(next_point);
            }
            // right
            let dir = (1, 0);
            let next_point = Point{x: point.x + dir.0, y: point.y + dir.1};
            if is_valid(next_point, path, grid) {
                neighbors.push(next_point);
            }
        }
    }
    neighbors
}