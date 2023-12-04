#![allow(dead_code)]

use itertools::Itertools;

#[derive(Debug)]
struct Game {
    red: usize,
    green: usize,
    blue: usize,
}
impl Game {
    fn new(red: usize, green: usize, blue: usize) -> Game {
        Game { red, green, blue }
    }
}

fn parse_roll< 'a, T>(dice: T, color: &str) -> usize
where
    T: Iterator<Item = &'a &'a str>,
{
    dice.filter(|x| x.contains(color))
        .map(|d| d.trim().split(' ').next().unwrap().parse::<usize>().unwrap())
        .sum::<usize>()
}
fn solve(input: &str, red_dice: usize, green_dice: usize, blue_dice: usize) -> i32 {
    input
        .lines()
        .map(|l| {
            let parts: Vec<_> = l.split(':').collect();
            let game = parts[0].split(' ').last().unwrap().parse::<i32>().unwrap();
            let rolls: Vec<_> = parts[1].split(';').collect();

            let game_results= rolls
                .iter()
                .map(|r| {
                    dbg!(r);
                    let dice: Vec<_> = r.split(',').collect();
                    dbg!(&dice);
                    let red = parse_roll(dice.iter(), "red");
                    let green = parse_roll(dice.iter(), "green");
                    let blue = parse_roll(dice.iter(), "blue");
                    Game::new(red, green, blue)
                })
                   .all(|x| x.red <= red_dice && x.green <= green_dice && x.blue <= blue_dice) ;
            if game_results {
                game
            } else {
                0
            }
            
        })
        .sum()
}


fn solve2(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let parts: Vec<_> = l.split(':').collect();
            let rolls: Vec<_> = parts[1].split(';').collect();
            let game_results= rolls
                .iter()
                .map(|r| {
                    let dice: Vec<_> = r.split(',').collect();
                    let red = parse_roll(dice.iter(), "red");
                    let green = parse_roll(dice.iter(), "green");
                    let blue = parse_roll(dice.iter(), "blue");
                    Game::new(red, green, blue)
                }).collect_vec();
            let red_results = game_results.iter().map(|g| g.red).max_by(|x, y| x.cmp(y)).unwrap_or(0);
            let green_results = game_results.iter().map(|g| g.green).max_by(|x, y| x.cmp(y)).unwrap_or(0);
            let blue_results = game_results.iter().map(|g| g.blue).max_by(|x, y| x.cmp(y)).unwrap_or(0);
            red_results * green_results * blue_results 
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = include_str!("./part1.test.txt");
        assert_eq!(8, solve(input, 12,13,14));
    }

    #[test]
    fn real() {
        let input = include_str!("./part1.txt");
        assert_eq!(2683, solve(input, 12,13,14));
    }


    #[test]
    fn example_part2() {
        let input = include_str!("./part1.test.txt");
        assert_eq!(2286, solve2(input) );
    }

    #[test]
    fn real_part2() {
        let input = include_str!("./part1.txt");
        assert_eq!(49710, solve2(input) );
    }
}
