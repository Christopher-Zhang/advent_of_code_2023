use regex::Regex;

pub async fn advent(data: String) -> usize {
    let (workflows, parts) = parse(data);
    solve(&workflows, &parts)
}

fn solve(workflows: &Vec<Workflow>, parts: &Vec<Part>) -> usize {
    let mut answer = 0;
    for part in parts {
        let mut cur_name = "in";
        'Workflow: loop {
            if cur_name == "R" {
                break;
            }
            if cur_name == "A" {
                answer += part.x + part.m + part.a + part.s;
                break;
            }

            let mut cur_workflow = workflows.iter().find(|workflow| workflow.name == cur_name.to_string()).unwrap();
            for rule in cur_workflow.rules.iter() {
                if rule.rule == 0 {
                    cur_name = rule.target.as_str();
                    continue 'Workflow;
                }
                let mut comp = match rule.letter {
                    'x' => part.x,
                    'm' => part.m,
                    'a' => part.a,
                    's' => part.s,
                    _ => panic!("unexpected letter")
                };
                if rule.rule < 0 {
                    comp *= -1;
                }
                if comp > rule.rule {
                    cur_name = rule.target.as_str();
                    continue 'Workflow;
                }
            }
        }
    }
    answer as usize
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64
}

#[derive(PartialEq, Debug, Clone)]
struct Rule {
    letter: char,
    rule: i64,
    target: String
}

#[derive(PartialEq, Debug, Clone)]
struct Workflow {
    name: String,
    rules: Vec<Rule>
}

fn parse(data: String) -> (Vec<Workflow>, Vec<Part>) {
    let data = data.split("\n\n").collect::<Vec<&str>>();
    let workflows = data[0].to_string();
    let parts = data[1].to_string();
    let mut ret_workflows = Vec::<Workflow>::new();
    let mut ret_parts = Vec::<Part>::new();
    for workflow in workflows.lines() {
        let mut rules = Vec::<Rule>::new();

        let re = Regex::new(r"(\S+)\{(.+)\}").unwrap();
        let Some(matches) = re.captures(workflow) else {
            panic!("failed to parse");
        };

        let name = matches[1].to_string();
        let rule_texts = matches[2].to_string();
        for rule_text in rule_texts.split(',') {
            let rule: Rule;
            if rule_text.contains(':') {
                let re = Regex::new(r"(\S+)(>|<)(\S+):(\S+)").unwrap();
                let Some(matches) = re.captures(rule_text) else {
                    panic!("failed to match");
                };
                let letter = matches[1].chars().nth(0).unwrap();
                let mut number = matches[3].parse::<i64>().unwrap();
                if &matches[2] == "<" {
                    number *= -1;
                }
                rule = Rule{
                    letter,
                    rule: number,
                    target: matches[4].to_string()
                }
            }
            else {
                rule = Rule {
                    letter: ' ',
                    rule: 0,
                    target: rule_text.to_string()
                };
            }
            rules.push(rule);
        }
        ret_workflows.push(Workflow { name, rules });
    }

    for line in parts.lines() {
        let re = Regex::new(r"x=(\d+),m=(\d+),a=(\d+),s=(\d+)").unwrap();
        let Some(matches) = re.captures(line) else {
            panic!("failed to match");
        };
        let x = matches[1].parse::<i64>().unwrap();
        let m = matches[2].parse::<i64>().unwrap();
        let a = matches[3].parse::<i64>().unwrap();
        let s = matches[4].parse::<i64>().unwrap();
        ret_parts.push(Part { x,m,a,s });
    }
    (ret_workflows, ret_parts)
}

pub async fn advent_2(data: String) -> usize {
    let mut answer = 0;

    return answer;
}