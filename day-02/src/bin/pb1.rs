fn main() {
    let src = include_str!("src1.txt");
//     let src = "7 6 4 2 1
// 1 2 7 8 9
// 9 7 6 2 1
// 1 3 2 4 5
// 8 6 4 4 1
// 1 3 6 7 9";

    let mut reports = src
        .lines()
        .map(str::split_whitespace)
        .map(|split| {
            split
                .map(str::parse::<i32>)
                .map(Result::unwrap)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let diffs = reports
        .iter_mut()
        .map(|v| v.windows(2).map(|s| s[0] - s[1]).collect::<Vec<i32>>())
        .collect::<Vec<_>>();

    let res = diffs
        .iter()
        .filter(|diffs| diffs.iter().all(|d| *d < 0) || diffs.iter().all(|d| *d > 0))
        .filter(|diffs| diffs.iter().all(|diff| (1..=3).contains(&diff.abs())))
        .collect::<Vec<_>>().len();

    println!("{res:?}");
}
