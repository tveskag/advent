pub fn run(content: &str) -> usize {
    let mut counter = 0;
    let length = 12;

    for pack in content.lines() {
        let mut cut_off = pack.len();
        let mut joltage = 0;
        for n in (0..length).rev() {
            let (index, value) = extract(pack, cut_off, n);
            cut_off = index + 1;
            joltage += 10usize.pow(
                n.try_into()
                    .unwrap(),
            ) * value;
        }

        counter += joltage;
    }
    return counter;
}

fn extract(pack: &str, take: usize, skip: usize) -> (usize, usize) {
    pack.chars()
        .rev()
        .skip(skip)
        .take(take)
        .map(|e| {
            e.to_digit(10)
                .unwrap() as usize
        })
        .enumerate()
        .max_by(|a, b| {
            a.1.cmp(&b.1)
        })
        .unwrap()
}
