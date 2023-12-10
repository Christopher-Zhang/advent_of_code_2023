use std::collections::{HashMap, HashSet};

type Graph = HashMap<(usize, usize), char>;

pub async fn advent(data: String) -> usize {
    let mut graph: Graph = HashMap::new();
    let mut start = (0,0);
    let mut width = 0;
    let mut height = 0;
    for (y, line) in data.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (x,y);
            }
            graph.insert((x,y), c);
            width = std::cmp::max(width, x);
        }
        height = std::cmp::max(height, y);
    }
    let answer = traverse((usize::MAX, usize::MAX), start, &graph, start, true);
    return answer / 2;
}

fn traverse(from: (usize, usize), current: (usize, usize), graph: &Graph, start: (usize, usize), is_first: bool) -> usize {
    if current == start && !is_first{
        return 0;
    }
    let c = graph.get(&current).unwrap();
    let mut next: (usize, usize);
    match c {
        '|' => {
            next = (current.0, current.1 + 1);
            if !is_valid(next, from, &graph) {
                next = (current.0, current.1 - 1);
            }
        },
        '-' => {
            next = (current.0 + 1, current.1);
            if !is_valid(next, from, &graph) {
                next = (current.0 - 1, current.1);
            }
        },
        'L' => {
            next = (current.0, current.1 - 1);
            if !is_valid(next, from, &graph) {
                next = (current.0 + 1, current.1);
            }
        },
        'J' => {
            next = (current.0, current.1 - 1);
            if !is_valid(next, from, &graph) {
                next = (current.0 - 1, current.1);
            }
        },
        '7' => {
            next = (current.0, current.1 + 1);
            if !is_valid(next, from, &graph) {
                next = (current.0 - 1, current.1);
            }
        },
        'F' => {
            next = (current.0, current.1 + 1);
            if !is_valid(next, from, &graph) {
                next = (current.0 + 1, current.1);
            }
        },
        '.' => {
            panic!("failstate");
        }
        // NOTE this has to be hardcoded based on the input :)
        'S' => {
            next = (current.0, current.1 - 1);
            if !is_valid(next, from, &graph) {
                next = (current.0 + 1, current.1);
            }
        },
        _ => {
            panic!("failstate");
        }
    };

    return traverse(current, next, graph, start, false) + 1;
}
fn _print_graph(graph: &Graph) -> () {
    let mut x: usize;
    let mut y = 0;
    loop {
        let mut st = "".to_string();
        x = 0;
        if let None = graph.get(&(x,y)) {
            break;
        }
        loop {
            if let None = graph.get(&(x,y)) {
                break;
            }
            let mut c = graph.get(&(x,y)).unwrap().clone();
            if c == '@' {
                c = '.';
            }
            st.push(c);
            x += 1;
        }
        println!("{}", st);
        y += 1;
    }
}
fn is_valid(next: (usize, usize), from: (usize, usize), graph: &Graph) -> bool {
    match graph.get(&next) {
        Some(_) => return next != from,
        None => return false 
    };
}

pub async fn advent_2(data: String) -> i32 {
    let mut graph: Graph = HashMap::new();
    let mut start = (0,0);
    let mut width = 0;
    let mut height = 0;
    for (y, line) in data.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (x,y);
            }
            graph.insert((x,y), c);
            width = std::cmp::max(width, x);
        }
        height = std::cmp::max(height, y);
    }
    width += 1;
    height += 1;
    let mut marked: Graph = HashMap::new();
    mark_loop((usize::MAX, usize::MAX), start, &graph, &mut marked, start, true);
    for y in 0..height {
        for x in 0..width {
            if let None = marked.get(&(x,y)) {
                marked.insert((x,y), '.');
            }
        }
    }

    let mut expanded = expand(width, height, marked);
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    let mut total = 0;
    for y in 0..height*2 {
        for x in 0..width*2 {
            let mut marked: HashSet<(i32, i32)> = HashSet::new();
            let size = bfs((x as i32, y as i32), &mut expanded, &mut seen, &mut marked);
            // if size <= 0 {
            //     for (i,j) in marked {
            //         let i = i as usize;
            //         let j = j as usize;
            //         if let Some(c) = expanded.get(&(i,j)) {
            //             if c.clone() == 'I' {
            //                 expanded.insert((i,j), '.');
            //             }
            //         }
            //     }
            // }
            if size > 0 {
                total += size;
            }
            
        }
    }
    return total;
}

fn mark_loop(from: (usize, usize), current: (usize, usize), graph: &Graph, marked: &mut Graph, start: (usize, usize), is_first: bool) -> () {
    if current == start && !is_first{
        return;
    }
    let c = graph.get(&current).unwrap();
    marked.insert(current, c.clone());
    let mut next: (usize, usize);
    match c {
        '|' => {
            next = (current.0, current.1 + 1);
            if !is_valid(next, from, &graph) {
                next = (current.0, current.1 - 1);
            }
        },
        '-' => {
            next = (current.0 + 1, current.1);
            if !is_valid(next, from, &graph) {
                next = (current.0 - 1, current.1);
            }
        },
        'L' => {
            next = (current.0, current.1 - 1);
            if !is_valid(next, from, &graph) {
                next = (current.0 + 1, current.1);
            }
        },
        'J' => {
            next = (current.0, current.1 - 1);
            if !is_valid(next, from, &graph) {
                next = (current.0 - 1, current.1);
            }
        },
        '7' => {
            next = (current.0, current.1 + 1);
            if !is_valid(next, from, &graph) {
                next = (current.0 - 1, current.1);
            }
        },
        'F' => {
            next = (current.0, current.1 + 1);
            if !is_valid(next, from, &graph) {
                next = (current.0 + 1, current.1);
            }
        },
        '.' => {
            panic!("failstate");
        }
        // NOTE this has to be hardcoded based on the input :)
        'S' => {
            next = (current.0, current.1 - 1);
            if !is_valid(next, from, &graph) {
                next = (current.0 + 1, current.1);
            }
        },
        _ => {
            panic!("failstate");
        }
    };

    mark_loop(current, next, graph, marked, start, false);
}

