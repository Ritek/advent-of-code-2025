use std::fs;

fn find_max_number(line: &str, k: usize) -> i64 {
    let mut stack: Vec<u8> = Vec::new();
    let chars = line.as_bytes();
    let line_length = line.len();

    for i in 0..line.len() {
        let remaining = line_length - i;
        let num: u8 = chars[i];

        while stack.len() > 0 && *stack.last().unwrap() < num && stack.len() - 1 + remaining >= k {
            stack.pop();
        }

        stack.push(num);
    }

    let mut ans: i64 = 0;
    for &digit in stack.iter().take(k) {
        ans = ans * 10 + (digit as i64 - 0x30);
    }

    ans
}

fn main() {
    let path = "./input.txt";
    let content = fs::read_to_string(path).expect("Erorr reading file");

    let mut ans1: i64 = 0;
    for line in content.lines() {
        ans1 += find_max_number(line, 2);
    }

    let mut ans2: i64 = 0;
    for line in content.lines() {
        ans2 += find_max_number(line, 12);
    }

    println!("Answer to question 1: {}", ans1);
    println!("Answer to question 2: {}", ans2);
}
