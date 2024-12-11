use memoize::memoize;

const BLINKS: usize = 75;

type Stone = u64;

fn main() {
    let src = include_str!("src2.txt");
    // let src = "125 17";
    // let src = "1";

    let stones = src
        .split_whitespace()
        .map(str::parse::<Stone>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    let result = stones.iter().fold(0, |acc, &stone| {
        let result = blink_a_stone_n_times(BLINKS, stone);
        acc + result
    });
    println!("{result}");
}

#[memoize]
fn blink_a_stone_n_times(blinks: usize, stone: Stone) -> usize {
    if blinks == 0 {
        return 1;
    }

    if stone == 0 {
        blink_a_stone_n_times(blinks - 1, 1)
    } else {
        let s = format!("{stone}");
        if s.len() % 2 == 0 {
            let (left, right) = s.split_at(s.len() / 2);
            blink_a_stone_n_times(blinks - 1, left.parse::<Stone>().unwrap())
                + blink_a_stone_n_times(blinks - 1, right.parse::<Stone>().unwrap())
        } else {
            blink_a_stone_n_times(blinks - 1, stone * 2024)
        }
    }
}
