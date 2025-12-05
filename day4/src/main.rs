use std::fs;

type Matrix = Vec<Vec<char>>;
type Position = (usize, usize);

fn print_matrix(matrix: &Matrix) {
    for row in matrix {
        println!("{:?}", row);
    }
}

fn count_neighbors(matrix: &Matrix, pos: Position) -> i8 {
    let mut neighbors = 0;
    let (row, col) = pos;

    let dir: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (d_row, d_col) in dir {
        let new_row_isize = row as isize + d_row;
        let new_col_isize = col as isize + d_col;

        if new_row_isize < 0 || new_col_isize < 0 {
            continue;
        }

        let new_row = new_row_isize as usize;
        let new_col = new_col_isize as usize;

        if let Some(row_vec) = matrix.get(new_row) {
            if let Some(&cell) = row_vec.get(new_col) {
                if cell == '@' {
                    neighbors += 1;
                }
            }
        }
    }

    neighbors
}

fn find_cells_with_less_than_four(matrix: &Matrix) -> Vec<(usize, usize)> {
    let mut positions: Vec<(usize, usize)> = Vec::new();
    for (row, vec_rows) in matrix.iter().enumerate() {
        for (col, _) in vec_rows.iter().enumerate() {
            if matrix[row][col] == '@' && count_neighbors(&matrix, (row, col)) < 4 {
                positions.push((row, col));
            }
        }
    }
    return positions;
}

fn remove_cells(matrix: &mut Matrix) -> usize {
    let mut counter = 0;

    loop {
        let cells = find_cells_with_less_than_four(matrix);
        let len = cells.len();
        if len == 0 {
            break;
        }
        counter += len;

        for (row, col) in cells {
            matrix[row][col] = '.';
        }
    }

    counter
}

fn main() {
    let content = fs::read_to_string("./input.txt").expect("Error reading file");

    let mut matrix: Matrix = Vec::new();
    for line in content.lines() {
        let temp: Vec<char> = line.chars().into_iter().collect();
        matrix.push(temp);
    }

    let ans1 = find_cells_with_less_than_four(&mut matrix).len();
    let ans2 = remove_cells(&mut matrix);

    println!("Answer to question 1: {}", ans1);
    println!("Answer to question 2: {}", ans2);

    // print_matrix(&matrix);
}
