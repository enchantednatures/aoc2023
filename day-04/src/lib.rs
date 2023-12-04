#![allow(dead_code)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use itertools::Itertools;

pub fn solve_part_1(input: &str) -> usize {
    input
        .lines()
        .map(|f| {
            let game_name_idx = f.find(':').expect("Invalid Game input");
            let game_del = f.find('|').expect("Invalid Game input");
            let mut game_number: usize = 0;
            dbg!(&game_name_idx, &game_del);
            for (idx, c) in f[..game_name_idx].char_indices() {
                if c.is_numeric() {
                    game_number = f[idx..game_name_idx]
                        .parse()
                        .expect("Could not parse game number")
                }
            }
            dbg!(game_number);
            dbg!(&f[game_name_idx + 1..game_del]);

            let scores: Vec<usize> = f[game_name_idx + 1..game_del]
                .split_whitespace()
                .map(|f| f.trim())
                .filter(|f| !f.is_empty())
                .map(|x| x.parse().unwrap())
                .collect();

            dbg!(&f[game_del + 1..f.len()]);
            let winning: HashSet<usize> = f[game_del + 1..f.len()]
                .split_whitespace()
                .map(|f| f.trim())
                .filter(|f| !f.is_empty())
                .map(|x| x.parse().unwrap())
                .collect();
            dbg!(&winning, &scores);

            return scores
                .iter()
                .filter(|f| winning.contains(f))
                .fold(0, |mut acc, x| {
                    acc = acc * 2;
                    if acc == 0 {
                        acc = 1;
                    }
                    acc
                });
        })
        .sum()
}

#[derive(Debug, Clone)]
struct Game {
    number: usize,
    overlap: usize,
}

pub fn solve_part_2(input: &str) -> usize {
    let mut games: GameLookup = GameLookup::with_capacity(300);
    let _: Vec<()> = input
        .lines()
        .map(|f| {
            let game_name_idx = f.find(':').expect("Invalid Game input");
            let game_del = f.find('|').expect("Invalid Game input");
            let mut number: usize = 0;
            let idx = f
                .find(|c: char| c.is_numeric())
                .expect("Could not find game number");
            number = f[idx..game_name_idx]
                .parse()
                .expect("Could not parse game number");

            let my_numbers: HashSet<usize> = f[game_name_idx + 1..game_del]
                .split_whitespace()
                .map(|f| f.trim())
                .filter(|f| !f.is_empty())
                .map(|x| x.parse().unwrap())
                .collect();

            let winning_numbers: HashSet<usize> = f[game_del + 1..f.len()]
                .split_whitespace()
                .map(|f| f.trim())
                .filter(|f| !f.is_empty())
                .map(|x| x.parse().unwrap())
                .collect();

            let overlap = winning_numbers.intersection(&my_numbers);
            let _ = games.insert(
                number,
                Game {
                    number,
                    overlap: overlap.collect_vec().len(),
                },
            );
        })
        .collect();
    let mut queue = GameQueue::new();
    let mut played = GamesPlayed::new();
    // queue.push_front(&games[&1]);
    for g in games.values() {
        queue.push_back(g);
        played.insert(g.number, 0);
    }

    println!("solving time");
    // solve(&games[&1], &games, &mut queue, &mut played);
    solve_outer(&games, &mut queue, &mut played);
    played.iter().map(|(_, y)| y).sum()
}
type GameLookup = HashMap<usize, Game>;
type GamesPlayed = HashMap<usize, usize>;
type GameQueue<'a> = VecDeque<&'a Game>;

fn solve_outer<'a>(games: &'a GameLookup, queue: &'a mut GameQueue<'a>, played: &mut GamesPlayed) {
    while let Some(game) = queue.pop_front() {
        played
            .entry(game.number)
            .and_modify(|f| *f += 1)
            .or_insert(1);

        for idx in 1..=game.overlap {
            let next_game = idx + game.number;
            let g = games.get(&next_game);
            if let Some(g) = g {
                println!("pushing {}", g.number);
                queue.push_back(g);
                // let t: HashMap<usize, usize> = &queue.iter().map(|f| (f.number, 1)).collect();
            }
        }
    }
}

fn solve<'a>(
    game: &Game,
    games: &'a GameLookup,
    queue: &'a mut GameQueue<'a>,
    played: &mut GamesPlayed,
) {
    // dbg!(&queue);
    played
        .entry(game.number)
        .and_modify(|f| *f += 1)
        .or_insert(1);

    let overlap = game.overlap;

    // dbg!(&overlap);
    for idx in 1..=overlap {
        let next_game = idx + game.number + 1;
        dbg!(&idx, &next_game, &game);
        let g = games.get(&next_game);
        if let Some(g) = g {
            queue.push_back(g);
            dbg!(&queue);
        }
    }
    if let Some(next_game) = queue.pop_front() {
        solve(next_game, games, queue, played);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let input = include_str!("./part1.test.txt");
        assert_eq!(13, solve_part_1(input));
    }

    #[test]
    fn part_1_real() {
        let input = include_str!("./part1.txt");
        assert_eq!(23441, solve_part_1(input));
    }

    #[test]
    fn part_2_example() {
        let input = include_str!("./part1.test.txt");
        assert_eq!(30, solve_part_2(input));
    }

    #[test]
    fn part_2_real() {
        let input = include_str!("./part1.txt");
        assert_eq!(5923918, solve_part_2(input));
    }
}
