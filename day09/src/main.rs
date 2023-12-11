fn extrapolate_sequence(sequence: Vec<i32>) -> i32 {
    let mut extrapolations: Vec<Vec<i32>> = vec![sequence];

    // Traverse Sequence to find lowest level
    while !extrapolations.last().unwrap().iter().all(|&x| x == 0) {
        let next_sequence: Vec<i32> = extrapolations
            .last()
            .unwrap()
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect();
        extrapolations.push(next_sequence);
    }
    // Extrapolate to end of sequence
    extrapolations
        .into_iter()
        .rev()
        .fold(0, |state, current| state + current.last().unwrap())
}

fn solve_part_one() -> i32 {
    let puzzle_input: Vec<_> = include_str!("../data/input.txt")
        .split('\n')
        .map(|l| {
            l.split_whitespace()
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    puzzle_input
        .into_iter()
        .map(|s| extrapolate_sequence(s))
        .sum()
}

fn main() {
    println!("The answer to part one is {:?}", solve_part_one());
    //println!("The answer to part two is {:?}", solve_part_two());
}
