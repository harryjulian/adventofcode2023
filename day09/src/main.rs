fn extrapolate_sequence(sequence: Vec<i32>) -> (i32, i32) {
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
    let forwards = extrapolations
        .iter()
        .rev()
        .fold(0, |state, current| state + current.last().unwrap());

    // Extrapolate to the beginning of the sequence
    let backwards = extrapolations
        .iter()
        .rev()
        .fold(0, |state, current| current.first().unwrap() - state);

    // Return tuple for both answers
    (forwards, backwards)
}

fn main() {
    let puzzle_input: Vec<_> = include_str!("../data/input.txt")
        .split('\n')
        .map(|l| {
            l.split_whitespace()
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    let sequences: Vec<(i32, i32)> = puzzle_input
        .into_iter()
        .map(|s| extrapolate_sequence(s))
        .collect();
    let p1: i32 = sequences.iter().map(|x| x.0).sum();
    let p2: i32 = sequences.iter().map(|x| x.1).sum();
    println!("The answer to part one is: {:?}", p1);
    println!("The answer to part two is: {:?}", p2);
}
