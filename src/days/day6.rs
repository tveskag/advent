// use std::iter::Zip;

use crate::util::parse;

// macro_rules! split {
//     ($x: expr, $y: tt) => {
//         let $(
//             $x.next().unwrap().chars()
//         )*
//     };
// }

// macro_rules! zip {
//     ($x: expr) => ($x);
//     ($x: expr, $($y: expr), +) => (
//         $x.iter().zip(
//             zip!($($y), +))
//     )
// }

// fn zipper(mut main: &impl Iterator, tozip: &impl Iterator) -> Zip<T, R> {
//     main.zip(tozip)
// }

// pub fn run2(input: &str) -> usize {
//     let rows = input.split("\n");
//     let init = rows.last();

//     rows.fold(init, |acc, row| acc.zip(row.chars));

//     let (numbers1, numbers2, numbers3, numbers4, operators) = split!(rows, 4);

//     let zipped = zip!(numbers1.chars(), numbers2, numbers3, numbers4);
//     zipped.map(|val| match val {
//         (((num1, num2),num3),num4) =>
//     })

// }

pub fn run(input: &str) -> usize {
    let mut numbers = input.split("\n");
    //.map(|list| list.split(" ").map(|val| parse::<usize>(val)));

    let numbers1 = numbers
        .next()
        .unwrap();
    let numbers2 = numbers
        .next()
        .unwrap();
    let numbers3 = numbers
        .next()
        .unwrap();
    let numbers4 = numbers
        .next()
        .unwrap();
    let operators = numbers
        .next()
        .unwrap();

    let zipped = operators
        .split_whitespace()
        .zip(
            numbers1
                .split_whitespace()
                .map(|val| parse::<usize>(val)),
        )
        .zip(
            numbers2
                .split_whitespace()
                .map(|val| parse::<usize>(val)),
        )
        .zip(
            numbers3
                .split_whitespace()
                .map(|val| parse::<usize>(val)),
        )
        .zip(
            numbers4
                .split_whitespace()
                .map(|val| parse::<usize>(val)),
        );

    zipped.fold(0, |acc, val| match val {
        (((("*", num1), num2), num3), num4) => acc + num1 * num2 * num3 * num4,
        (((("+", num1), num2), num3), num4) => acc + num1 + num2 + num3 + num4,
        ((((op, num1), num2), num3), num4) => panic!(
            "Operator: {}, Num1: {}, Num2: {}, Num3: {}, Num4: {}",
            op, num1, num2, num3, num4
        ),
    })
}
