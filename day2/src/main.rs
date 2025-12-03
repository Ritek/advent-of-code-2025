use std::{collections::HashSet, fs, io::Read};

fn read_file(path: &str) -> std::io::Result<String> {
    let mut f = fs::File::open(path)?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn parse_line(line: &String) -> Vec<(i64, i64)> {
    line.split(",")
        .map(|x| {
            let (a, b) = x.split_once("-").expect("Expected one -");
            let x: i64 = a.parse().unwrap();
            let y: i64 = b.parse().unwrap();
            (x, y)
        })
        .collect()
}

fn find_invalid_in_range((start, end): (i64, i64)) -> Vec<i64> {
    let mut invalid: Vec<i64> = Vec::new();
    let range = start..=end;

    for num in range {
        let num_str = num.to_string();
        let l = num_str.len();
        let (left, right) = num_str.split_at(l / 2);
        // println!("{} | {}", left, right);
        if left.eq(right) {
            invalid.push(num);
        }
    }

    invalid
}

fn is_invalid(num: &String) -> bool {
    let length = num.len();
    let mid = length / 2;

    for n in 1..=mid {
        let chunks = num
            .as_bytes()
            .chunks(n)
            .map(|chunk| std::str::from_utf8(chunk).unwrap());

        let hash_set: HashSet<&str> = HashSet::from_iter(chunks);

        if hash_set.len() == 1 {
            return true;
        }
    }

    return false;
}

fn find_invalid_in_range2((start, end): (i64, i64)) -> Vec<i64> {
    let mut invalid: Vec<i64> = Vec::new();
    let range = start..=end;

    for num in range {
        let num_str = num.to_string();
        if is_invalid(&num_str) {
            invalid.push(num);
        }
    }

    invalid
}

fn main() {
    let content = match read_file("./input.txt") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Filed in reading file: {}", e);
            return;
        }
    };

    let nums = parse_line(&content);
    let mut sum: i64 = 0;
    for num in nums {
        let invalid = find_invalid_in_range(num);
        // println!("{:?} -> {:?}", num, invalid);
        sum += invalid.iter().sum::<i64>();
    }

    let nums2 = parse_line(&content);
    let mut sum2: i64 = 0;
    for num in nums2 {
        let invalid = find_invalid_in_range2(num);
        sum2 += invalid.iter().sum::<i64>();
    }

    println!("Answer to question 1: {}", sum);
    println!("Answer to question 2: {}", sum2);
}
