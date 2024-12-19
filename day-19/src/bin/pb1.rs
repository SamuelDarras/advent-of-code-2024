fn main() {
    let src = include_str!("src1.txt");

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
    let mut rules = lines.next().unwrap().split(", ").collect::<Vec<_>>();
    rules.sort_by(|a, b| b.len().cmp(&a.len()));

    let patterns = lines.skip(1).collect::<Vec<_>>();
    // println!("{patterns:?}");

    let result = patterns
        .iter()
        .filter(|pattern| greedy(&rules, &pattern))
        .count();
    println!("{result:?}");
}

fn greedy(rules: &Vec<&str>, pattern: &str) -> bool {
    if pattern.len() == 0 {
        true
    } else {
        rules.iter().any(|rule| {
            // println!(
            //     "{pattern} | {rule} ; {} =?= {}",
            //     if pattern.len() >= rule.len() {
            //         pattern[pattern.len() - rule.len()..].to_string()
            //     } else {
            //         "---".to_string()
            //     },
            //     rule.to_string()
            // );
            pattern.len() >= rule.len()
                && pattern[pattern.len() - rule.len()..] == **rule
                && greedy(rules, &pattern[0..(pattern.len() - rule.len())])
        })
    }
}
