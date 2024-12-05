use std::collections::{HashMap, HashSet};

pub fn run_day05_p1(input: &str) {
    let (page_ordering_rules_str, pages_str) = input.split_once("\n\n").unwrap();

    let mut page_ordering_rules = HashMap::new();
    page_ordering_rules_str.lines().for_each(|l| {
        let (k, v) = l.split_once("|").unwrap();
        let key = k.parse::<u8>().unwrap();
        let value = v.parse::<u8>().unwrap();
        page_ordering_rules
            .entry(key)
            .and_modify(|e: &mut HashSet<u8>| {
                e.insert(value);
            })
            .or_insert(HashSet::<u8>::from([value]));
    });

    let updates: Vec<Vec<u8>> = pages_str
        .lines()
        .map(|l| l.split(',').map(|i| i.parse::<u8>().unwrap()).collect())
        .collect();

    let mut answer = 0u32;
    for page in &updates {
        let mut correct_order = false;
        for i in 0..page.len() {
            let eval_page = page[i];
            let pages_before = &page[..i];
            if let Some(spec_page_order_rules) = page_ordering_rules.get(&eval_page) {
                correct_order = pages_before
                    .iter()
                    .all(|p| !spec_page_order_rules.contains(p));
            }

            if !correct_order {
                break;
            }
        }

        if correct_order {
            let middle_i = page.len() / 2;
            answer += page[middle_i] as u32;
        }
    }

    println!("{}", answer);
}

pub fn run_day05_p2(input: &str) {
    let (page_ordering_rules_str, pages_str) = input.split_once("\n\n").unwrap();

    let mut page_ordering_rules: HashMap<u8, HashSet<u8>> = HashMap::new();
    page_ordering_rules_str.lines().for_each(|l| {
        let (k, v) = l.split_once("|").unwrap();
        let key = k.parse::<u8>().unwrap();
        let value = v.parse::<u8>().unwrap();
        page_ordering_rules
            .entry(key)
            .and_modify(|e: &mut HashSet<u8>| {
                e.insert(value);
            })
            .or_insert(HashSet::<u8>::from([value]));
    });

    let mut updates: Vec<Vec<u8>> = pages_str
        .lines()
        .map(|l| l.split(',').map(|i| i.parse::<u8>().unwrap()).collect())
        .collect();

    let mut answer = 0u32;
    for page in &mut updates {
        let mut correct_order = false;
        for i in 0..page.len() {
            let eval_page = &page[i];
            let pages_before = &page[..i];
            if let Some(spec_page_order_rules) = page_ordering_rules.get(&eval_page) {
                pages_before.iter().enumerate().for_each(|(j, p)| {
                    if spec_page_order_rules.contains(p) {
                        page.swap(i, j);
                    }
                });
            }
        }

        if !correct_order {
            let middle_i = page.len() / 2;
            answer += page[middle_i] as u32;
        }
    }

    println!("{}", answer);
}

fn check_page(page: &Vec<u8>, page_ordering_rules: &HashMap<u8, HashSet<u8>>) -> bool {
    let mut correct_order = false;
    for i in 0..page.len() {
        let eval_page = page[i];
        let pages_before = &page[..i];
        if let Some(spec_page_order_rules) = page_ordering_rules.get(&eval_page) {
            correct_order = pages_before
                .iter()
                .all(|p| !spec_page_order_rules.contains(p));
        }

        if !correct_order {
            break;
        }
    }
    correct_order
}

pub const TESTD5: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
