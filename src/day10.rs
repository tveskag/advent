use crate::util::{BoolVec, parse};

fn empty_false(size: usize, indices: Vec<usize>) -> Vec<bool> {
    [..size]
        .iter()
        .enumerate()
        .map(|(i, _)| indices.contains(&i))
        .collect()
}

fn trim(string: &str) -> &str {
    string.trim_matches(&['{', '}', '(', ')', '[', ']'][..])
}

pub fn run(input: &str) -> usize {
    let split = input
        .lines()
        .map(|line| {
            line.split_once(" ")
                .unwrap()
        })
        .map(|(indicator, rest)| {
            let (buttons, joltage) = rest
                .rsplit_once(" ")
                .unwrap();
            (trim(indicator), buttons, trim(joltage))
        });

    let mut counter = 0;
    for (indicator, buttons, _) in split {
        let indicator_vec: Vec<bool> = indicator
            .chars()
            .map(|c| if c == '#' { true } else { false })
            .collect();

        let buttons_mat: Vec<Vec<bool>> = buttons
            .split(" ")
            .map(|button| {
                let indices: Vec<usize> = trim(button)
                    .split(",")
                    .map(|c| parse::<usize>(c))
                    .collect();
                empty_false(indicator_vec.len(), indices)
            })
            .collect();
        counter +=
            find_smallest_spanning_set(buttons_mat, indicator_vec).len();
    }
    counter
}

fn gaussian_elimination(vectors: Vec<Vec<bool>>) {
    match vectors.iter().enumerate().find(|(_, vec)| match vec.first() {
        Some(v) => *v,
        None => panic!("Empty vector")
    }) {
        Some((ind, _)) => vectors.swap(0, ind),
        None => ()
    }
}

fn find_smallest_spanning_set(
    buttons: Vec<Vec<bool>>,
    indicator: Vec<bool>,
) -> Vec<Vec<bool>> {
    buttons

    if vectors.last().unwrap().last().unwrap() {

    }
}
