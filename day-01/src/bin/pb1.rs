fn main() {
    let src = include_str!("src1.txt");
    let (mut left, mut right) = src
        .lines()
        .map(str::split_whitespace)
        .map(|mut split| (split.next().unwrap(), split.next().unwrap()))
        .map(|(left, right)| (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap()))
        .unzip::<u32, u32, Vec<u32>, Vec<u32>>();

    left.sort();
    right.sort();

    let sum = left
        .iter()
        .zip(right.iter())
        .map(|(&left, &right)| left.abs_diff(right))
        .sum::<u32>();

    println!("Sum={sum}");
}
