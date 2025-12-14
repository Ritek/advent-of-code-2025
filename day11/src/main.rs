use std::{collections::HashMap, fs};

fn parse_line(line: &str) -> (&str, Vec<&str>) {
    let (name, nei) = line.split_once(": ").unwrap();
    let neighbors = nei.split_whitespace().into_iter().collect();

    (name, neighbors)
}

fn dp<'a>(
    neighbors: &HashMap<&'a str, Vec<&'a str>>,
    seen: &mut HashMap<&'a str, usize>,
    current: &'a str,
    finish: &str,
) -> usize {
    if let Some(visited) = seen.get(current) {
        return *visited;
    }

    if current == finish {
        return 1;
    }

    let total = match neighbors.get(current) {
        Some(v) => v
            .iter()
            .map(|next| dp(&neighbors, seen, next, finish))
            .sum(),
        None => 0,
    };
    seen.insert(current, total);

    total
}

fn problem2(edges: HashMap<&str, Vec<&str>>) -> usize {
    (dp(&edges, &mut HashMap::new(), "svr", "dac")
        * dp(&edges, &mut HashMap::new(), "dac", "fft")
        * dp(&edges, &mut HashMap::new(), "fft", "out"))
        + (dp(&edges, &mut HashMap::new(), "svr", "fft")
            * dp(&edges, &mut HashMap::new(), "fft", "dac")
            * dp(&edges, &mut HashMap::new(), "dac", "out"))
}

fn main() {
    let content = fs::read_to_string("./input.txt").expect("Error reading the file");

    let edges: HashMap<&str, Vec<&str>> = content.lines().map(|line| parse_line(line)).collect();

    let ans1 = dp(&edges, &mut HashMap::new(), "you", "out");

    println!("Answer to question 1: {}", ans1);

    let ans2 = problem2(edges);

    println!("Answer to question 2: {}", ans2);
}
