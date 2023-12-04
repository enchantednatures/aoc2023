#![allow(dead_code)]

#[derive(Debug)]
struct Game {
    number: usize,
    winning_numbers: Vec<usize>,
    my_numbers: Vec<usize>,
}

pub fn solve_part_1(input: &str) -> usize {
    input
        .lines()
        .map(|f| {
            let game_name_idx = f.find(':').expect("Invalid Game input");
            let game_del = f.find('|').expect("Invalid Game input");
            let mut game_number: usize = 0;
            for (idx, c) in f[..game_name_idx].char_indices() {
                if c.is_numeric() {
                    game_number = f[idx..game_name_idx]
                        .parse()
                        .expect("Could not parse game number")
                }
            }
            dbg!(game_number);

            let scores = f[game_name_idx..game_del]
                .split_whitespace()
                .map(|f| f.trim())
                .filter(|f| f.is_empty());

            dbg!(scores);

            0
        })
        .sum()
}

pub fn solve_part_2(_input: &str) -> usize {
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
        let _input = include_str!("./part1.txt");
        // assert_eq!(2683, solve(input, 12,13,14));
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
