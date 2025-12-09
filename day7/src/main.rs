use std::fs;

fn solution1(content: &String) -> i32 {
    let mut beams: Vec<bool> = Vec::new();
    let mut ans1 = 0;

    for (row, line) in content.lines().enumerate() {
        if row == 0 {
            for (_, col) in line.chars().enumerate() {
                beams.push(col == 'S');
            }
            continue;
        }

        for (c, col) in line.chars().enumerate() {
            if col == '^' {
                let old: [bool; 3] = [beams[c - 1], beams[c], beams[c + 1]];

                beams[c] = false;
                beams[c - 1] = true;
                beams[c + 1] = true;

                if old != [beams[c - 1], beams[c], beams[c + 1]] {
                    ans1 += 1;
                }
            }
        }
    }

    ans1
}

fn solution2(content: &String) -> usize {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    content
        .lines()
        .for_each(|x| matrix.push(x.chars().collect()));

    let mut dp: Vec<usize> = vec![1; matrix[0].len()];
    for row in (0..matrix.len()).rev() {
        for (col, c) in matrix[row].iter().enumerate() {
            if *c == '^' {
                dp[col] = dp[col - 1] + dp[col + 1];
            }
        }
    }

    let start: usize = matrix[0]
        .iter()
        .enumerate()
        .find_map(|(i, x)| {
            if *x == 'S' {
                return Some(i);
            } else {
                return None;
            }
        })
        .unwrap();

    return dp[start];
}

fn main() {
    let content = fs::read_to_string("./input.txt").expect("Error reading file");

    let ans1 = solution1(&content);

    println!("Answer to question 1: {}", ans1);

    let ans2 = solution2(&content);
    println!("Answer to question 2: {}", ans2);
}