fn expand(width: usize, height: usize, graph: Graph) -> Graph {
    let mut expanded: Graph = HashMap::new();
    for y in 0..height {
        for x in 0..width {
            let c = graph.get(&(x,y)).unwrap().clone();
            expanded.insert((x * 2, y * 2), c);
            match c{
                '|' => {
                    expanded.insert((x * 2, y * 2 + 1), c);
                    expanded.insert((x * 2, y * 2 - 1), c);
                },
                '-' => {
                    expanded.insert((x * 2 + 1, y * 2), c);
                    expanded.insert((x * 2 - 1, y * 2), c);
                },
                'L' => {
                    expanded.insert((x * 2 + 1, y * 2), '-');
                    expanded.insert((x * 2, y * 2 - 1), '|');
                },
                'J' => {
                    expanded.insert((x * 2 - 1, y * 2), '-');
                    expanded.insert((x * 2, y * 2 - 1), '|');
                },
                '7' => {
                    expanded.insert((x * 2 - 1, y * 2), '-');
                    expanded.insert((x * 2, y * 2 + 1), '|');
                },
                'F' => {
                    expanded.insert((x * 2 + 1, y * 2), '-');
                    expanded.insert((x * 2, y * 2 + 1), '|');
                },
                // NOTE this has to be hardcoded based on the input :)
                'S' => {
                    expanded.insert((x * 2 + 1, y * 2), '-');
                    expanded.insert((x * 2, y * 2 - 1), '|');
                },
                _ => {

                }
            } 
            
        }
    }

    for y in 0..height*2 {
        for x in 0..width*2 {
            if let None = expanded.get(&(x,y)) {
                expanded.insert((x,y), '@');
            }
        }
    }
    return expanded;
}

fn _shrink(width: usize, height: usize, graph: Graph) -> Graph {
    let mut shrunk: Graph = HashMap::new();
    for y in 0..height {
        if y % 2 != 0 {
            continue;
        }
        for x in 0..width {
            if x % 2 != 0 {
                continue;
            }
            let c = graph.get(&(x,y)).unwrap().clone();
            if c == '.' {
                shrunk.insert((x/2, y/2), 'O');
            }
            else {
                shrunk.insert((x/2, y/2), c);
            }
        }
    }
    return shrunk;
}

fn _dfs(cur: (i32, i32), graph: &mut Graph, seen: &mut HashSet<(i32, i32)>, marked: &mut HashSet<(i32, i32)>) -> i32 {
    if cur.0 < 0 || cur.1 < 0 {
        return i32::MIN;
    }
    if let None = graph.get(&(cur.0 as usize, cur.1 as usize)) {
        return i32::MIN;
    }

    let c = graph.get(&(cur.0 as usize, cur.1 as usize)).unwrap().clone();
    if seen.contains(&cur) {
        return 0;
    }
    seen.insert(cur);
    if c != '.' && c != '@' {
        return 0;
    }
    let up = _dfs((cur.0, cur.1 - 1), graph, seen, marked);
    let down = _dfs((cur.0, cur.1 + 1), graph, seen, marked);
    let left = _dfs((cur.0 - 1, cur.1), graph, seen, marked);
    let right = _dfs((cur.0 + 1, cur.1), graph, seen, marked);
    if up < 0 {
        return i32::MIN;
    }
    if down < 0 {
        return i32::MIN;
    }
    if left < 0 {
        return i32::MIN;
    }
    if right < 0 {
        return i32::MIN;
    }
    let increment = match c {
        '.' => 1,
        '@' => 0,
        _ => 0
    };
    if c == '.' {
        marked.insert(cur);
        graph.insert((cur.0 as usize, cur.1 as usize), 'I');
    }
    return up + left + down + right + increment;
}

fn bfs(cur: (i32, i32), graph: &mut Graph, seen: &mut HashSet<(i32, i32)>, marked: &mut HashSet<(i32, i32)>) -> i32 {
    use std::collections::VecDeque;
    let mut queue: VecDeque<(i32,i32)> = VecDeque::new();
    queue.push_back(cur);
    let mut size = 0;
    while queue.len() != 0 {
        let cur = queue.pop_front().unwrap();
        if cur.0 < 0 || cur.1 < 0 {
            size =  i32::MIN;
            continue;
        }
        if let None = graph.get(&(cur.0 as usize, cur.1 as usize)) {
            size =  i32::MIN;
            continue;
        }
        if seen.contains(&cur) {
            continue;
        }
        seen.insert(cur);
        let c = graph.get(&(cur.0 as usize, cur.1 as usize)).unwrap().clone();
        if c != '.' && c != '@' {
            continue;
        }
        let increment = match c {
            '.' => 1,
            '@' => 0,
            _ => 0
        };
        if c == '.' {
            marked.insert(cur);
            graph.insert((cur.0 as usize, cur.1 as usize), 'I');
        }

        size += increment;
        let directions = [(cur.0, cur.1 - 1),(cur.0, cur.1 + 1),(cur.0 - 1, cur.1),(cur.0 + 1, cur.1)];
        for dir in directions {
            queue.push_back(dir);
        }
    }
    return size;
}