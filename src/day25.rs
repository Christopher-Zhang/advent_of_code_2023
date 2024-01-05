use bimap::BiMap;
use itertools::Itertools;
use counter::Counter;
use rand::{thread_rng, Rng};
use pathfinding::prelude::{dijkstra, bfs_reach};
use std::collections::HashSet;

type Mapping = BiMap<String, usize>;
type Graph = Vec<Vec<bool>>;
pub async fn advent(data: String) -> usize {
    let mut answer = 0;
    while answer == 0 {
        let (mut graph, mapping) = parse(data.clone());
        answer = solve(&mut graph, &mapping);
    }
    answer
}

fn solve(graph: &mut Graph, mapping: &Mapping) -> usize {
    let mut rng = thread_rng();
    for _ in 0..3 {
        let mut counter = [0 as usize; 0].iter().cloned().collect::<Counter<_>>();
        for _ in 0..1000 {

            let start = rng.gen_range(0..mapping.len());
            let end = rng.gen_range(0..mapping.len());

            let wrapped = dijkstra(&start, |n: &usize| {graph[*n].iter().enumerate().filter_map(|(index, b)| {if *b {return Some((index, 1))} None})}, |n: &usize| { n == &end });
            let result = match wrapped {
                Some(r) => r,
                None => {
                    panic!("failed");
                }
            };
            let vec = result.0;
            let new_counter = vec.iter().cloned().collect::<Counter<_>>();
            counter.extend(&new_counter);
        }
        let a = counter.most_common()[0].0;
        let b = counter.most_common()[1].0;
        graph[a][b] = false;
        graph[b][a] = false;
    }

    let groups = check_group(&graph);

    if groups.0 == 0 || groups.1 == 0 {
        return 0;
    }
    println!("{} {}", groups.0, groups.1);
    groups.0 * groups.1

        
}

fn check_group (graph: &Graph) -> (usize, usize) {

    let mut rng = thread_rng();
    let start = rng.gen_range(0..graph.len());

    let group = bfs_reach(start, |n| {
        graph[*n].iter().enumerate().filter_map(|(i, b)|{
            if *b {
                return Some(i);
            }
            None
        })
    }).collect_vec();

    (group.len(), graph.len() - group.len())
}
pub async fn advent_2(_: String) -> usize {0}

fn parse(data: String) -> (Graph, Mapping) {
    let mut mapping: Mapping = BiMap::new();
    let mut set = HashSet::<String>::new();
    for line in data.lines() {
        let split = line.split(": ").collect_vec();
        let mut nodes = split[1].split(' ').collect_vec();
        nodes.push(split[0]);
        for node in nodes {
            if !set.contains(node) {
                set.insert(node.to_string());
            }
        }
    }
    for (i, node) in set.iter().enumerate() {
        mapping.insert(node.to_string(), i);
    }
    let node_count = set.len();
    let mut graph: Graph = Vec::with_capacity(node_count);
    for _ in 0..node_count {
        graph.push(vec![false; node_count]);
    }
    for line in data.lines() {
        let split = line.split(": ").collect_vec();
        let source = split[0];
        let nodes = split[1].split(' ').collect_vec();
        
        let a = mapping.get_by_left(source).unwrap();
        for node in nodes {
            // println!("{node}");
            let b = mapping.get_by_left(node).unwrap();
            graph[*a][*b] = true;
            graph[*b][*a] = true;
        }
    }
    (graph, mapping)
}
