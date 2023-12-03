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

fn _find_digits_and_words(s: &str, words: &Vec<&str>) -> Vec<i32> {
    let mut digits: Vec<i32> = vec![];
    for idx in 0..s.len() {
        let v = s.chars().nth(idx).unwrap();
        if v.is_numeric() {
            digits.push(v.to_digit(10).unwrap() as i32);
            continue;
        }
        for (k, v) in words.iter().enumerate() {
            if s[idx..].starts_with(v) {
                digits.push(k as i32 + 1);
                continue;
            }
        }
    }
    vec![*digits.first().unwrap(), *digits.last().unwrap()]
}

fn solve_part_one() -> i32 {
    include_str!("../data/input.txt")
        .split("\n")
        .into_iter()
        .map(|s| {
            let digits = _find_digits(s);
            (digits[0] * 10) + digits[1]
        })
        .sum()
}

fn solve_part_two() -> i32 {
    let words: Vec<&str> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    include_str!("../data/input.txt")
        .split("\n")
        .into_iter()
        .map(|s| {
            let digits = _find_digits_and_words(s, &words);
            (digits[0] * 10) + digits[1]
        })
        .sum()
}

fn main() {
    println!("The answer to part 1 is: {:?}", solve_part_one());
    println!("The answer to part 2 is: {:?}", solve_part_two());
}
