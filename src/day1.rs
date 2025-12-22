fn part2(content: String) {
    let mut dial: i32 = 50;
    let mut counter: i32 = 0;

    for line in content.lines() {
        let (sign, change) = match line.split_at(1) {
            ("R", amount) => (1, amount.parse::<i32>().unwrap()),
            ("L", amount) => (-1, amount.parse::<i32>().unwrap()),
            _ => (1, 0),
        };
        for _ in 0..change {
            dial += sign;
            let digits = dial.to_string().chars().rev().take(2).collect::<String>();
            if dial == 0 || digits == "00" {
                counter += 1;
            }
        }

        //println!(
        //    "line: {:?}, dial: {:?}, dif: {:?}, counter: {:?}",
        //    line, dial, dif, counter
        //);
    }
    println!("zeros: {:?}", counter)
}

fn part1(content: String) {
    let mut dial: i32 = 50;
    let mut counter: i32 = 0;

    for line in content.lines() {
        dial += match line.split_at(1) {
            ("R", amount) => amount.parse::<i32>().unwrap(),
            ("L", amount) => -amount.parse::<i32>().unwrap(),
            _ => 0,
        };
        let digits = dial.to_string().chars().rev().take(2).collect::<String>();

        //println!("line: {:?}, dial: {:?}, digits: {:?}", line, dial, digits);

        if dial == 0 || digits == "00" {
            counter += 1;
        }
    }
    println!("zeros: {:?}", counter)
}
