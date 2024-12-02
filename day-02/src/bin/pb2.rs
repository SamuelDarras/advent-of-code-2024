const DAMPENING: bool = true;

pub fn main() {
    let puzzle_input = include_str!("src2.txt");

    let lines: Vec<Vec<u64>> = puzzle_input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect()
        })
        .collect();

    let safe_count = lines
        .into_iter()
        .filter(|levels| {
            (0..levels.len()).any(|skip| {
                let mut levels = levels.clone();
                levels.remove(skip);
                check_levels(&levels)
            })
        })
        .count();

    println!("Safe: {}", safe_count);
}

fn check_levels(levels: &[u64]) -> bool {
    let direction = levels[1] > levels[0];
    levels.windows(2).all(|w| {
        let dist: i32 = w[0] as i32 - w[1] as i32;
        (w[1] > w[0]) == direction && dist >= -3 && dist <= 3 && dist != 0
    })
}
