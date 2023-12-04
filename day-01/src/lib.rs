use std::fs;

const ALL_DIGITS: [&str; 18] = [
    "one",
    "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3",
    "4", "5", "6", "7", "8", "9",
];

enum Digits {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl From<&str> for Digits {
    fn from(value: &str) -> Self {
        match value {
            "one" => Digits::One,
            "1" => Digits::One,
            "two" => Digits::Two,
            "2" => Digits::Two,
            "three" => Digits::Three,
            "3" => Digits::Three,
            "four" => Digits::Four,
            "4" => Digits::Four,
            "five" => Digits::Five,
            "5" => Digits::Five,
            "six" => Digits::Six,
            "6" => Digits::Six,
            "seven" => Digits::Seven,
            "7" => Digits::Seven,
            "eight" => Digits::Eight,
            "8" => Digits::Eight,
            "nine" => Digits::Nine,
            "9" => Digits::Nine,
            _ => panic!("Unknown digit"),
        }
    }
}

impl Digits {
    fn to_num(&self) -> usize {
        match self {
            Digits::One => 1,
            Digits::Two => 2,
            Digits::Three => 3,
            Digits::Four => 4,
            Digits::Five => 5,
            Digits::Six => 6,
            Digits::Seven => 7,
            Digits::Eight => 8,
            Digits::Nine => 9,
        }
    }

    fn find_first_in_line(line: &str) -> Self {
        let mut first_idx = usize::MAX;
        let mut first_digit = None;
        for digit in ALL_DIGITS.iter() {
            if let Some(idx) = line.find(digit) {
                if idx <= first_idx {
                    first_idx = idx;
                    first_digit = Some(digit);
                }
            }
        }
        if let Some(digit) = first_digit {
            Digits::from(*digit)
        } else {
            panic!("No digit found in line");
        }
    }
    fn find_last_in_line(line: &str) -> Self {
        let mut last_idx = usize::MIN;
        let mut last_digit = None;
        for digit in ALL_DIGITS.iter() {
            let idx = line.rfind(*digit);
            if let Some(idx) = idx {
                if idx >= last_idx {
                    last_idx = idx;
                    last_digit = Some(*digit);
                }
            }
        }
        if let Some(digit) = last_digit {
            Digits::from(digit)
        } else {
            panic!("No digit found in line");
        }
    }
}

#[derive(Debug)]
struct Input {
    data: String,
}

impl Input {
    fn new(day: u8, env: &str) -> Self {
        let filename = format!("files/day{:02}.{}.txt", day, env);
        dbg!(&filename);
        let data = fs::read_to_string(filename).expect("Unable to read file");
        Self { data }
    }
}

fn day1(input: Input) -> usize {
    let mut sum = 0;
    for line in input.data.lines() {
        let first = line.chars().find(|c| c.is_numeric()).unwrap();
        let last = line.chars().rev().find(|c| c.is_numeric()).unwrap();

        let arr = [first, last];
        let num: String = arr.iter().collect();
        let num: usize = num.parse().unwrap();
        sum += num;
    }
    println!("sum: {}", sum);
    sum
}


impl Input {
    fn solve(&self) -> usize {
        return self
            .data
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let first = Digits::find_first_in_line(line).to_num();
                let last = Digits::find_last_in_line(line).to_num();

                

                // dbg!(line, first, last, num);
                10 * first + last
            })
            .sum();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_digit_at_start() {
        assert_eq!(Digits::find_first_in_line("4abctwothree").to_num(), 4);
        assert_eq!(Digits::find_first_in_line("5abctwothree").to_num(), 5);
    }

    #[test]
    fn spelled_digit_at_start() {
        assert_eq!(Digits::find_first_in_line("one234five").to_num(), 1);
    }

    #[test]
    fn both_numeric_and_spelled_digits() {
        assert_eq!(Digits::find_first_in_line("abc3deftwoghi").to_num(), 3);
        assert_eq!(Digits::find_first_in_line("four5six7").to_num(), 4);
        assert_eq!(Digits::find_first_in_line("one5six7").to_num(), 1);
        assert_eq!(Digits::find_first_in_line("sastwo5six7").to_num(), 2);
        assert_eq!(Digits::find_first_in_line("slkfsthreefour5six7").to_num(), 3);
        assert_eq!(Digits::find_first_in_line("akljsfdfivefive5six7").to_num(), 5);
        assert_eq!(Digits::find_first_in_line("slkdfsevenakljsfdfivefive5six7").to_num(), 7);
    }

    #[test]
    fn mixed_characters() {
        assert_eq!(Digits::find_first_in_line("xyz9abceightdef2ghi").to_num(), 9);
        assert_eq!(Digits::find_first_in_line("sevenabc1xyz").to_num(), 7);
    }

    #[test]
    fn numeric_digit_at_end() {
        assert_eq!(Digits::find_last_in_line("abctwothree4").to_num(), 4);
    }

    #[test]
    fn spelled_digit_at_end() {
        assert_eq!(Digits::find_last_in_line("234fiveone").to_num(), 1);
    }


    #[test]
    fn last_both_numeric_digits() {
        assert_eq!(Digits::find_last_in_line("four5sixseven").to_num(), 7);
    }

    #[test]
    fn last_both_numeric_and_spelled_digits() {
        assert_eq!(Digits::find_last_in_line("abc3deftwoghi7").to_num(), 7);
        assert_eq!(Digits::find_last_in_line("four5sixseven").to_num(), 7);
    }

    #[test]
    fn last_mixed_characters() {
        assert_eq!(Digits::find_last_in_line("nine9abceightdef2ghi").to_num(), 2);
        assert_eq!(
            Digits::find_last_in_line("7sqthfchpjklpn")
                .to_num(),
            7
        );
        assert_eq!(Digits::find_last_in_line("sevenabc1xyzthree").to_num(), 3);
    }

    #[test]
    fn it_works() {
        let input = Input::new(1, "test");
        assert_eq!(142, day1(input))
    }

    #[test]
    fn day_1_prod() {
        let input = Input::new(1, "prod");
        assert_eq!(56506, day1(input))
    }

    #[test]
    fn day_1_2_test() {
        let input = Input::new(1, "02.test");
        assert_eq!(281, input.solve())
    }

    #[test]
    fn day_1_2_test_prod() {
        let input = Input::new(1, "prod");
        assert_eq!(56017, input.solve())
    }
}
