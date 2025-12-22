fn parse(val: &str) -> isize {
    val.parse::<isize>().unwrap()
}

fn count(ranges: &str, id: isize) -> usize {
    for range in ranges.lines() {
        let (low, high) = range
            .split_once("-")
            .map(|(l, h)| (parse(l), parse(h)))
            .unwrap();
        if id - low > 0 && high - id > 0 {
            return 1;
        }
    }
    return 0;
}

pub fn run(input: &str) -> usize {
    let mut counter = 0;
    let (ranges, ids) = input.split_once("\n\n").unwrap();

    for id in ids.lines() {
        let val = parse(id);
        counter += count(ranges, val)
    }
    counter
}
