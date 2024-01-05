use std::collections::{HashMap, VecDeque};
use indexmap::IndexSet;
static mut HIGH: usize = 0;
static mut LOW: usize = 0;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum GateType {
    FlipFlop,
    Broadcaster,
    Conjunction,
    None
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum Signal {
    High,
    Low
}

#[derive(Debug, Clone)]
struct Node {
    gate_type: GateType,
    name: String,
    sources: HashMap<String, Signal>,
    targets: IndexSet<String>,
    signal: Signal
}
fn flip(signal: &Signal) -> Signal {
    match signal {
        Signal::High => Signal::Low,
        Signal::Low => Signal::High,
    }
}
fn get_node_index(name: &str, nodes: &Vec<Node>) -> Option<usize> {
    nodes.iter().position(|v| v.name == name)
}
fn increment(signal: Signal, do_increment: bool) {
    if do_increment {
        unsafe {
            match signal {
                Signal::High => HIGH += 1,
                Signal::Low => LOW += 1
            };
        }
    }
}

fn send(from: String, to: String, signal: Signal, queue: &mut VecDeque<(String, Signal)>, debug: bool) {
    if debug {
        println!("{} {:?} {}", from, signal, to);
    }
    queue.push_back((to, signal));
}
fn button(nodes: &mut Vec<Node>, debug: bool, do_increment: bool) -> (bool,bool,bool,bool) {
    let mut queue: VecDeque<(String, Signal)> = VecDeque::new();
    queue.push_back((String::from("roadcaster"), Signal::Low));
    increment(Signal::Low, do_increment);
    let mut sb = false;
    let mut nd = false;
    let mut ds = false;
    let mut hf = false;
    while !queue.is_empty() {
        let (current_name, signal) = queue.pop_front().unwrap();
        // println!("{:?} sent to {}", signal, current_name);
        let node_index = get_node_index(&current_name, nodes).expect("failed to find node");
        if signal == Signal::Low {
            match current_name.as_str() {
                "sb" => sb = true,
                "nd" => nd = true,
                "ds" => ds = true,
                "hf" => hf = true,
                _ => ()
            };
        }
        match nodes[node_index].gate_type {
            GateType::Broadcaster => {
                for target in nodes[node_index].targets.iter() {
                    send(current_name.clone(), target.to_string(), signal, &mut queue, debug);
                    increment(signal, do_increment);
                }
            },
            GateType::FlipFlop => {
                if signal == Signal::Low {
                    let new_signal = flip(&nodes[node_index].signal);
                    nodes[node_index].signal = new_signal;

                    let mut targets: Vec<String> = Vec::new();
                    for target in nodes[node_index].targets.iter() {
                        targets.push(target.to_string());
                    }
                    for target in targets {
                        let target_node_index = get_node_index(target.as_str(), nodes).expect("failed to find node");
                        nodes[target_node_index].sources.insert(current_name.clone(), new_signal);
                        increment(new_signal, do_increment);
                        send(current_name.clone(), target.to_string(), new_signal, &mut queue, debug);
                    }
                }
            },
            GateType::Conjunction => {
                let mut output = Signal::Low;
                for (_, signal) in nodes[node_index].sources.iter() {
                    if signal == &Signal::Low {
                        output = Signal::High;
                        break;
                    }
                }
                nodes[node_index].signal = output;

                let mut targets: Vec<String> = Vec::new();
                for target in nodes[node_index].targets.iter() {
                    targets.push(target.to_string());
                }
                for target in targets {
                    increment(nodes[node_index].signal, do_increment);
                    let target_node_index = get_node_index(target.as_str(), nodes).expect("failed to find node");
                    nodes[target_node_index].sources.insert(current_name.clone(), output);
                    send(current_name.clone(), target.to_string(), nodes[node_index].signal, &mut queue, debug);
                }
            },
            _ => ()
        }
    }
    return (sb,nd,ds,hf);
}

pub async fn advent(data: String) -> usize {
    unsafe {
        HIGH = 0;
        LOW = 0;
    }
    let mut nodes: Vec<Node> = Vec::new();
    for line in data.lines() {
        let first_char = line.chars().nth(0).unwrap();
        let split: Vec<&str> = line.split(" -> ").collect();
        let name = &split[0][1..];
        let recipients: Vec<&str> = split[1].split(", ").collect();
        let gate_type = match first_char {
            '%' => GateType::FlipFlop,
            '&' => GateType::Conjunction,
            _ => GateType::Broadcaster
        };
        let signal = match gate_type {
            GateType::FlipFlop => Signal::Low,
            GateType::Broadcaster => Signal::Low,
            GateType::Conjunction => Signal::High,
            GateType::None => Signal::Low,
        };

        if let Some(node_index) = get_node_index(name, &mut nodes) {
            nodes[node_index].gate_type = gate_type;
            nodes[node_index].signal = signal;
            recipients.iter().for_each(|v| {
                nodes[node_index].targets.insert(v.to_string());
            });
        }
        else {
            let mut targets: IndexSet<String> = IndexSet::new();
            recipients.iter().for_each(|v| {
                targets.insert(v.to_string());
            });
            let node = Node {
                gate_type,
                name: name.to_string(),
                sources: HashMap::new(),
                targets,
                signal
            };
            nodes.push(node);
        }

        recipients.iter().for_each(|v| {
            if let Some(node_index) = get_node_index(v, &mut nodes) {
                nodes[node_index].sources.insert(name.to_string(), Signal::Low);
            }
            else {
                let mut sources: HashMap<String, Signal> = HashMap::new();
                sources.insert(name.to_string(), Signal::Low);
                let node = Node {
                    gate_type: GateType::None,
                    name: v.to_string(),
                    sources,
                    targets: IndexSet::new(),
                    signal
                };
                nodes.push(node);
            }
        });
    }

    for _ in 0..1000 {
        button(&mut nodes, false, true);
    }
    unsafe {
        return HIGH * LOW;
    }
}

pub async fn advent_2(data: String) -> usize {
    let mut nodes: Vec<Node> = Vec::new();
    for line in data.lines() {
        let first_char = line.chars().nth(0).unwrap();
        let split: Vec<&str> = line.split(" -> ").collect();
        let name = &split[0][1..];
        let recipients: Vec<&str> = split[1].split(", ").collect();
        let gate_type = match first_char {
            '%' => GateType::FlipFlop,
            '&' => GateType::Conjunction,
            _ => GateType::Broadcaster
        };
        let signal = match gate_type {
            GateType::FlipFlop => Signal::Low,
            GateType::Broadcaster => Signal::Low,
            GateType::Conjunction => Signal::High,
            GateType::None => Signal::Low,
        };

        if let Some(node_index) = get_node_index(name, &mut nodes) {
            nodes[node_index].gate_type = gate_type;
            nodes[node_index].signal = signal;
            recipients.iter().for_each(|v| {
                nodes[node_index].targets.insert(v.to_string());
            });
        }
        else {
            let mut targets: IndexSet<String> = IndexSet::new();
            recipients.iter().for_each(|v| {
                targets.insert(v.to_string());
            });
            let node = Node {
                gate_type,
                name: name.to_string(),
                sources: HashMap::new(),
                targets,
                signal
            };
            nodes.push(node);
        }

        recipients.iter().for_each(|v| {
            if let Some(node_index) = get_node_index(v, &mut nodes) {
                nodes[node_index].sources.insert(name.to_string(), Signal::Low);
            }
            else {
                let mut sources: HashMap<String, Signal> = HashMap::new();
                sources.insert(name.to_string(), Signal::Low);
                let node = Node {
                    gate_type: GateType::None,
                    name: v.to_string(),
                    sources,
                    targets: IndexSet::new(),
                    signal
                };
                nodes.push(node);
            }
        });
    }
    let mut i = 0;
    let mut counts: [usize; 4] = [0; 4];
    loop {
        i += 1;
        if i % 100000 == 0 {
            println!("{i} iterations...");
        }
        let results = button(&mut nodes, false, false);
        for (counter, result) in [results.0, results.1, results.2, results.3].iter().enumerate() {
            if *result && counts[counter] == 0 {
                println!("Period for {counter} is {i}");
                counts[counter] = i;
            }
        }
        if counts.iter().all(|v| v != &0) {
            break;
        } 
    }
    return counts.iter().fold(1, |a,b| a * b);
}
