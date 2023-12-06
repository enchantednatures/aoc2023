#![allow(dead_code)]

use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug)]
struct Almanac<'a> {
    input: &'a str,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy)]
struct Key {
    source: usize,
    dest: usize,
    range: usize,
}

impl Key {
    fn get_value(&self, k: usize) -> usize {
        (self.dest) + (k - self.source) 
    }
}

pub trait GetKeyLocation {
    fn get_value(&self, k: usize) -> usize;
}

impl From<[usize; 3]> for Key {
    fn from(value: [usize; 3]) -> Self {
        Self {
            dest: value[0],
            source: value[1],
            range: value[2],
        }
    }
}

trait InRange {
    fn check(&self, k: usize) -> Option<&Key>;
}

impl InRange for &Vec<Key> {
    fn check(&self, k: usize) -> Option<&Key> {
        self.iter()
            .filter(|f| f.source <= k && k <= f.source + f.range)
            .next()
    }
}

impl GetKeyLocation for &Vec<Key> {
    fn get_value(&self, k: usize) -> usize {
        self.iter()
            .filter(|f| {
                // dbg!(f, k);
                f.source <= k && k <= f.source + f.range
            })
            .next()
            .unwrap_or(&Key {
                source: k,
                dest: k,
                range: 1,
            })
            .get_value(k)
    }
}

impl<'a> Almanac<'a> {
    fn new(input: &str) -> Almanac {
        Almanac { input }
    }

    fn seeds(&self) -> Vec<usize> {
        self.input
            .lines()
            .nth(0)
            .expect("Unable to get first line")
            .split(":")
            .last()
            .expect("couldn't get seeds")
            .split_whitespace()
            .map(|f| f.trim())
            .filter(|f| !f.is_empty())
            .map(|x| x.parse().expect("could not parse to usize"))
            .collect()
    }

    fn build_maps(&self) -> HashMap<(&'a str, &'a str), Vec<Key>> {
        // let lines = self.input.lines();
        let li: HashMap<(&'a str, &'a str), Vec<Key>> = self
            .input
            .split("\n\n")
            .skip(1)
            .map(|l| {
                let (map_type, ranges) =
                    l.split(" map:\n").collect_tuple().expect("unable to split");
                let (source, _, dest) = map_type
                    .split("-")
                    .collect_tuple()
                    .expect("unable to split source/dest");
                // dbg!(map_type, ranges);
                // dbg!(source, dest);

                let keys = ranges.lines().map(|f| {
                    // let r: [&str; 3] = f.splitn(3, " ").collect::<Vec<_>>().into();
                    let mut line_split = f.splitn(3, " ");
                    let r: [usize; 3] =
                        std::array::from_fn(|_| line_split.next().unwrap().parse().unwrap());
                    r
                });
                // keys.map(|k| { (source, dest), Key::from(k), 0) })
                ((source, dest), keys.map(|k| Key::from(k)).collect_vec())
            })
            .collect();
        // dbg!(li);
        // dbg!(&li[&("humidity".to_string(), "location".to_string())]);

        li
    }
}
struct G<'a> {
    adj: Vec<(&'a str, &'a str)>,
}
impl<'a> G<'a> {
    fn paths(start: &'a str, end: &'a str) -> Vec<Vec<&'a str>> {
        todo!()
    }
}

pub fn solve_part_1(input: &str) -> usize {
    let almanac = Almanac { input };
    let maps = almanac.build_maps();
    let path = vec![
        "seed",
        "soil",
        "fertilizer",
        "water",
        "light",
        "temperature",
        "humidity",
        "location",
    ];

    let mut min_loc = usize::MAX;
    for seed in almanac.seeds() {
        let mut s = 0;
        let mut d = 1;
        let mut next_key = Some((path[s], path[d]));
        let mut value = seed;
        while let Some(key) = next_key {
            let m = maps.get(&key);
            dbg!(seed, m, key, value);
            value = m.unwrap_or(&vec![]).get_value(value);
            s += 1;
            d += 1;

            next_key = None;
            if d < path.len() {
                next_key = Some((path[s], path[d]));
            }
            if key.1 == "location" {
                min_loc = min_loc.min(value);
            }
        }
    }

    min_loc
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
        assert_eq!(35, solve_part_1(input));
    }

    #[test]
    fn part_1_real() {
        let input = include_str!("./part1.txt");
        assert_eq!(2683, solve_part_1(input));
    }

    #[test]
    fn get_default_value() {
        let keys = &vec![];
        assert_eq!(13, keys.get_value(13));
    }

    #[test]
    fn get_value() {
        let keys = &vec![
            Key {
                source: 0,
                dest: 0,
                range: 1,
            },
            Key {
                source: 2,
                dest: 10,
                range: 5,
            },
        ];
        assert_eq!(0, keys.get_value(0));
        assert_eq!(11, keys.get_value(3));
    }

    #[test]
    fn get_test_data_value() {
        let keys = &vec![
        Key {
            source: 15,
            dest: 0,
            range: 37,
        },
        Key {
            source: 52,
            dest: 37,
            range: 2,
        },
        Key {
            source: 0,
            dest: 39,
            range: 15,
        },
    ];
        assert_eq!(52, keys.get_value(13));
    }

    #[test]
    fn key_get_value () {
        let key = Key {
            source: 2,
            dest: 10,
            range: 5,
        };
        assert_eq!(10, key.get_value(2));
        assert_eq!(11, key.get_value(3));
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
