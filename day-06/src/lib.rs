#![allow(dead_code)]

use itertools::Itertools;

#[derive(Debug)]
struct Race {
    time: usize,
    record: usize,
}

impl Race {
    fn new(time: usize, record: usize) -> Self {
        Self { time, record }
    }

    fn min_time_held_to_win(&self) -> usize {
        let mpms = self.record / self.time;
        // let min_time_held = self.time - (mpms + (self.time - mpms));
        let mut min = 0;
        for t in 1..self.time {
            if (t * (self.time - t)) > self.record {
                return t;
            }
        }
        0
    }

    fn max_time_held_to_win(&self) -> usize {
        let mpms = self.record / self.time;
        // let min_time_held = self.time - (mpms + (self.time - mpms));
        let mut min = 0;
        for t in (1..self.time).rev() {
            if (t * (self.time - t)) > self.record {
                return t;
            }
        }
        self.time
    }

    fn solve(&self) -> usize {
        // (self.min_time_held_to_win()..=self.max_time_held_to_win()).count()
        self.max_time_held_to_win() - self.min_time_held_to_win() + 1
    }
}

pub fn solve_part_1<'a>(input: &'a str) -> usize {
    let (times, distances) = input.splitn(2, "\n").collect_tuple().unwrap();
    // dbg!(times, distances);
    let parse = move |input: &'a str| {
        input
            .splitn(2, ":")
            .last()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|f| f.trim().parse::<usize>().unwrap())
    };

    parse(times)
        .zip_eq(parse(&distances))
        .map(|(t, d)| {
            let race = Race::new(t, d);
            dbg!(
                &race,
                &race.min_time_held_to_win(),
                &race.max_time_held_to_win()
            );
            race.solve()
            // dbg!(race.min_time_held_to_win());
        })
        .product()
}

pub fn solve_part_2<'a>(input: &'a str) -> usize {
    let (times, distances) = input.splitn(2, "\n").collect_tuple().unwrap();
    // dbg!(times, distances);
    let parse = move |input: &'a str| {
        input
            .splitn(2, ":")
            .last()
            .unwrap()
            .trim()
            .replace(" ", "")
            .parse::<usize>()
            .unwrap()
    };

    let race = Race::new(parse(times), parse(distances));
    dbg!(
        &race,
        &race.min_time_held_to_win(),
        &race.max_time_held_to_win()
    );
    race.solve()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let input = include_str!("./part1.test.txt");
        assert_eq!(288, solve_part_1(input));
    }

    #[test]
    fn part_1_real() {
        let input = include_str!("./part1.txt");
        assert_eq!(2065338, solve_part_1(input));
    }

    #[test]
    fn part_2_example() {
        let input = include_str!("./part1.test.txt");
        assert_eq!(71503, solve_part_2(input));
    }

    #[test]
    fn part_2_real() {
        let input = include_str!("./part1.txt");
        assert_eq!(2286, solve_part_2(input));
}
}
