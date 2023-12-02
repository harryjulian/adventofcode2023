use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Colour {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Draw {
    colour: Colour,
    n: i32,
}

struct Game {
    game_n: i32,
    draws: Vec<Vec<Draw>>,
}

fn _construct_game(game_str: String) -> Game {
    // Grab game id
    let chars: Vec<char> = game_str.chars().collect();
    let mut p: usize = 5;
    let mut game_id: Vec<char> = vec![];
    while chars[p] != ':' {
        game_id.push(chars[p]);
        p += 1;
    }
    let game_id: i32 = str::parse::<i32>(&String::from_iter(game_id.iter())).unwrap();

    // Remove game id and useless chars
    let mut s: String = chars[p..].iter().collect();
    s = s.replace(&[':', ','][..], "");

    // Split draws
    let draws: Vec<&str> = s.split(";").map(|s| s.trim()).collect();

    let mut result: Vec<Vec<Draw>> = vec![];
    for d in draws.iter() {
        let mut out: Vec<Draw> = vec![];
        let i: Vec<&str> = d.split(" ").collect();
        for j in i.windows(2).step_by(2) {
            let n = str::parse::<i32>(j[0]).unwrap();
            let col = match j[1] {
                "blue" => Colour::Blue,
                "green" => Colour::Green,
                "red" => Colour::Red,
                _ => unreachable!(),
            };
            out.push(Draw { colour: col, n: n })
        }
        result.push(out);
    }
    Game {
        game_n: game_id,
        draws: result,
    }
}

fn _validate_draws(game: &Game, max_draw: &HashMap<Colour, i32>) -> bool {
    for d in game.draws.iter() {
        for s in d.iter() {
            let max_val = max_draw.get(&s.colour).unwrap();
            if s.n > *max_val {
                return false;
            }
        }
    }
    true
}

fn _find_minimum_set_power(game: Game) -> i32 {
    // Create hash map to hold max for each colour
    let mut hm: HashMap<Colour, i32> = HashMap::new();

    // Iterate through each draw to find the max for each colours
    for draw in game.draws.iter() {
        for d in draw.iter() {
            let curr_value = hm.get(&d.colour);
            match curr_value {
                Some(x) => {
                    if d.n > *curr_value.unwrap() {
                        hm.insert(d.colour, d.n);
                        continue;
                    };
                }
                None => {
                    hm.insert(d.colour, d.n);
                }
            }
        }
    }
    hm.values().into_iter().fold(1, |acc, num| acc * num)
}

fn solve_part_one() -> i32 {
    let puzzle_input: Vec<&str> = include_str!("../data/input.txt").split("\n").collect();
    let max_draw: HashMap<Colour, i32> =
        HashMap::from([(Colour::Red, 12), (Colour::Green, 13), (Colour::Blue, 14)]);
    let result: i32 = puzzle_input
        .into_iter()
        .map(|g| _construct_game(String::from(g)))
        .filter(|g| _validate_draws(g, &max_draw))
        .map(|g| g.game_n)
        .sum();
    result
}

fn solve_part_two() -> i32 {
    let puzzle_input: Vec<&str> = include_str!("../data/input.txt").split("\n").collect();
    let result: i32 = puzzle_input
        .into_iter()
        .map(|g| _find_minimum_set_power(_construct_game(String::from(g))))
        .sum();
    result
}

fn main() {
    println!("The answer to part one is {:?}", solve_part_one());
    println!("The answer to part two is {:?}", solve_part_two());
}
