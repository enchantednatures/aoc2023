#![allow(dead_code)]

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct PartNumber {
    line: usize,
    value: usize,
    start_index: usize,
    end_index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Gear {
    line: usize,
    idx: usize,
}

fn get_positions_touching_number(number: &PartNumber) -> impl IntoIterator<Item = (usize, usize)> {
    let mut pos: Vec<(usize, usize)> = Vec::new();
    if number.start_index > 0 {
        pos.push((number.line, number.start_index - 1));
        pos.push((number.line + 1, number.start_index - 1));
    }

    if number.line > 0 {
        pos.push((number.line - 1, number.start_index));
        pos.push((number.line - 1, number.end_index + 1));
    }
    if number.line > 0 && number.start_index > 0 {
        pos.push((number.line - 1, number.start_index - 1));
    }

    pos.push((number.line, number.end_index + 1));
    pos.push((number.line + 1, number.end_index + 1));
    for x in number.start_index..=number.end_index {
        pos.push((number.line + 1, x));
        // pos.push((number.line + 1, x+1));
        if number.line > 0 {
            pos.push((number.line - 1, x));
            // pos.push((number.line - 1, x+1));
        }
    }
    pos
}

fn is_valid(input: &str, number: &PartNumber) -> bool {
    let mut pos: Vec<(usize, usize)> = Vec::new();
    if number.start_index > 0 {
        pos.push((number.line, number.start_index - 1));
        pos.push((number.line + 1, number.start_index - 1));
    }

    if number.line > 0 {
        pos.push((number.line - 1, number.start_index));
        pos.push((number.line - 1, number.end_index + 1));
    }
    if number.line > 0 && number.start_index > 0 {
        pos.push((number.line - 1, number.start_index - 1));
    }

    pos.push((number.line, number.end_index + 1));
    pos.push((number.line + 1, number.end_index + 1));
    for x in number.start_index..=number.end_index {
        pos.push((number.line + 1, x));
        // pos.push((number.line + 1, x+1));
        if number.line > 0 {
            pos.push((number.line - 1, x));
            // pos.push((number.line - 1, x+1));
        }
    }
    // dbg!(&pos, &number);
    let is_num = pos.iter().any(|(line, idx)| {
        if let Some(l) = input.lines().nth(*line) {
            let nth = l.chars().nth(*idx);
            if let Some(char_at_pos) = nth {
                if !char_at_pos.is_numeric() && char_at_pos != '.' {
                    // dbg!("not a number", number, char_at_pos, line, idx);
                    return true;
                } else if !char_at_pos.is_numeric() && char_at_pos == '.' {
                    return false;
                }
            }
        }
        // dbg!("a number", number, line, idx);
        false
    });
    dbg!(is_num, number);
    is_num
}

fn is_gear_ratio(gear: &Gear, part: &PartNumber) -> bool {
    for (line, idx) in get_positions_touching_number(&part) {
        if line == gear.line && idx == gear.idx {
            return true;
        }
    }
    false
}

fn get_gear_ratios_t<'a, G, P>(gears: &[&'a Gear], numbers: &[&'a PartNumber]) -> usize {
    gears
        .iter()
        .map(|gear| {
            let mut ratios = Vec::new();
            let numbers_touching_gear = numbers
                .into_iter()
                .filter(|p| is_gear_ratio(gear, *p))
                .collect_vec();
            if numbers_touching_gear.len() == 2 {
                ratios.push(numbers_touching_gear.iter().fold(0, |a, b| a * b.value));
            }
            return ratios;
        })
        .flatten()
        .sum()
}

fn get_gear_ratios(gears: Vec<Gear>, numbers: Vec<PartNumber>) -> usize
{
    let mut ratios = Vec::new();
    for gear in gears {
        let numbers_touching_gear = numbers
            .iter()
            .filter(|p| is_gear_ratio(&gear, *p))
            .collect_vec();
        if numbers_touching_gear.len() == 2 {
            ratios.push(numbers_touching_gear.iter().fold(1, |a, b| a * b.value));
        }
    }
    ratios.iter().sum()
}

fn extract_gears(input: &str) -> impl Iterator<Item = Gear> + '_ {
    input
        .lines()
        .enumerate()
        .map(|(line, x)| {
            let mut gears = Vec::new();
            for (idx, c) in x.chars().enumerate() {
                if c == '*' {
                    let mut gear = Gear { line, idx };
                    gears.push(gear);
                }
            }
            gears
        })
        .flatten()
}

fn extract_numbers(input: &str) -> impl Iterator<Item = PartNumber> + '_ {
    input
        .lines()
        .enumerate()
        .map(|(line, x)| {
            let mut numbers = Vec::new();
            let mut s = String::new();
            let mut start_index = 0;
            for (cidx, c) in x.chars().enumerate() {
                if c.is_numeric() {
                    if s.len() == 0 {
                        start_index = cidx;
                    }
                    s.push(c);
                } else {
                    if s.len() > 0 {
                        let value = s.parse::<usize>().unwrap();
                        numbers.push(PartNumber {
                            line,
                            value,
                            start_index,
                            end_index: cidx - 1,
                        });
                        s = String::new();
                    }
                }
            }
            if s.len() > 0 {
                let value = s.parse::<usize>().unwrap();
                numbers.push(PartNumber {
                    line,
                    value,
                    start_index,
                    end_index: x.len() - 1,
                });
            }
            return numbers;
        })
        .flatten()
}

