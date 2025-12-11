use std::cmp::{max, min};
use std::fs;

type IntType = i64;
type Point = (IntType, IntType);
type LineSegment = (Point, Point);

fn get_size(point1: (IntType, IntType), point2: (IntType, IntType)) -> IntType {
    let (x1, y1) = point1;
    let (x2, y2) = point2;

    let x = IntType::abs(x1 - x2) + 1;
    let y = IntType::abs(y1 - y2) + 1;

    return x * y;
}

fn solution1(points: &Vec<(IntType, IntType)>) -> IntType {
    let mut ans1: IntType = 0;
    for point1 in points.iter() {
        for point2 in points.iter() {
            let size = get_size(*point1, *point2);
            if size > ans1 {
                ans1 = size;
            }
        }
    }

    ans1
}

fn sort_segment(p1: Point, p2: Point) -> LineSegment {
    if p1 < p2 { (p1, p2) } else { (p2, p1) }
}

fn solution2(corners: &Vec<Point>) -> IntType {
    let n = corners.len();

    let mut edges: Vec<LineSegment> = Vec::new();
    let mut sizes: Vec<(IntType, Point, Point)> = Vec::new();

    for i in 0..n {
        let p_curr = corners[i];
        let p_prev = corners[(i + n - 1) % n];
        edges.push(sort_segment(p_curr, p_prev));

        for j in i + 1..n {
            let p_other = corners[j];
            let (c1, c2) = sort_segment(p_curr, p_other);
            sizes.push((get_size(c1, c2), c1, c2));
        }
    }

    edges.sort_by_key(|e| get_size(e.0, e.1));
    sizes.sort_by_key(|s| s.0);

    for (size, (x1, y1), (x2, y2)) in sizes.iter().rev() {
        let rect_min_x = x1;
        let rect_max_x = x2;
        let rect_min_y = min(y1, y2);
        let rect_max_y = max(y1, y2);

        // Check the heuristic: Does any polygon edge's bounding box fit completely inside the rectangle?
        let is_valid = !edges.iter().any(|&((x3, y3), (x4, y4))| {
            // Because edges are sorted: x3 <= x4 and y3 is not necessarily <= y4.
            // We need the min/max of the edge itself.
            let edge_min_x = x3;
            let edge_max_x = x4;
            let edge_min_y = min(y3, y4);
            let edge_max_y = max(y3, y4);

            // Heuristic check:
            // Does the edge's bounding box fit *strictly* inside the rectangle's interior?
            // Note: This check relies on the coordinates being sorted for the rectangle.
            edge_max_x > *rect_min_x
                && edge_min_x < *rect_max_x
                && edge_max_y > *rect_min_y
                && edge_min_y < *rect_max_y
        });

        if is_valid {
            return *size;
        }
    }

    0
}

fn main() {
    let content = fs::read_to_string("./input.txt").expect("Error reading file");

    let points: Vec<(IntType, IntType)> = content
        .lines()
        .map(|line| match line.split_once(",") {
            Some((x, y)) => (x.parse::<IntType>().unwrap(), y.parse().unwrap()),
            None => panic!("Cannot split line"),
        })
        .collect();

    let ans1 = solution1(&points);
    println!("Answer to question 1: {}", ans1);

    let ans2 = solution2(&points);
    println!("Answer to question 2: {}", ans2);
}
