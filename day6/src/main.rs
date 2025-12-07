use std::fs;

fn read_num_line(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|x| x.parse::<i32>().expect("Cannot convert"))
        .collect()
}

fn problem1(content: &String) -> i64 {
    let mut operations: Vec<&str> = Vec::new();
    let mut matrix: Vec<Vec<i32>> = Vec::new();

    for (i, line) in content.lines().into_iter().enumerate() {
        if i == content.lines().count() - 1 {
            operations = line.split_whitespace().into_iter().collect();
            break;
        }

        matrix.push(read_num_line(line));
    }

    let mut results: Vec<i64> = Vec::new();
    for (r, row) in matrix.iter().enumerate() {
        if r == 0 {
            results = row.iter().map(|x| i64::from(*x)).collect();
            continue;
        }

        for (c, col) in row.iter().enumerate() {
            if operations[c] == "*" {
                results[c] *= i64::from(*col);
            } else {
                results[c] += i64::from(*col);
            }
        }
    }

    results.iter().sum()
}

fn parse_buffer(buffer: &Vec<char>) -> Vec<i64> {
    let x: String = buffer.iter().collect();
    x.trim()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

fn problem2(content: &String) -> i64 {
    let mut ans2: i64 = 0;
    let mut buffer: Vec<char> = Vec::new();

    // Pre-split lines once
    let lines: Vec<&str> = content.lines().collect();

    // Pre-convert each line to Vec<char> for O(1) indexing
    let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    // Determine column count from the first line
    let columns = grid.first().map(|l| l.len()).unwrap_or(0);
    for i in (0..=columns).rev() {
        for (j, line) in content.lines().enumerate() {
            let c = line.chars().nth(i).unwrap_or(' ');
            match c {
                ' ' => {
                    if j == content.lines().count() - 1 {
                        buffer.push(' ');
                    }
                }
                '+' => {
                    let vals = parse_buffer(&buffer);
                    ans2 += vals.iter().sum::<i64>();
                    buffer.clear();
                }
                '*' => {
                    let vals = parse_buffer(&buffer);
                    ans2 += vals.iter().product::<i64>();
                    buffer.clear();
                }
                _ => buffer.push(c),
            }
        }
    }

    ans2
}

fn main() {
    let content = fs::read_to_string("./input.txt").expect("Error reading a file");

    let ans1: i64 = problem1(&content);
    println!("Answer to question 1: {}", ans1);

    let ans2: i64 = problem2(&content);
    println!("Answer to question 2: {}", ans2);
}
