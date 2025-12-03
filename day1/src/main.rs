use std::fs::File;
use std::io::{self};

fn get_file_buffer(path: &str) -> io::BufReader<File> {
    let file_result = File::open(path);
    let file = match file_result {
        Ok(file) => file,
        Err(error) => panic!("File {} not found: {}", path, error),
    };

    io::BufReader::new(file)
}

fn parse_line<R: io::BufRead, F: FnMut(i32)>(reader: R, mut callback: F) -> io::Result<()> {
    for line in reader.lines() {
        let line = line?;
        let (dir_str, num_str) = line.split_at(1);

        let dir = if dir_str == "R" { 1 } else { -1 };
        let num = num_str
            .parse::<i32>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        callback(dir * num)
    }

    Ok(())
}

fn main() {
    let mut initial: i32 = 50;
    let mut ans1: i32 = 0;
    let mut ans2: i32 = 0;

    let reader = get_file_buffer("./input.txt");
    let _ = parse_line(reader, |num: i32| {
        initial = (initial + num).rem_euclid(100);

        // for _ in 0..num.abs() {
        //     if num > 0 {
        //         initial += 1;
        //     } else {
        //         initial -= 1;
        //     }
        //     initial = initial.rem_euclid(100);

        //     if initial == 0 {
        //         ans2 += 1;
        //     }
        // }
        let loops = (num / 100).abs();
        let rest = num.div_euclid(100);
        println!("{} + {}", initial, num);
        println!("{} & {}\n", loops, rest);

        initial += num;
        initial = initial.div_euclid(100);

        if initial == 0 {
            ans1 += 1
        }
    });

    println!("Answer to question 1: {}", ans1);
    println!("Ansert to question 2: {}", ans2); // 6892
}
