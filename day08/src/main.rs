use std::collections::HashMap;
use std::process;

#[derive(Debug, Clone)]
struct Edges {
    left: String,
    right: String,
}

fn main() {
    let (instructions, graph) = include_str!("../data/input.txt")
        .split_once("\n\n")
        .unwrap();
    let mut instructions: Vec<char> = instructions.chars().collect();
    let graph: HashMap<String, Edges> = graph
        .split('\n')
        .map(|l| {
            let tuple = String::from(l.replace(&['(', ')', ' '][..], ""));
            let (prefix, suffix) = tuple.split_once('=').unwrap();
            let (left, right) = suffix.split_once(',').unwrap();
            (
                String::from(prefix),
                Edges {
                    left: String::from(left),
                    right: String::from(right),
                },
            )
        })
        .collect::<Vec<(String, Edges)>>()
        .into_iter()
        .collect();

    println!(
        "The answer to part one is {:?}",
        solve_part_one(instructions.clone(), graph.clone())
    );
    //println!(
    //    "The answer to part two is {:?}",
    //    solve_part_two(instructions.clone(), graph.clone())
    //);
}

fn solve_part_one(mut instructions: Vec<char>, graph: HashMap<String, Edges>) -> i32 {
    let mut n_steps: i32 = 0;
    let mut curr_node: &str = "AAA";
    while curr_node != "ZZZ" {
        match instructions.first().unwrap() {
            'L' => {
                n_steps += 1;
                curr_node = &graph.get(&String::from(curr_node)).unwrap().left;
            }
            'R' => {
                n_steps += 1;
                curr_node = &graph.get(&String::from(curr_node)).unwrap().right;
            }
            _ => unreachable!(),
        }
        instructions.rotate_left(1);
    }
    n_steps
}

fn solve_part_two(mut instructions: Vec<char>, graph: HashMap<String, Edges>) -> i32 {
    let mut n_steps: i32 = 0;
    let mut nodes: Vec<&str> = graph
        .iter()
        .filter(|(k, v)| k.chars().last().unwrap() == 'A')
        .map(|(k, v)| k.as_str())
        .collect();

    while !nodes.iter().all(|n| n.chars().last().unwrap() == 'Z') {
        match instructions.first().unwrap() {
            'L' => {
                n_steps += 1;
                nodes = nodes
                    .clone()
                    .into_iter()
                    .map(|n| graph.get(&String::from(n)).unwrap().left.as_str())
                    .collect();
            }
            'R' => {
                n_steps += 1;
                nodes = nodes
                    .clone()
                    .into_iter()
                    .map(|n| graph.get(&String::from(n)).unwrap().right.as_str())
                    .collect();
            }
            _ => unreachable!(),
        }
        instructions.rotate_left(1);
    }
    n_steps
}
