pub fn main(content: String) -> usize {
    let mut counter = 0;
    for interval in content.split(",") {
        let (startstr, endstr) = interval.split_once("-").unwrap();
        println!("start - end: {:?} - {:?}", startstr, endstr);
        let (start, end) = (
            startstr.parse::<usize>().unwrap(),
            endstr.trim().parse::<usize>().unwrap(),
        );
        for n in start..end + 1 {
            counter += part1(n);
            counter += part2(n);
        }
    }
    return counter;
}

fn part1(n: usize) -> usize {
    let str = n.to_string();
    let (left, right) = str.split_at(str.len() / 2);
    if left == right {
        return n;
    }
    return 0;
}

fn part2(n: usize) -> usize {
    let str = n.to_string();
    match str.split_at(str.len() / 2) {
        (left, right) if right.len() == left.len() => even(left, right),
        (left, right) => odd(left, right),
        (_, _) => panic!("empty"),
    }
    fn even(left: &str, right: &str) {}
    fn odd(left: &str, right: &str) {}
    return 0;
}
