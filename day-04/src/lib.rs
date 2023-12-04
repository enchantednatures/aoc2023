#![allow(dead_code)]

use std::collections::HashSet;

#[derive(Debug)]
struct Game {
    number: usize,
    winning_numbers: Vec<usize>,
    my_numbers: Vec<usize>,
}

pub fn solve_part_1(input: &str) -> usize {
    input.lines().map(|f| {
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

    }).sum()
}

pub fn solve_part_2(input: &str) -> usize {
    todo!()
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

    // #[test]
    // fn part_2_example() {
    //     let input = include_str!("./part1.test.txt");
    //     assert_eq!(2286, solve2(input) );
    // }

    // #[test]
    // fn part_2_real() {
    //     let input = include_str!("./part1.txt");
    //     assert_eq!(49710, solve2(input) );
    // }
}
