use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

fn main() {
    let src = include_str!("src2.txt");
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

    //     let src = "1|2
    // 2|3
    // 1|4
    // 3|4
    // 4|5

    // 4,3,2,1";

    let src = src
        .split("\r\n\r\n")
        // .split("\n\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let rules_src = &src[0];
    let orders_src = &src[1];

    let mut rules: HashMap<usize, HashSet<usize>> = HashMap::new();
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

    let mut incorrects = Vec::new();
    for order in orders {
        if order.windows(2).any(|s| {
            let left = s[0];
            let right = s[1];

            match rules.get(&left) {
                Some(set) => !set.contains(&right),
                None => true,
            }
        }) {
            incorrects.push(order);
        }
    }

    let mut result = 0;
    for incorrect in incorrects {
        // println!("Rules: {rules:?}");
        let mut new_rules = HashMap::<usize, HashSet<usize>>::new();
        let interest: HashSet<usize> = HashSet::from_iter(incorrect.into_iter());
        for i in interest.iter() {
            match rules.get(&i) {
                Some(set) => {
                    let new_rule = set
                        .intersection(&interest)
                        .map(|u| *u)
                        .collect::<HashSet<usize>>();
                    new_rules.insert(*i, new_rule);
                }
                None => {
                    new_rules.insert(*i, HashSet::new());
                }
            }
        }
        // println!("Interest: {interest:?}");
        // println!("New rules: {new_rules:?}");

        let mut sorted = Vec::new();
        while !new_rules.is_empty() {
            let least_constrained = match new_rules.iter().find(|(_, s)| s.is_empty()) {
                Some((v, _)) => Some(v.clone()),
                None => Some(*new_rules.keys().last().unwrap()),
            };
            if let Some(least_constrained) = least_constrained {
                sorted.push(least_constrained);
                remove_v(&mut new_rules, &least_constrained);
            }
            // println!("{new_rules:?}");
            // println!("{least_constrained:?}");
        }
        println!("{sorted:?}");
        result += sorted[sorted.len()/2];
    }

    println!("{result}");
}

fn remove_v<'a>(
    map: &'a mut HashMap<usize, HashSet<usize>>,
    v: &usize,
) -> &'a HashMap<usize, HashSet<usize>> {
    map.remove(v);
    for k in map.clone().keys() {
        map.entry(*k).and_modify(|e| {
            e.remove(v);
        });
    }
    map
}
