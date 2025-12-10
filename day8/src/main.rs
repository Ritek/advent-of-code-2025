use std::collections::HashMap;

type Point = (i64, i64, i64);

fn dist2(a: Point, b: Point) -> i64 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    let dz = a.2 - b.2;
    dx * dx + dy * dy + dz * dz
}

struct UnionFind {
    parent: HashMap<Point, Point>,
    rank: HashMap<Point, i32>,
    num_sets: usize,
}

impl UnionFind {
    fn new(points: &Vec<Point>) -> Self {
        let mut parent = HashMap::new();
        let mut rank = HashMap::new();
        let num_sets: usize = points.len();

        for &p in points {
            parent.insert(p, p);
            rank.insert(p, 0);
        }

        UnionFind {
            parent,
            rank,
            num_sets,
        }
    }

    fn find(&mut self, p: Point) -> Point {
        let parent_p = *self.parent.get(&p).unwrap();

        if parent_p != p {
            let root = self.find(parent_p);
            self.parent.insert(p, root);
            root
        } else {
            p
        }
    }

    fn union(&mut self, a: Point, b: Point) -> bool {
        let ra = self.find(a);
        let rb = self.find(b);

        if ra == rb {
            return false;
        }

        let rank_a = self.rank[&ra];
        let rank_b = self.rank[&rb];

        if rank_a < rank_b {
            self.parent.insert(ra, rb);
        } else if rank_a > rank_b {
            self.parent.insert(rb, ra);
        } else {
            self.parent.insert(rb, ra);
            self.rank.insert(ra, rank_a + 1);
        }
        self.num_sets -= 1;

        true
    }
}

const MERGES: usize = 1000;
fn main() {
    let content = std::fs::read_to_string("./input.txt").expect("Cannot read file");

    let points: Vec<Point> = content
        .lines()
        .map(|line| {
            let nums: Vec<i64> = line.split(',').map(|n| n.parse().unwrap()).collect();
            (nums[0], nums[1], nums[2])
        })
        .collect();

    let mut uf = UnionFind::new(&points);

    let mut edges: Vec<(i64, Point, Point)> = Vec::new();
    let n = points.len();
    for i in 0..n {
        for j in i + 1..n {
            let a = points[i];
            let b = points[j];
            let d2 = dist2(a, b);
            edges.push((d2, a, b));
        }
    }

    edges.sort_by_key(|e| e.0);

    for i in 0..MERGES {
        let (_d2, a, b) = edges[i];
        uf.union(a, b);
    }

    let mut groups: HashMap<Point, usize> = HashMap::new();
    for &p in &points {
        let root = uf.find(p);
        *groups.entry(root).or_insert(0) += 1;
    }

    let mut group_sizes: Vec<usize> = groups.values().cloned().collect();
    group_sizes.sort_by(|a, b| b.cmp(a));

    let mut ans1: i64 = 1;
    let top_three_sizes = group_sizes.iter().take(3);

    for size in top_three_sizes {
        ans1 *= *size as i64;
    }

    println!("Answer to question 1: {}", ans1);

    let mut i = 0;
    let mut last: (Point, Point) = (edges[0].1, edges[0].2);
    while uf.num_sets > 1 {
        let (_d2, a, b) = edges[i];
        let grouped = uf.union(a, b);

        if grouped {
            last = (edges[i].1, edges[i].2);
        }
        i += 1;
    }

    let ans2: i64 = last.0.0 * last.1.0;
    println!("Answer to question 2: {}", ans2);
}
