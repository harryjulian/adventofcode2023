use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Scratchcard {
    game: usize,
    winners: HashSet<i32>,
    numbers: HashSet<i32>,
}

impl Scratchcard {
    fn score(&self) -> i32 {
        match self.n_winners() as i32 {
            0 => return 0,
            x => return (0..x - 1).fold(1, |acc, _| acc * 2),
            _ => unreachable!(),
        }
    }

    fn n_winners(&self) -> i32 {
        self.winners.intersection(&self.numbers).count() as i32
    }
}

fn parser() -> Vec<Scratchcard> {
    let pattern = Regex::new("[0-9] |").unwrap();
    let number_pattern = Regex::new("[0-9]+").unwrap();
    let mut game_n: usize = 1;
    let puzzle_input = include_str!("../data/input.txt")
        .split('\n')
        .map(|x| x.split(':').collect::<Vec<&str>>())
        .map(|x| {
            x.last()
                .unwrap()
                .chars()
                .filter(|c| pattern.is_match(&c.to_string()))
                .collect::<String>()
        })
        .map(|x| {
            let mut s = x.split('|');
            let winners = HashSet::from_iter(
                number_pattern
                    .find_iter(s.next().unwrap())
                    .filter_map(|digits| digits.as_str().parse().ok()),
            );
            let numbers = HashSet::from_iter(
                number_pattern
                    .find_iter(s.next().unwrap())
                    .filter_map(|digits| digits.as_str().parse().ok()),
            );
            let scratchcard = Scratchcard {
                game: game_n,
                winners: winners,
                numbers: numbers,
            };
            game_n += 1;
            scratchcard
        })
        .collect();
    puzzle_input
}

fn solve_part_one() -> i32 {
    parser().into_iter().map(|sc| sc.score()).sum()
}

fn solve_part_two() -> i32 {
    let scratchcards_index = parser();
    let mut scratchcards_queue = scratchcards_index.clone();
    let mut idx_evaluated: usize = 0;
    while idx_evaluated < scratchcards_queue.len() {
        let sc = &scratchcards_queue[idx_evaluated];
        let mut cards_to_add: Vec<Scratchcard> = (0..sc.n_winners())
            .map(|idx| {
                scratchcards_index
                    .iter()
                    .nth(sc.game + idx as usize)
                    .unwrap()
            })
            .map(|x| x.clone())
            .collect();
        scratchcards_queue.append(&mut cards_to_add);
        idx_evaluated += 1;
    }
    scratchcards_queue.len() as i32
}

fn main() {
    println!("The answer to part one is: {:?}", solve_part_one());
    println!("The answer to part two is: {:?}", solve_part_two());
}
