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

    let mut rules: HashMap<usize, HashSet<usize>> = HashMap::new();
    let src = src
        .split("\r\n\r\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let rules_src = &src[0];
    let orders_src = &src[1];

    for rule in rules_src.lines() {
        if let [page, before_page, ..] = rule
            .split("|")
            .map(str::parse::<usize>)
            .map(Result::unwrap)
            .collect::<Vec<usize>>()[..]
        {
            match rules.get(&page) {
                Some(before) => {
                    let mut set = before.to_owned();
                    set.insert(before_page);
                    rules.insert(page, set);
                }
                None => {
                    let mut set = HashSet::new();
                    set.insert(before_page);
                    rules.insert(page, set);
                }
            }
        }
    }

    let orders = orders_src
        .lines()
        .map(|s| {
            s.split(',')
                .map(str::parse::<usize>)
                .map(Result::unwrap)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();

    let mut corrects = Vec::new();
    for order in orders {
        if order.windows(2).all(|s| {
            let left = s[0];
            let right = s[1];

            match rules.get(&left) {
                Some(set) => set.contains(&right),
                None => false,
            }
        }) {
            corrects.push(order);
        }
    }

    let result = corrects
        .iter()
        .map(|correct| correct[correct.len() / 2])
        .sum::<usize>();

    println!("{result}");
}
