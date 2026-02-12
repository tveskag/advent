use std::collections::{HashMap, HashSet};

pub fn run(input: &str) -> usize {
    let state: HashMap<&str, Vec<&str>> = HashMap::new();
    // let nodes_amount = input
    //     .lines()
    //     .count();
    let graph = input
        .lines()
        .fold(state, |mut acc, line| {
            let (label, nodes) = match line.split_once(":") {
                Some((label, nodes)) => (
                    label,
                    nodes
                        .trim()
                        .split(" ")
                        .collect(),
                ),
                None => panic!("Malformed input"),
            };
            acc.insert(label, nodes);
            acc
        });

    dfs(&graph, "jmb", &HashSet::new(), 0)
}

fn dfs(
    graph: &HashMap<&str, Vec<&str>>,
    vertex: &str,
    visited: &HashSet<&str>,
    sum: usize,
) -> usize {
    // if vertex == "dac" {
    //     return 0;
    // let dac = visited.contains("dac");
    // let fft = visited.contains("fft");
    // if dac && fft {
    //     return 1;
    // } else {
    //     return 0;
    // }
    // }
    if vertex == "out" {
        //println!("{:#?}", visited);
        return 1;
    }

    let nodes = graph
        .get(vertex)
        .unwrap();

    let mut new_visited = visited.clone();

    nodes
        .into_iter()
        .fold(sum, |acc, node| {
            if new_visited.contains(node) {
                acc
            } else {
                new_visited.insert(node);
                acc + dfs(graph, node, &new_visited, sum)
            }
        })
}
