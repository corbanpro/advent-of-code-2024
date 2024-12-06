use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
pub struct Rule {
    pub before: Vec<i32>,
    pub after: Vec<i32>,
}

impl Rule {
    fn new() -> Rule {
        Rule {
            before: vec![],
            after: vec![],
        }
    }
}

fn main() {
    let raw_rules_and_updates = fs::read_to_string("../../assets/rules.txt").unwrap();
    let rules_and_updates: Vec<&str> = raw_rules_and_updates.trim().split("\n\n").collect();
    let rules = rules_and_updates[0]
        .split("\n")
        .map(|rule| {
            rule.split("|")
                .map(|page| page.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    let updates = rules_and_updates[1]
        .split("\n")
        .map(|rule| {
            rule.split(",")
                .map(|page| page.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let processed_rules = process_rules(rules);

    let filtered_updates = filter_updates(processed_rules, updates);

    println!("{:#?}", filtered_updates);

    let result = add_middle_pages(filtered_updates);

    println!("{result}")
}

fn process_rules(raw_rules: Vec<Vec<i32>>) -> HashMap<i32, Rule> {
    let mut processed_rules: HashMap<i32, Rule> = HashMap::new();

    for rule in raw_rules {
        let before_page_rule = processed_rules.entry(rule[0]).or_insert(Rule::new());
        before_page_rule.after.push(rule[1]);

        let after_page_rule = processed_rules.entry(rule[1]).or_insert(Rule::new());
        after_page_rule.before.push(rule[0]);
    }

    processed_rules
}

fn filter_updates(rules: HashMap<i32, Rule>, updates: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut filtered_updates: Vec<Vec<i32>> = vec![];
    for update in updates {
        let mut update_valid = true;
        for (index, page) in update.iter().enumerate() {
            let pages_before = &update[0..index];
            let pages_after = &update[index + 1..];

            for before_page in pages_before {
                if rules.get(page).unwrap().after.contains(before_page) {
                    update_valid = false;
                }
            }

            for after_page in pages_after {
                if rules.get(page).unwrap().before.contains(after_page) {
                    update_valid = false;
                }
            }
        }
        if update_valid {
            filtered_updates.push(update)
        }
    }
    filtered_updates
}

fn add_middle_pages(updates: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    for update in updates {
        let middle_update = update.get((update.len()) / 2).unwrap();
        println!("{}", middle_update);
        result += middle_update
    }
    result
}
