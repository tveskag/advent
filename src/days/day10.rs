use std::collections::HashMap;

use crate::util::parse;
use itertools::{Combinations, Itertools};

fn trim(string: &str) -> &str {
    string.trim_matches(&['{', '}', '(', ')', '[', ']'][..])
}

fn build_graph(size: u32) -> HashMap<u16, (Vec<u16>, bool)> {
    let mut graph = HashMap::new();
    for binary in 0..2u16.pow(size) {
        let mut edges = Vec::new();
        for power in 0..size {
            let edge = binary ^ 2u16.pow(power);
            if edge < binary {
                edges.push(edge);
            }
        }
        graph.insert(binary, (edges, false));
    }
    graph
}

fn visit_all(graph: &mut HashMap<u16, (Vec<u16>, bool)>, vertex: u16) {
    let (nodes, _) = graph
        .entry(vertex)
        .and_modify(|(_, visited)| {
            *visited = true;
        })
        .or_default();

    for node in nodes.clone() {
        visit_all(graph, node);
    }
}

fn boolean_gaussian_elimination(vectors: Vec<u32>) -> Vec<u32> {
    let mut sorted = vectors.clone();
    sorted.sort();

    //println!("Sorted: {:?}", sorted);
    let Some((first, rest)) = sorted.split_first() else {
        return sorted;
    };

    let mut smaller: Vec<u32> = Vec::new();
    for vector in rest {
        let dif = first ^ vector;
        if dif < *first {
            smaller.push(dif);
        } else {
            smaller.push(*vector);
        }
    }
    let mut out = Vec::new();
    out.push(*first);
    if smaller.len() > 0 {
        [out, boolean_gaussian_elimination(smaller)].concat()
    } else {
        out
    }
}

fn find_rank(vectors: Vec<u32>) -> u32 {
    let gauss = boolean_gaussian_elimination(vectors);
    //println!("{:?}", gauss);
    gauss
        .iter()
        .fold(0, |acc, v| if *v == 0 { acc } else { acc + 1 })
}

pub fn run(input: &str) -> usize {
    let split = input
        .lines()
        .map(|line| {
            line.split_once(" ")
                .unwrap()
        })
        .map(|(indicator, rest)| {
            let (buttons, joltage) = rest
                .rsplit_once(" ")
                .unwrap();
            (trim(indicator), buttons, trim(joltage))
        });

    let mut counter = 0;
    for (indicator, buttons, _) in split {
        let indicator: u32 = indicator
            .chars()
            .rev()
            .fold(0b0u32, |acc, c| {
                if c == '#' {
                    let out = acc << 1;
                    out + 1
                } else {
                    acc << 1
                }
            });

        let size = buttons.len();
        let buttons: Vec<u32> = buttons
            .split(" ")
            .map(|button| {
                trim(button)
                    .split(",")
                    .fold(0b0u32, |acc, c| acc + 2u32.pow(parse::<u32>(c)))
            })
            .collect();

        // println!("Indicator: {:#018b}", indicator);
        // buttons
        //     .iter()
        //     .for_each(|b| println!("{:#018b}", b));

        let mut graph = build_graph(size);

        // for (node, edges) in graph.clone() {
        //     let edges_str = edges
        //         .iter()
        //         .fold(String::new(), |acc, e| format!("{}\n{:#016b}", acc, e));
        //     println!("{:#016b} -- {}\n\n", node, edges_str);
        // }

        let start = 2u16.pow(size) - 1;
        let min_span = dfs(indicator, &buttons, &mut graph, &start);
        println!("{min_span}");
        counter += min_span;

        // let amount = naive(indicator, buttons, size);
        // counter += amount;
    }
    counter as usize
}

fn naive(indicator: u32, buttons: Vec<u32>, size: usize) -> usize {
    for n in 1..size + 1 {
        for combination in buttons
            .iter()
            .combinations(n)
        {
            if combination
                .iter()
                .fold(indicator, |acc, button| acc ^ *button)
                == 0
            {
                println!("Hit: {:?}", combination);
                return n;
            }
        }
    }
    panic!("not found");
}

fn dfs(
    indicator: u32,
    buttons: &Vec<u32>,
    graph: &mut HashMap<u16, (Vec<u16>, bool)>,
    vertex: &u16,
) -> u32 {
    let (nodes, visited) = graph
        .get(vertex)
        .unwrap();

    if *visited {
        return vertex.count_ones() + 1;
    }

    let active: Vec<u32> = buttons
        .iter()
        .enumerate()
        .filter_map(|(i, b)| {
            if vertex >> i & 1 == 1 {
                Some(b.clone())
            } else {
                None
            }
        })
        .collect();

    let mut active_with_indicator = active.clone();
    let span_rank = find_rank(active);
    active_with_indicator.push(indicator);
    let indicator_rank = find_rank(active_with_indicator);

    if indicator_rank > span_rank {
        visit_all(graph, *vertex);
        return vertex.count_ones() + 1;
    }

    nodes
        .clone()
        .into_iter()
        .fold(32u32, |acc, node| {
            u32::min(acc, dfs(indicator, buttons, graph, &node))
        })
}
