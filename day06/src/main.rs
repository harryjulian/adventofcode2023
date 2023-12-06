fn solve_part_one() -> i32 {
    let (times, distance) = include_str!("../data/input.txt").split_once('\n').unwrap();
    let times: Vec<i32> = times
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let distance: Vec<i32> = distance
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    times
        .into_iter()
        .zip(distance.into_iter())
        .map(|(time, distance)| {
            (0..=time)
                .scan(0, |t_button_held, n| {
                    *t_button_held += 1;
                    let distance = (time - *t_button_held) * *t_button_held; // distance achieved in race
                    Some(distance)
                })
                .filter(|x: &i32| x > &distance)
                .count() as i32
        })
        .product()
}

fn solve_part_two() -> i64 {
    let (time, distance) = include_str!("../data/input.txt").split_once('\n').unwrap();
    let time: i64 = time
        .split_once(':')
        .unwrap()
        .1
        .replace(" ", "")
        .parse::<i64>()
        .unwrap();
    let distance: i64 = distance
        .split_once(':')
        .unwrap()
        .1
        .replace(" ", "")
        .parse::<i64>()
        .unwrap();
    (0..=time)
        .scan(0, |t_button_held, n| {
            *t_button_held += 1;
            let distance = (time - *t_button_held) * *t_button_held; // distance achieved in race
            Some(distance)
        })
        .filter(|x: &i64| x > &distance)
        .count() as i64
}

fn main() {
    println!("The answer for part one is: {:?}", solve_part_one());
    println!("The answer for part two is: {:?}", solve_part_two());
}
