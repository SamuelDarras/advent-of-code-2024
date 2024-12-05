use std::collections::{HashMap, HashSet};

fn main() {
    let src = include_str!("src1.txt");
    //     let src = "47|53
    // 97|13
    // 97|61
    // 97|47
    // 75|29
    // 61|13
    // 75|53
    // 29|13
    // 97|29
    // 53|29
    // 61|53
    // 97|53
    // 61|29
    // 47|13
    // 75|47
    // 97|75
    // 47|61
    // 75|61
    // 47|29
    // 75|13
    // 53|13

    // 75,47,61,53,29
    // 97,61,53,29,13
    // 75,29,13
    // 75,97,47,61,53
    // 61,13,29
    // 97,13,75,29,47";

    let src = src
        .split("\r\n\r\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let rules_src = &src[0];
    let orders_src = &src[1];

    let mut rules: HashMap<usize, HashSet<usize>> = HashMap::new();
    rules_src.lines().for_each(|rule| {
        if let [page, before_page, ..] = rule
            .split("|")
            .map(str::parse::<usize>)
            .map(Result::unwrap)
            .collect::<Vec<usize>>()[..]
        {
            let mut set = rules.get(&page).map_or(HashSet::new(), |v| v.to_owned());
            set.insert(before_page);
            rules.insert(page, set);
        }
    });

    let orders = orders_src
        .lines()
        .map(|s| {
            s.split(',')
                .map(str::parse::<usize>)
                .map(Result::unwrap)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();

    let result = orders
        .iter()
        .filter(|order| {
            order
                .windows(2)
                .all(|s| rules.get(&s[0]).map_or(false, |set| set.contains(&s[1])))
        })
        .map(|correct| correct[correct.len() / 2])
        .sum::<usize>();

    println!("{result}");
}
