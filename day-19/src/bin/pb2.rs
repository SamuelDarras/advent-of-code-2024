use memoize::memoize;
use std::time::Instant;

fn main() {
    let src = include_str!("src2.txt");

    //     let src = "r, wr, b, g, bwu, rb, gb, br
    //
    // brwrr
    // bggr
    // gbbr
    // rrbgbr
    // ubwu
    // bwurrg
    // brgr
    // bbrgwb
    // ";

    //     let src = "rgb, bwu
    //
    // rgbwu";

    let mut lines = src.lines();
    let mut rules = lines
        .next()
        .unwrap()
        .split(", ")
        .map(|rule| rule.to_string())
        .collect::<Vec<String>>();
    rules.sort_by(|a, b| b.len().cmp(&a.len()));

    let patterns = lines.skip(1).collect::<Vec<_>>();
    // println!("{patterns:?}");

    let now = Instant::now();
    let result = patterns
        .iter()
        .map(|pattern| greedy(rules.clone(), pattern.to_string()))
        .sum::<usize>();
    let elapsed = now.elapsed();
    println!("{result}, in {elapsed:?}");
}

#[memoize]
fn greedy(rules: Vec<String>, pattern: String) -> usize {
    if pattern.len() == 0 {
        0
    } else {
        rules
            .iter()
            .filter_map(|rule| {
                // println!(
                //     "{pattern} | {rule} ; {} =?= {}",
                //     if pattern.len() >= rule.len() {
                //         pattern[pattern.len() - rule.len()..].to_string()
                //     } else {
                //         "---".to_string()
                //     },
                //     rule.to_string()
                // );
                if *pattern == **rule {
                    Some(1)
                } else if pattern.len() >= rule.len()
                    && pattern[pattern.len() - rule.len()..] == **rule
                {
                    let r = greedy(
                        rules.clone(),
                        pattern[0..(pattern.len() - rule.len())].to_string(),
                    );
                    Some(r)
                } else {
                    None
                }
            })
            .sum::<usize>()
    }
}
