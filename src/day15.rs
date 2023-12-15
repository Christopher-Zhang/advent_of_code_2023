pub async fn advent(data: String) -> usize {
    let mut answer = 0;
    for word in data.split(',') {
        let hash = hash_string(word);
        answer += hash;
    }
    return answer;
}
fn hash_string(st: &str) -> usize {
    let mut hash = 0;
    for c in st.chars() {
        hash = do_hash(c, hash);
    }
    return hash;
}
fn do_hash(c: char, hash: usize) -> usize {
    let mut hash = hash;
    hash += get_ascii(c) as usize;
    hash *= 17;
    hash = hash % 256;
    return hash;
}

pub async fn advent_2(data: String) -> usize {
    let mut answer = 0;
    let mut boxes: Vec<Vec<(String, usize)>> = Vec::new();
    for _ in 0..256 {
        boxes.push(Vec::new());
    }
    for step in data.split(',') {
        if let Some(index) = step.chars().position(|c| c == '=') {
            let label = step[0..index].to_string();
            let focus: usize = step[index+1..].parse::<usize>().unwrap();
            let box_num = hash_string(&label.as_str());

            if let Some(index) = boxes[box_num].iter().position(|v| v.0 == label) {
                boxes[box_num][index].1 = focus;
            }
            else {
                boxes[box_num].push((label, focus));
            }
        }
        else if let Some(minus) = step.chars().position(|c| c == '-'){
            let label = step[0..minus].to_string();
            let box_num = hash_string(&label.as_str());
            if let Some(index) = boxes[box_num].iter().position(|v| v.0 == label) {
                boxes[box_num].remove(index);
            }
        }
        else {
            panic!("Error parsing input");
        }
    }
    for (i, box_vec) in boxes.iter().enumerate() {
        let mut total = 0;
        let a = i + 1;
        for (j, lens) in box_vec.iter().enumerate() {
            let b = j + 1;
            let c = lens.1;
            total += a * b * c;
        }
        answer += total;
    }
    return answer;
}

fn get_ascii (c: char) -> u8 {
    return c.to_ascii_lowercase() as u8;
}
