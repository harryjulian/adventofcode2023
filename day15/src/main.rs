use std::collections::HashMap;

fn hash(s: &Vec<i32>) -> i32 {
    s.iter().fold(0, |state, val| ((state + val) * 17) % 256)
}

fn main() {
    let puzzle_input: Vec<_> = include_str!("../data/test_input.txt").split(',').collect();

    // Solve part one
    let p1: i32 = puzzle_input
        .iter()
        .map(|s| {
            let s = s
                .chars()
                .map(|c| c.to_ascii_lowercase() as i32)
                .collect::<Vec<i32>>();
            hash(&s)
        })
        .sum();
    dbg!(&p1);
}
