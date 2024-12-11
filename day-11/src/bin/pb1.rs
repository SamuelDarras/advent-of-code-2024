use std::sync::mpsc::channel;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let src = include_str!("src1.txt");
    // let src = "125 17";

    let stones = src
        .split_whitespace()
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    println!("Stones: {stones:?}");

    let mut stones = stones;

    let ttl = 25;
    for i in 0..ttl {
        let (sender, reciever) = channel();
        stones.par_iter().for_each_with(sender, |sen, stone| {
            if *stone == 0 {
                sen.send(1).unwrap();
            } else {
                let s = format!("{stone}");
                if s.len() % 2 == 0 {
                    let (left, right) = s.split_at(s.len() / 2);
                    sen.send(left.parse::<usize>().unwrap()).unwrap();
                    sen.send(right.parse::<usize>().unwrap()).unwrap();
                } else {
                    sen.send(*stone * 2024).unwrap();
                }
            }
        });
        let new_stones = reciever.iter().collect::<Vec<_>>();
        println!(
            "Blink #{i:03}, Delta={:>8}",
            new_stones.len() - stones.len()
        );
        stones = new_stones;
    }

    println!("There is {} stones", stones.len())
}
