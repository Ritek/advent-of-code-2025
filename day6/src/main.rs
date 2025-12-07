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

    let columns = content.lines().nth(0).unwrap().split("").count();
    for i in (0..=columns).rev() {
        for (j, line) in content.lines().enumerate() {
            let num = match line.chars().nth(i) {
                Some(x) => x,
                None => ' ',
            };
            match num {
                ' ' => {
                    if j == content.lines().count() - 1 {
                        buffer.push(' ');
                    }
                }
                '+' => {
                    let y = parse_buffer(&buffer);
                    // println!("+{:?}", y);
                    ans2 += y.iter().sum::<i64>();
                    buffer.clear();
                }
                '*' => {
                    let y = parse_buffer(&buffer);
                    // println!("*{:?}", y);
                    ans2 += y.iter().product::<i64>();
                    buffer.clear();
                }
                _ => buffer.push(num),
            }
        }
    }

    ans2
}

fn main() {
    let content = fs::read_to_string("./input.txt").expect("Error reading a file");

    let ans1: i64 = problem1(&content);
    println!("{:?}", ans1);

    let ans2: i64 = problem2(&content);
    println!("{:?}", ans2);

    println!("Answer to question 2: {}", ans2);
}
