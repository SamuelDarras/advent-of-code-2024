use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

/// Example:
/// ```
/// 1|2
/// 2|3
/// 1|4
/// 4|5
/// 3|5
/// 
/// 4,3,2,1
/// ```
/// 
/// - Make the rule set:
///     rs = {}
///     for each rule:
///         rs += {rule.page: {rule.constraint}}
///
/// ```
/// {1: {2, 4}, 2: {3}, 3: {4}, 4: {5}}
/// ```
/// 
/// - Filter the incorrect orderings:
///     incorrect = []
///     for oredering in orederings
///         for (page, next_page) in ordering.windows(2):
///             if page !E rs or next_page !E rs[page]:
///                 incorrect += ordering
///                 break
/// 
/// - Sort the orderings:
///     sorted_orderings = []
///     for ordering in orderings:
///         stripped_rs = 0
///         for k, set in rs:
///             stripped_rs += {k: set N ordering}
/// 
///         sorted = []
///         while stripped_rs != 0:
///             k = find(k: {}) // Find the key of an empty set
///             stripped_rs /= k
///             for k', set in stripped_rs:
///                 stripped_rs[k'] /= k
///             sorted += k
///         sorted_orderings += sorted
///
/// - Take the middle page and sum them:
///     result = 0
///     for sorted_ordering in sorted_orderings:
///         result += sorted_ordering[|sorted_ordering| / 2]

type Page = usize;

#[derive(Debug)]
struct Rules {
    rules: HashMap<Page, HashSet<Page>>,
}
impl Rules {
    fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    fn insert(&mut self, page: Page, before: Page) {
        self.rules
            .entry(page)
            .or_insert(HashSet::new())
            .insert(before);
    }

    fn contains(&self, key: &Page) -> bool {
        self.rules.keys().any(|k| k == key)
    }

    fn get(&mut self, key: &Page) -> &HashSet<Page> {
        self.rules.entry(*key).or_default()
    }

    fn striped(&self, keep: &Vec<Page>) -> Self {
        let keep_set = HashSet::from_iter(keep.iter().cloned());
        let mut new_map = self
            .rules
            .iter()
            .filter(|(k, _)| keep_set.contains(k))
            .map(|(key, set)| {
                (
                    *key,
                    set.intersection(&keep_set)
                        .cloned()
                        .collect::<HashSet<Page>>(),
                )
            })
            .collect::<HashMap<Page, HashSet<Page>>>();

        for k in keep {
            new_map.entry(*k).or_insert(HashSet::new());
        }

        Self { rules: new_map }
    }

    fn strip(&mut self, value: &Page) {
        self.rules.remove_entry(value);
        self.rules.values_mut().for_each(|set| {
            set.remove(value);
        });
    }

    fn find_least_constrain(&self) -> Page {
        self.rules
            .iter()
            .find(|(_, value)| value.is_empty())
            .unwrap()
            .0
            .clone()
    }
}
impl FromStr for Rules {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut this = Self::new();
        match s
            .lines()
            .map(|l| {
                let mut pages = l
                    .trim()
                    .split('|')
                    .map(str::parse::<Page>)
                    .map(Result::unwrap);
                this.insert(pages.next()?, pages.next()?);
                Some(())
            })
            .all(|o| o.is_some())
        {
            true => Ok(this),
            false => Err(()),
        }
    }
}

fn main() {
    let src = include_str!("src2.txt");
    //     let src = "1|2
    // 2|3
    // 1|4
    // 3|4
    // 4|5

    // 4,3,2,1";

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

    let split_src = src.split("\r\n\r\n").collect::<Vec<_>>();
    let mut rules = str::parse::<Rules>(split_src[0]).unwrap();
    let orders = split_src[1]
        .lines()
        .map(|l| {
            l.split(',')
                .map(str::parse::<Page>)
                .map(Result::unwrap)
                .collect::<Vec<Page>>()
        })
        .collect::<Vec<Vec<_>>>();

    let incorrect_orders = filter_orders(&orders, &mut rules);

    let mut result = 0;
    for order in incorrect_orders {
        let sorted_order = sort_order(order, &rules);
        result += sorted_order[sorted_order.len() / 2];
    }
    println!("{result}");
}

fn filter_orders<'a>(orders: &'a Vec<Vec<Page>>, rules: &mut Rules) -> Vec<&'a Vec<Page>> {
    orders
        .iter()
        .filter(|order| {
            order.windows(2).any(|window| {
                let page = window[0];
                let next = window[1];
                !rules.contains(&page) || !rules.get(&page).contains(&next)
            })
        })
        .collect::<Vec<_>>()
}

fn sort_order(order: &Vec<Page>, rules: &Rules) -> Vec<Page> {
    let mut stripped_rules = rules.striped(order);

    let mut result = Vec::new();
    while stripped_rules.rules.len() > 0 {
        let least_constrained = stripped_rules.find_least_constrain();
        stripped_rules.strip(&least_constrained);
        result.push(least_constrained);
    }
    result
}
