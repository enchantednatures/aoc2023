
fn day1_2(input: Input) {
    let digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut sum = 0;
    for line in input.data.lines() {
        let mut first = line.chars().find(|c| c.is_numeric()).unwrap();
        let first_idx = line.find(first).unwrap();

        if let Some(f) = digits
            .iter()
            .enumerate()
            .filter(|(idx, num_name)| line.contains(**num_name) && idx + 1 < first_idx)
            .min_by(|x, y| {
                let x_num = x.1;
                let y_num = y.1;

                let x_idx = line.find(x_num).unwrap();
                let y_idx = line.find(y_num).unwrap();

                x_idx.cmp(&y_idx)
            })
        {
            first = format!("{}", f.0 + 1).chars().next().unwrap();
        }

        let mut last = line.chars().rev().find(|c| c.is_numeric()).unwrap();
        let last_idx = line.rfind(last).unwrap();

        if let Some(f) = digits
            .iter()
            .enumerate()
            .filter(|(idx, num_name)| line.contains(**num_name) && idx + 1 > last_idx)
            .max_by(|x, y| {
                let x_num = x.1;
                let y_num = y.1;

                let x_idx = line.find(x_num).unwrap();
                let y_idx = line.find(y_num).unwrap();

                x_idx.cmp(&y_idx)
            })
        {
            last = format!("{}", f.0 + 1).chars().next().unwrap();
        }

        let arr = [first, last];
        let num: String = arr.iter().collect();
        let num: u32 = num.parse().unwrap();
        sum += num;
    }
    println!("sum: {}", sum);
}
