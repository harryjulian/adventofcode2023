use std::collections::HashSet;

fn _validate_tile(
    puzzle_input: &Vec<Vec<char>>,
    idx_i: usize,
    idx_j: usize,
    col_len: usize,
    row_len: usize,
    symbols: &HashSet<char>,
) -> bool {
    let lower_i = if idx_i == 0 { 0 } else { idx_i - 1 };
    let higher_i = if idx_i + 1 < col_len {
        idx_i + 1
    } else {
        col_len - 1
    };
    let lower_j = if idx_j == 0 { 0 } else { idx_j - 1 };
    let higher_j = if idx_j + 1 < row_len {
        idx_j + 1
    } else {
        row_len - 1
    };
    for i in lower_i..=higher_i {
        for j in lower_j..=higher_j {
            match symbols.contains(&puzzle_input[i][j]) {
                false => continue,
                true => return true,
            }
        }
    }
    false
}

fn solve_part_one() -> i32 {
    let puzzle_input: Vec<Vec<char>> = include_str!("../data/input.txt")
        .split("\n")
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();
    let symbols: HashSet<char> = HashSet::from(['%', '&', '/', '#', '*', '$', '=', '@', '-', '+']);
    let (col_len, row_len): (usize, usize) = (puzzle_input.len(), puzzle_input[0].len());
    let (mut stack, mut result): (Vec<char>, i32) = (Vec::new(), 0);
    let mut curr_stack_validated: bool = false;

    // Iterate over cartesian product of indices
    for (idx_i, i) in puzzle_input.iter().enumerate() {
        for (idx_j, j) in i.iter().enumerate() {
            // If the tile is numeric, validate it and add it to the observed number
            if puzzle_input[idx_i][idx_j].is_numeric() {
                stack.push(*j);
                if !curr_stack_validated {
                    curr_stack_validated =
                        _validate_tile(&puzzle_input, idx_i, idx_j, col_len, row_len, &symbols);
                };
            // If not, check if we have a number waiting to be added to the result
            } else if stack.len() > 0 && curr_stack_validated {
                result += str::parse::<i32>(&stack.iter().collect::<String>()).unwrap();
                stack.clear();
                curr_stack_validated = false;
            } else if stack.len() > 0 && !curr_stack_validated {
                stack.clear();
                curr_stack_validated = false;
            }
        }
    }
    result
}

fn main() {
    println!("Part one is: {:?}", solve_part_one());
}
