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
    let (workflows, _) = parse(data);
    return solve_2(&workflows);
}

type Ranges = [Range; 4];
type Range = (usize, usize);
fn solve_2(workflows: &Vec<Workflow>) -> usize {
    let start = workflows.iter().find(|w| w.name == "in").unwrap();
    let mut ranges: Ranges = [(1,4000); 4];
    dfs(start, &mut ranges, workflows)
}

fn dfs(workflow: &Workflow, ranges: &mut Ranges, workflows: &Vec<Workflow>) -> usize {
    let mut ret = 0;
    for rule in workflow.rules.iter() {
        if rule.rule == 0 {
            if rule.target == "A" {
                ret += get_ways(ranges);
            }
            else if rule.target != "R" {
                let next_workflow = workflows.iter().find(|w| w.name == rule.target).unwrap();
                ret += dfs(next_workflow, ranges, workflows);
            }
            break;
        }

        let range_index = match rule.letter {
            'x' => 0,
            'm' => 1,
            'a' => 2,
            's' => 3,
            _ => panic!("unexpected state")
        };
        let range = ranges[range_index];
        if range == (0,0) {
            return 0;
        }
        if let Some((yes_range, no_range)) = get_range(rule.rule, range) {
            if rule.target != "R" {
                let mut next_ranges = ranges.clone();
                next_ranges[range_index] = yes_range;
                if rule.target == "A" {
                    ret += get_ways(&next_ranges);
                }
                else {
                    let next_workflow = workflows.iter().find(|w| w.name == rule.target).unwrap();
                    ret += dfs(next_workflow, &mut next_ranges, workflows);
                }
            }
            ranges[range_index] = no_range;
        }
    }
    ret
}

fn get_range(rule: i64, range: Range) -> Option<(Range, Range)> {
    let zero = (0,0);
    if rule > 0 {
        let rule = rule as usize;
        if rule < range.1 {
            if rule < range.0 {
                return Some((range, zero));
            }
            return Some(
                ((rule + 1, range.1), (range.0, rule))
            );
        }
    }
    else if rule < 0 {
        let rule = (-rule) as usize;
        if rule > range.0 {
            if rule > range.1 {
                return Some((range, zero));
            }
            return Some(
                ((range.0, rule - 1), (rule, range.1))
            );
        }
    }
    None
}

fn get_ways(ranges: &Ranges) -> usize {
    ranges.iter().fold(1, |a,b| a * (b.1 - b.0 + 1))
}