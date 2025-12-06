use std::{cmp::max, fs};

fn parse_ids(line: &str) -> (i64, i64) {
    match line.split_once("-") {
        Some((s, e)) => (s.parse::<i64>().unwrap(), e.parse::<i64>().unwrap()),
        None => panic!("Could not split {}", line),
    }
}

fn parse_file(content: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    match content.split_once("\n\n") {
        Some((ids_str, ranges_str)) => {
            let ranges = ids_str.lines().map(|x| parse_ids(x)).collect();
            let ids = ranges_str
                .lines()
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
            (ranges, ids)
        }
        None => panic!("Could not split file in two!"),
    }
}

fn main() {
    let content = fs::read_to_string("./input.txt").expect("Error reading file");

    let (mut ranges, ids) = parse_file(content.as_ref());

    ranges.sort_by_key(|x| x.0);

    let mut ans1 = 0;
    for id in ids {
        for (start, end) in &ranges {
            if (start..=end).contains(&&id) {
                ans1 += 1;
                break;
            }
        }
    }

    let mut merged: Vec<(i64, i64)> = Vec::new();
    for range in ranges {
        match merged.pop() {
            None => merged.push(range),
            Some(popped) => match range.0 <= popped.1 {
                true => merged.push((popped.0, max(popped.1, range.1))),
                false => {
                    merged.push(popped);
                    merged.push(range);
                }
            },
        }
    }

    let mut ans2 = 0;
    for range in &merged {
        let size = (range.0..=range.1).count();
        ans2 += size;
    }

    println!("Answer to question 1: {}", ans1);
    println!("Answer to question 2: {}", ans2);
}
