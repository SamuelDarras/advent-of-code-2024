use std::collections::HashMap;

fn main() {
    let src = include_str!("src2.txt");
    //     let src = "3   4
    // 4   3
    // 2   5
    // 1   3
    // 3   9
    // 3   3";

    let mut count: HashMap<u32, u32> = HashMap::new();
    let left = src
        .lines()
        .map(str::split_whitespace)
        .map(|mut split| (split.next().unwrap(), split.next().unwrap()))
        .map(|(left, right)| {
            let left = left.parse::<u32>().unwrap();
            let right = right.parse::<u32>().unwrap();

            let _ = match count.get(&right) {
                Some(c) => count.insert(right, c + 1),
                None => count.insert(right, 1),
            };

            left
        })
        .collect::<Vec<u32>>();

    let score = left
        .iter()
        .map(|&n| match count.get(&n) {
            Some(c) => *c * n,
            None => 0,
        })
        .sum::<u32>();

    println!("{count:?}");
    println!("Score={score}");
}
