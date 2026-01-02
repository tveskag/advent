pub fn run(input: &str) -> usize {
    let (first, rest) = input
        .split_once("\n")
        .unwrap();
    let line_length = first.len();
    let state: Vec<bool> = first
        .chars()
        .map(|c| if c == 'S' { true } else { false })
        .collect();
    let (final_row, amount) = rest
        .lines()
        .fold((state, 0), |(acc_state, mut acc_sum), line| {
            let splitters = line
                .chars()
                .map(|c| if c == '^' { true } else { false });
            //let new_state: Vec<bool> = Vec::new();
            let new_state = acc_state
                .iter()
                .zip(splitters)
                .enumerate()
                .fold(
                    vec![false; line_length],
                    |mut acc, (index, (beam, split))| {
                        if *beam {
                            if split {
                                acc[index - 1] = true;
                                acc[index + 1] = true;
                                acc_sum += 1
                            } else {
                                acc[index] = true;
                            }
                        };
                        acc
                    },
                );
            (new_state, acc_sum)
        });

    amount
}
