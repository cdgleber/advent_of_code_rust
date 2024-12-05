use std::collections::{ HashMap, HashSet };

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
        .map(|l|
            l
                .split(',')
                .map(|i| i.parse::<u8>().unwrap())
                .collect()
        )
        .collect();

    let mut answer = 0u32;
    for page in &updates {
        let mut correct_order = false;
        for i in 0..page.len() {
            let eval_page = page[i];
            let pages_before = &page[..i];
            if let Some(spec_page_order_rules) = page_ordering_rules.get(&eval_page) {
                correct_order = pages_before.iter().all(|p| !spec_page_order_rules.contains(p));
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
    let (page_ordering_rules_str, pages_str) = input.split_once("\r\n\r\n").unwrap(); // f you windows \r\n\r\n for windows \n\n for unix

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

    let updates: Vec<Vec<u8>> = pages_str
        .lines()
        .map(|l|
            l
                .split(',')
                .map(|i| i.parse::<u8>().unwrap())
                .collect()
        )
        .collect();

    let mut answer = 0u32;
    for page_index in 0..updates.len() {
        let page = updates[page_index].clone();

        if !check_page(&updates[page_index], &page_ordering_rules) {
            //if false execute custom bubble sort
            let new_page = custom_sort(page, &page_ordering_rules);
            let middle_i = new_page.len() / 2;
            answer += new_page[middle_i] as u32;
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
            correct_order = pages_before.iter().all(|p| !spec_page_order_rules.contains(p));
        }

        if !correct_order {
            break;
        }
    }
    correct_order
}

fn custom_sort(mut page: Vec<u8>, page_ordering_rules: &HashMap<u8, HashSet<u8>>) -> Vec<u8> {
    let mut correct_order = false;

    for page_index in 0..page.len() {
        let temp = page.clone();
        let eval_page = temp[page_index];
        let pages_before = &temp[..page_index];
        if let Some(spec_page_order_rules) = page_ordering_rules.get(&eval_page) {
            for page_before_index in 0..pages_before.len() {
                if spec_page_order_rules.contains(&pages_before[page_before_index]) {
                    page.swap(page_before_index, page_index);
                    continue;
                }
                correct_order = true;
            }
        }
    }

    if correct_order {
        page
    } else {
        custom_sort(page, page_ordering_rules)
    }
}

pub const TESTD5: &str =
    "47|53
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
