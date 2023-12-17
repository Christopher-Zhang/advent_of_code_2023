
#[derive(PartialEq, Debug, Clone, Copy)]
enum Direction {
    N,
    S,
    E,
    W
}
type Grid = Vec<Vec<usize>>;
pub async fn advent(data: String) -> usize {
    let mut answer = 0;
    let mut grid: Grid = Vec::new();
    for line in data.lines() {
        let mut row: Vec<usize> = Vec::new();
        for c in line.chars() {
            row.push(char::to_digit(c, 10).unwrap() as usize);
        }
        grid.push(row);
    }


    return answer;
}

fn traverse(x: i32, y: i32, dir: Direction, steps_left: usize, grid: Grid) -> usize {
    if x < 0 || y < 0 || y as usize >= grid.len() || x as usize >= grid[0].len() {
        return 0;
    }



    return 0;
}

fn get_next_point(x: i32, y: i32, dir: Direction) -> (i32, i32) {
    match dir {
        Direction::E => {
            return (x+1, y);
        },
        Direction::N => {
            return (x, y-1);
        },
        Direction::S => {
            return (x, y+1);
        },
        Direction::W => {
            return (x-1, y);
        }
    };
}

fn get_next_dir(dir: Direction, need_turn: bool) -> Direction {
    
    dir
}

pub async fn advent_2(data: String) -> usize {
    let mut answer = 0;

    return answer;
}