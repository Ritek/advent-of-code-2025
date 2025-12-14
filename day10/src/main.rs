use std::{
    collections::{HashSet, VecDeque},
    fs,
};

fn parse_line(line: &str) -> (Vec<usize>, Vec<Vec<usize>>, Vec<usize>) {
    let mut target_buf: Vec<usize> = Vec::new();
    let mut button_buf: Vec<Vec<usize>> = Vec::new();
    let mut joltage_buf: Vec<usize> = Vec::new();

    let mut buffer: Option<Vec<usize>> = None;
    for (i, c) in line.chars().enumerate() {
        match (i, c) {
            (_, '[' | '{' | '(') => {
                buffer = Some(Vec::new());
                continue;
            }
            (_, ']') => {
                if let Some(v) = buffer.take() {
                    target_buf.extend(v);
                }
                continue;
            }
            (_, '}') => {
                if let Some(v) = buffer.take() {
                    joltage_buf.extend(v);
                }
                continue;
            }
            (_, ')') => {
                if let Some(v) = buffer.take() {
                    button_buf.push(v);
                }
                continue;
            }
            (_, ',') => (),
            (_, '.') => {
                if buffer.is_none() {
                    buffer = Some(Vec::new());
                }

                buffer.as_mut().unwrap().push(0);
            }
            (_, '#') => {
                if buffer.is_none() {
                    buffer = Some(Vec::new());
                }

                buffer.as_mut().unwrap().push(1);
            }
            (_, d) if d.is_ascii_digit() => {
                if buffer.is_none() {
                    buffer = Some(Vec::new());
                }

                buffer
                    .as_mut()
                    .unwrap()
                    .push(d.to_digit(10).unwrap() as usize);
            }
            _ => {}
        }
    }

    (target_buf, button_buf, joltage_buf)
}

fn find_clicks(target: Vec<usize>, buttons: Vec<Vec<usize>>) -> usize {
    let initial = 0usize;
    let target: usize = target
        .iter()
        .enumerate()
        .fold(0usize, |acc, (i, &b)| acc | (b << i));

    let mut queue = VecDeque::new();
    queue.push_back((initial, 0));

    let mut seen = HashSet::new();
    seen.insert(initial);

    while let Some((lights, dist)) = queue.pop_front() {
        if lights == target {
            return dist;
        }

        for button in buttons.iter() {
            let neighbor = button.iter().fold(lights, |acc, &n| acc ^ (1 << n));
            if seen.insert(neighbor) {
                queue.push_back((neighbor, dist + 1));
            }
        }
    }

    unreachable!("Target state is not reachable");
}

fn main() {
    let content = fs::read_to_string("./input.txt").expect("Error reading the file");

    let mut ans1 = 0;
    for line in content.lines() {
        let (target, buttons, joltage) = parse_line(line);

        // println!("{:?} | {:?} | {:?}", target, buttons, joltage);
        let min = find_clicks(target, buttons);
        // println!("min: {}", min);
        ans1 += min;
    }

    println!("Answer to question 1: {}", ans1);
}
