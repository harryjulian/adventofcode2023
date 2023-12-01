fn _find_digits(s: &str) -> Vec<i32> {
    let c: Vec<char> = s.chars().collect();
    let mut result = Vec::with_capacity(2);
    for i in c.iter() {
        if i.is_numeric() {
            result.push(i.to_digit(10).unwrap() as i32);
            break;
        }
    }
    for i in c.iter().rev() {
        if i.is_numeric() {
            result.push(i.to_digit(10).unwrap() as i32);
            break;
        }
    }
    result
}

fn _find_digits_or_words(s: &str) {}

fn solve_part_one() -> i32 {
    let input: Vec<Vec<i32>> = include_str!("input.txt")
        .split("\n")
        .into_iter()
        .map(|s| _find_digits(s))
        .collect();
    input.iter().map(|x| (x[0] * 10) + x[1]).sum()
}

fn solve_part_two() -> i32 {
    let input: Vec<Vec<i32>> = include_str!("input.txt")
        .split("\n")
        .into_iter()
        .map(|s| _find_digits_or_words(s))
        .collect();
    input.iter().map(|x| (x[0] * 10) + x[1]).sum()
}

fn main() {
    let part: i32 = str::parse::<i32>(&std::env::args().nth(1).expect("no pattern given")).unwrap();
    match part {
        1 => println!("The answer to part 1 is: {:?}", solve_part_one()),
        2 => println!("The answer to part 2 is: {:?}", solve_part_two()),
    }
}
