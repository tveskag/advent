pub fn main(content: String) -> u32 {
    let mut counter = 0;
    for pack in content.lines() {
        let (index, ten) = pack
            .chars()
            .take(pack.len() - 1)
            .map(|e| match e.to_digit(10) {
                Some(n) => -i16::from(n as u8),
                None => panic!("Error: not a digit"),
            })
            .enumerate()
            .min_by(|a, b| a.1.cmp(&b.1))
            .unwrap();
        let (one_index, one) = pack
            .chars()
            .skip(index + 1)
            .map(|e| match e.to_digit(10) {
                Some(n) => -i16::from(n as u8),
                None => panic!("Error: not a digit"),
            })
            .enumerate()
            .min_by(|a, b| a.1.cmp(&b.1))
            .unwrap();
        let joltage = -ten * 10 + -one;

        println!(
            "pack: {}, ten index: {}, one index: {}",
            pack, index, one_index
        );
        counter += joltage;
        //let one = println!("numbers: {:?}", numbers);
        //counter += numbers[0] * 10 + numbers[1];
    }
    return counter as u32;
}
