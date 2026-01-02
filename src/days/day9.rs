use crate::util::parse;

pub fn run(input: &str) -> usize {
    let tiles = input
        .split("\n")
        .take_while(|line| line.len() > 0)
        .map(|line| {
            let (x, y) = line
                .split_once(",")
                .unwrap();
            (parse::<isize>(x), parse::<isize>(y))
        });

    let squares = tiles
        .clone()
        .enumerate()
        .map(|(index1, tile1)| {
            tiles
                .clone()
                .skip(index1 + 1)
                .map(move |tile2| {
                    let (x1, y1) = tile1;
                    let (x2, y2) = tile2;
                    ((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1)
                })
        })
        .flatten();

    squares
        .max()
        .unwrap() as usize
}
