use std::str;
use std::fs;
use std::collections::HashMap;
use anyhow::Result;
use anyhow::anyhow;

pub fn path_exists(path: &str) -> bool {
    let metadata = fs::metadata(path);
    match metadata {
        Ok(v) => v.is_file(),
        Err(_) => false
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "./input";
    let data;
    data = fs::read_to_string(path).unwrap();

    let mut sum = 0;
    let mut grid: Vec<Vec<char>> = vec!();
    let mut checked: Vec<Vec<bool>> = vec!();
    for line in data.lines() {
        let vec: Vec<char> = line.chars().collect();
        grid.push(vec.clone());
        checked.push(vec![false; vec.len()]);
    }
    let mut gears: HashMap<(u32, u32), Vec<u32>> = HashMap::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            // println!("Checking x:{}, y:{}", x, y);
            check_tile(x as u32, y as u32, &grid, &mut checked, &mut gears).unwrap();
            // println!("{:?}", grid);
            // println!("{:?}", checked);
            // println!("{}", get_number(y as u32, row));
        }
    }
    println!("{:?}", gears);
    for gear in gears.values() {
        if gear.len() == 2 {
            sum += gear[0] * gear[1];
        }
    }

    println!("Final total: {}", sum);
    Ok(())
}

fn get_char(x: u32, y: u32, grid: &Vec<Vec<char>>) -> Result<char> {  
    if y >= (grid.len() as u32) {
        return Err(anyhow!("Out of bounds"));
    }
    let row = grid[y as usize].clone();
    if x >= (row.len() as u32) {
        return Err(anyhow!("Out of bounds"));
    }
    Ok(grid[y as usize][x as usize])
}
fn get_bool(x: u32, y: u32, checked: &Vec<Vec<bool>>) -> Result<bool> {
    if y >= (checked.len() as u32) {
        return Err(anyhow!("Out of bounds"));
    }
    let row = checked[y as usize].clone();
    if x >= (row.len() as u32) {
        return Err(anyhow!("Out of bounds"));
    }
    Ok(checked[y as usize][x as usize])
}

fn get_number(x: u32, row: &Vec<char>) -> u32 {
    // println!("checking row {}: {:?}", x, row);
    let mut cur = row[x as usize];
    let mut left = x.clone() as i32;
    while cur.is_digit(10) && left >= 0 {
        left -= 1;
        if left < 0 {
            break;
        }
        cur = row[left as usize];
    }
    
    let mut cur = row[x as usize];
    let mut right = x.clone() as i32;
    while cur.is_digit(10) && (right < (row.len() as i32)){
        right += 1;
        if right == (row.len() as i32) {
            break;
        }
        cur = row[right as usize];
    }
    // println!("left:{}, right:{}", left, right);
    let st: String = row[(left+1) as usize..right as usize].into_iter().collect();
    let num = st.parse::<u32>().unwrap();
    return num;

}

fn mark_number(x: u32, row: &Vec<char>, checked: &mut Vec<Vec<bool>>, y: u32) -> () {
    let mut cur = row[x as usize];
    let mut left = x.clone() as i32;
    while cur.is_digit(10) && left >= 0 {
        left -= 1;
        if left < 0 {
            break;
        }
        cur = row[left as usize];
    }
    
    let mut cur = row[x as usize];
    let mut right = x.clone() as i32;
    while cur.is_digit(10) && (right < (row.len() as i32)){
        right += 1;
        if right == (row.len() as i32) {
            break;
        }
        cur = row[right as usize];
    }

    for x in left+1..right {
        checked[y as usize][x as usize] = true;
    }
}

fn is_engine(x: u32, y: u32, grid: &Vec<Vec<char>>, checked: &mut Vec<Vec<bool>>, gears: &mut HashMap<(u32,u32), Vec<u32>>) -> bool {

    if get_bool(x, y, checked).unwrap_or(true) {
        return false;
    }
    let val = get_char(x, y, grid).unwrap_or('x');
    if val == 'x' {
        return false;
    }
    if !val.is_digit(10) && val != '.' {
        return true;
    }

    let directions = [
        [-1, -1],
        [-1, 0 ],
        [0, -1 ],
        [1, 0  ],
        [0, 1  ],
        [1, 1  ],
        [-1, 1 ],
        [1, -1 ],
    ];
    checked[y as usize][x as usize] = true;
        
    let mut valid: bool = false;
    for dir in directions {
        let new_x = x as i32 + dir[0];
        let new_y = y as i32 + dir[1];
        if new_x < 0 || new_y < 0 {
            continue;
        }

        let new_val = get_char(new_x as u32, new_y as u32, grid).unwrap_or('.');
        if new_val.is_digit(10) {
            if dir[1] == 0 {
                checked[y as usize][x as usize] = true;
                if is_engine(new_x as u32, new_y as u32, grid, checked, gears) {
                    valid = true;
                    continue;
                }
            }
        }
        else if new_val != '.' {
            let full_num = get_number(x, &grid[y as usize]);
            match gears.get_mut(&(new_x as u32,new_y as u32)) {
                Some(vec) => {
                    if !vec.contains(&full_num) {
                        vec.push(full_num);
                    }
                },
                None => {
                    gears.insert((new_x as u32,new_y as u32), vec![full_num]);
                }
            };
            valid = true;
        }
    }

    return valid;
}

fn check_tile(x: u32, y: u32, grid: &Vec<Vec<char>>, checked: &mut Vec<Vec<bool>>, gears: &mut HashMap<(u32,u32), Vec<u32>>) -> Result<u32> {
    let check = get_bool(x, y, checked).unwrap_or(true);
    if check {
        return Ok(0);
    }
    
    let val: char = get_char(x, y, grid).unwrap_or('.');
    if val.is_digit(10) {
        
        if is_engine(x, y, grid, checked, gears) {
            // ret += full_num;

            mark_number(x, &grid[y as usize], checked, y);
        }
    }
    checked[y as usize][x as usize] = true;
    Ok(0)
}
