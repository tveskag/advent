use std::iter::once;

pub fn run(content: &str) -> usize {
    let (first, _) = content
        .split_once("\n")
        .unwrap();
    let line_length = first.len();
    let empty_line = vec![0; line_length + 2];

    let lines: Vec<Vec<usize>> = once(empty_line.clone())
        .chain(
            content
                .lines()
                .map(|line| {
                    once('.')
                        .chain(line.chars())
                        .chain(once('.'))
                        .map(|c| if c == '@' { 1 } else { 0 })
                        .collect()
                }),
        )
        .chain(once(empty_line.clone()))
        .collect();

    let mut counter: usize = 0;

    for triple in lines.windows(3) {
        let [top, middle, bottom] = triple else {
            panic!("Too few lines!")
        };

        for square in top
            .windows(3)
            .zip(middle.windows(3))
            .zip(bottom.windows(3))
        {
            let ((top_slice, &[left, center, right]), bottom_slice) = square
            else {
                panic!("Too short lines!")
            };
            if center == 1 {
                let sum = left
                    + right
                    + top_slice
                        .iter()
                        .fold(0, |acc, e| acc + e)
                    + bottom_slice
                        .iter()
                        .fold(0, |acc, e| acc + e);
                if sum < 4 {
                    counter += 1;
                }
            }
        }
    }
    counter
}

/*

fn generate_lattice(content: String, side_length: usize) {


    for (row_index, row) in content.lines().enumerate() {
        for (space_index, space) in row.char_indices() {
            //content[]
        }
    }
}


fn unDirect(mat: [[bool; 10]; 10]) -> [[bool; 10]; 10] { mat.iter().enumerate().map(|(i, row)| { row.iter().enumerate().map(|(j, _)| mat[i][j] || mat[j][i] )})}

fn generateAdjacency(condition: (i: usize, j: usize, length: usize) -> bool) {
  return |x: number, y: number| {
    let list: [bool; x*x];
    return unDirect(list.map(|_, i| list.map(|_, j| condition(i,j,x,y) && if (i != j) { 1 } else {0})));
  }
}

//export const generateCircle = generateAdjacency((i, j, x, y) => i === j+x)

//export const generateSquare = generateAdjacency((i, j, x, y) =>
// (i%x && i === j+1)
// || (i === j+x)
//)
*/
