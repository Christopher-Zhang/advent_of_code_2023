use indexmap::IndexMap;

type Rule = (String, usize, String);
type Workflow = IndexMap<String, Rule>;
type Workflows = IndexMap<String, IndexMap<String, Rule>>;
type Part = IndexMap<String, usize>;
type Parts = Vec<Part>;

pub async fn advent(data: String) -> usize {
    let mut answer = 0;
    let (workflows, parts) = process_input(data);
    for part in parts {
        if solve(&part, &workflows) {
            let mut part_total = 0;
            part.values().for_each(|v| part_total += v);
            println!("Accepted {:?}, total: {part_total}", part);
            answer += part_total;
        }
        else {
            println!("Rejected {:?}", part);
        }
    }
    return answer;
}

pub async fn advent_2(data: String) -> usize {
    let mut answer = 0;

    return answer;
}

fn solve(part: &Part, workflows: &Workflows) -> bool {
    let mut workflow_name = "in";
    loop {
        let workflow = workflows.get(workflow_name).unwrap();
        // println!("checking workflow {}, {:?}", workflow_name, workflow);
        for (k,v) in workflow {
            if v.0 == "" {
                match k.as_str() {
                    "R" => {
                        return false
                    },
                    "A" => {
                        return true
                    },
                    _ => {
                        workflow_name = &k.as_str();
                        // println!("workflowname is now {workflow_name}");
                        break;
                    }
                }
            }
            else {
                let part_val = part.get(k).unwrap();
                let op = &v.0;
                let other = v.1;
                let then = &v.2;
                let check: bool;
                match op.as_str() {
                    ">" => check = part_val > &other,
                    "<" => check = part_val < &other,
                    _ => panic!("uh oh")
                }
                // println!("{} is {} {} ? then do {}", part_val, op, other, then);
                if check {
                    match then.as_str() {
                        "A" => return true,
                        "R" => return false,
                        _ => workflow_name = then
                    }
                    // println!("workflowname is now {then}");
                    break;
                }
                
            }
        }
    }
}
// 550124 too high

fn process_input(data: String) -> (Workflows, Parts) {
    let mut workflows: Workflows = IndexMap::new();
    let mut parts: Parts = Vec::new();

    let split: Vec<&str> = data.split("\n\n").collect();
    for line in split[0].lines() {
        let mut workflow: Workflow = IndexMap::new();
        let split: Vec<&str> = line.split('{').collect();
        let name = split[0].to_string();
        let line = &split[1][0..split[1].len()-1]; 
        let split: Vec<&str> = line.split(',').collect();
        for item in split {
            println!("{item}");
            let split: Vec<&str> = item.split(':').collect();
            if split.len() > 1 {
                let a = split[0][0..1].to_string();
                let b = split[0][1..2].to_string();
                let c = split[0][2..].parse::<usize>().unwrap();
                let d = split[1].to_string();
                // let rule = item[1..].to_string();
                
                workflow.insert(a, (b,c,d));
            }
            else {
                workflow.insert(item.to_string(), (String::from(""), 0, String::from("")));
            }
        }
        workflows.insert(name, workflow);
    }

    for line in split[1].lines() {
        let line = &line[1..line.len()-1];
        println!("{line}");
        let mut new_part: Part = IndexMap::new();
        for part in line.split(',') {
            let terms: Vec<&str> = part.split('=').collect();
            // let c = terms[0].chars().nth(0).unwrap();
            let num = terms[1].parse::<usize>().unwrap();
            new_part.insert(terms[0].to_string(),num);
        }
        parts.push(new_part);
    }
    // dbg!(&workflows);
    dbg!(&parts);
    return (workflows, parts);
}