use std::fs;

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    println!("Hello, world!");
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

fn get_first(line: &str) -> usize {
    let first_char = line.chars().find(|c| c.is_numeric());
    if first_char.is_none() {
        if let Some(first_spelled_out_digit) = DIGITS
            .iter()
            .filter(|num_name| line.contains(**num_name))
            .min_by(|x, y| {
                let x_idx = line.find(*x);
                let y_idx = line.find(*y);

                x_idx.cmp(&y_idx)
            })
        {
            return DIGITS
                .iter()
                .position(|x| x == first_spelled_out_digit)
                .unwrap()
                + 1;
        }
    }
    let first_num_char = line.find(first_char.unwrap()).unwrap();


    if let Some((first_occ, first_spelled_out_digit)) = DIGITS
        .iter()
        .enumerate()
        .filter(|(_idx, num_name)| line.contains(**num_name))
        .min_by(|x, y| {
            let x_num = x.1;
            let y_num = y.1;

            let x_idx = line.find(x_num);
            let y_idx = line.find(y_num);

            x_idx.cmp(&y_idx)
        })
        .map(|(_, num_name)| (line.find(num_name).unwrap(), num_name))
    {
        if first_occ < first_num_char {
            return DIGITS
                .iter()
                .position(|x| x == first_spelled_out_digit)
                .unwrap()
                + 1;
        };
    }
    first_char.unwrap().to_digit(10).unwrap() as usize
}

fn get_last(line: &str) -> usize {
    let last_char = line.chars().rfind(|c| c.is_numeric());
    if last_char.is_none() {
        if let Some((_last_occ, last_spelled_out_digit)) = DIGITS
            .iter()
            .enumerate()
            .filter(|(_idx, num_name)| line.contains(**num_name))
            .min_by(|x, y| {
                let x_num = x.1;
                let y_num = y.1;

                let x_idx = line.find(x_num).unwrap();
                let y_idx = line.find(y_num).unwrap();

                y_idx.cmp(&x_idx)
            })
        {
            return DIGITS
                .iter()
                .position(|x| x == last_spelled_out_digit)
                .unwrap() + 1;
        }
    }
    let last_num_char = line.rfind(last_char.unwrap()).unwrap();

    if let Some((last_occ, last_spelled_out_digit)) = DIGITS
        .iter()
        .enumerate()
        .filter(|(_idx, num_name)| line.contains(**num_name))
        .map(|(_idx, num_name)| (line.rfind(num_name).unwrap(), num_name))
        .max_by(|x, y| {
            let x_num = x.1;
            let y_num = y.1;

            let x_idx = line.find(x_num);
            let y_idx = line.find(y_num);

            x_idx.unwrap().cmp(&y_idx.unwrap())
        })
    {
        if last_occ > last_num_char {
            return DIGITS
                .iter()
                .position(|x| x == last_spelled_out_digit)
                .unwrap() + 1;
        };
    }

    last_char.unwrap().to_digit(10).unwrap() as usize
}


fn day1_2(input: &str) -> usize {
    return input.lines().filter(|line| !line.is_empty()).map(|line| {
        let first = get_first(line);
        let last = get_last(line);

        let num = 10 * first + last;

        dbg!(line, first, last, num);
        num
    }).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_digit_at_start() {
        assert_eq!(get_first("4abctwothree"), 4);
    }

    #[test]
    fn spelled_digit_at_start() {
        assert_eq!(get_first("one234five"), 1);
    }

    #[test]
    fn both_numeric_and_spelled_digits() {
        assert_eq!(get_first("abc3deftwoghi"), 3);
        assert_eq!(get_first("four5six7"), 4);
    }

    #[test]
    fn mixed_characters() {
        assert_eq!(get_first("xyz9abceightdef2ghi"), 9);
        assert_eq!(get_first("sevenabc1xyz"), 7);
    }

    #[test]
    fn numeric_digit_at_end() {
        assert_eq!(get_last("abctwothree4"), 4);
    }

    #[test]
    fn spelled_digit_at_end() {
        assert_eq!(get_last("234fiveone"), 1);
    }

    #[test]
    fn last_both_numeric_and_spelled_digits() {
        assert_eq!(get_last("abc3deftwoghi7"), 7);
        assert_eq!(get_last("four5sixseven"), 7);
    }


    #[test]
    fn last_mixed_characters() {
        assert_eq!(get_last("nine9abceightdef2ghi"), 2);
        assert_eq!(get_last("sevenabc1xyzthree"), 3);
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
        assert_eq!(281, day1_2(&input.data))
    }


    #[test]
    fn day_1_2_test_prod() {
        let input = Input::new(1, "prod");
        assert_eq!(0, day1_2(&input.data))
    }
}
