use std::collections::HashMap;

fn main() {
    let src = include_str!("src2.txt");
//     let src = "3   4
// 4   3
// 2   5
// 1   3
// 3   9
// 3   3";

    let (left, right) = src
        .lines()
        .map(str::split_whitespace)
        .map(|mut split| (split.next().unwrap(), split.next().unwrap()))
        .map(|(left, right)| (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap()))
        .unzip::<u32, u32, Vec<u32>, Vec<u32>>();

    let mut count: HashMap<u32, u32> = HashMap::new();
    for n in right.iter() {
        let _ = match count.get(n) {
            Some(c) => count.insert(*n, c + 1),
            None => count.insert(*n, 1),
        };
    }

    println!("{count:#?}");

    let score = left
        .iter()
        .map(|&n| match count.get(&n) {
            Some(c) => *c * n,
            None => 0,
        })
        .sum::<u32>();

    println!("Score={score}");
}