pub fn solve_part_1(input: &str) -> usize {
    extract_numbers(input)
        .filter(|x| is_valid(input, x))
        .map(|f| f.value)
        .sum()
}

pub fn solve_part_2(input: &str) -> usize {
    let gears = extract_gears(input);
    let numbers = extract_numbers(input);

    get_gear_ratios(gears.collect_vec(), numbers.collect_vec())
    
}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;

    use super::*;

    #[test]
    fn part_1_example() {
        let input = include_str!("./part1.test.txt");
        assert_eq!(4361, solve_part_1(input));
    }

    #[test]
    fn part_1_real() {
        let input = include_str!("./part1.txt");
        assert_eq!(539637, solve_part_1(input));
    }

    #[test]
    fn is_valid_when_symbol_adjacent() {
        let input = "....\n.23*.\n....";
        assert!(is_valid(
            input,
            &PartNumber {
                line: 1,
                value: 23,
                start_index: 1,
                end_index: 2,
            },
        ));
    }

    #[test]
    fn is_valid_when_symbol_diagonal_up_right() {
        let input = "..#.\n.23..\n....";
        assert!(is_valid(
            input,
            &PartNumber {
                line: 1,
                value: 23,
                start_index: 1,
                end_index: 2,
            },
        ));
    }

    #[test]
    fn extract_numbers_works() {
        let input = "#$&+!\n.....\n!.23.$\n.....\n!$&+!";
        let numbers = extract_numbers(input).collect::<Vec<_>>();
        assert_eq!(numbers.len(), 1);
        assert_eq!(
            **&numbers.first().unwrap(),
            PartNumber {
                line: 2,
                value: 23,
                start_index: 2,
                end_index: 3,
            }
        )
    }

    #[test]
    fn extract_numbers_works_at_start_of_line() {
        let input = "#$&..+!\n.......\n12....43.$\n.......\n!..$&+!";
        let numbers = extract_numbers(input).collect::<Vec<_>>();
        // assert_eq!(numbers.len(), 1);
        dbg!(&numbers);
        // assert!(false)
        assert_eq!(
            **&numbers.first().unwrap(),
            PartNumber {
                line: 2,
                value: 12,
                start_index: 0,
                end_index: 1,
            }
        )
    }

    #[test]
    fn extract_numbers_works_with_multiple_on_one_line() {
        let input = "#$&..+!\n.......\n!.23..43.$\n.......\n!..$&+!";
        let numbers = extract_numbers(input).collect::<Vec<_>>();
        // assert_eq!(numbers.len(), 1);
        dbg!(&numbers);
        // assert!(false)
        assert_equal(
            numbers,
            vec![
                PartNumber {
                    line: 2,
                    value: 23,
                    start_index: 2,
                    end_index: 3,
                },
                PartNumber {
                    line: 2,
                    value: 43,
                    start_index: 6,
                    end_index: 7,
                },
            ],
        )
    }

    #[test]
    fn is_valid_when_symbol_diagonal_down_right() {
        let input = "....\n.23..\n..#.";
        assert!(is_valid(
            input,
            &PartNumber {
                line: 1,
                value: 23,
                start_index: 1,
                end_index: 2,
            },
        ));
    }

    #[test]
    fn is_not_valid_when_symbols_not_adjacent() {
        let input = "#$&+!\n.....\n!.23.$\n.....\n!$&+!";
        assert!(!is_valid(
            input,
            &PartNumber {
                line: 2,
                value: 23,
                start_index: 2,
                end_index: 3,
            },
        ));
    }

    #[test]
    fn is_not_valid_when_symbols_not_adjacent_multiple() {
        let input = "#$&+!.\n......\n.35909\n......\n!.23.$\n......\n.....!\n!$&+!)\n";
        assert!(!is_valid(
            input,
            &PartNumber {
                line: 2,
                value: 35909,
                start_index: 1,
                end_index: 5,
            },
        ));
    }

    #[test]
    fn is_not_valid_when_symbols_not_adjacent_multiple_on_start_of_linj() {
        let input = "#$&+!.\n......\n.35909\n......\n!.23.$\n......\n.....!\n!$&+!)\n";
        assert!(!is_valid(
            &input,
            &PartNumber {
                line: 4,
                value: 23,
                start_index: 0,
                end_index: 1,
            },
        ));
    }

    #[test]
    fn extract_numbers_multiple() {
        let input = "#$&+!.\n......\n.35909\n......\n617*..\n......\n.....!\n!$&+!)";
        let numbers = extract_numbers(&input).collect::<Vec<_>>();
        assert_equal(
            numbers,
            vec![
                PartNumber {
                    line: 2,
                    value: 35909,
                    start_index: 1,
                    end_index: 5,
                },
                PartNumber {
                    line: 4,
                    value: 617,
                    start_index: 0,
                    end_index: 2,
                },
            ],
        );
    }

    #[test]
    fn is_valid_when_at_start_of_line_with_adjacent_symbol() {
        let input = "#$&+!.\n......\n.35909\n......\n617*..\n......\n.....!\n!$&+!)";
        assert!(is_valid(
            &input,
            &PartNumber {
                line: 4,
                value: 617,
                start_index: 0,
                end_index: 2,
            },
        ));
    }


    #[test]
    fn part_2_example() {
        let input = include_str!("./part1.test.txt");
        assert_eq!(467835, solve_part_2(input));
    }

    #[test]
    fn part_2_real() {
        let input = include_str!("./part1.txt");
        assert_eq!(49710, solve_part_2(input) );
    }
}
