use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
struct Manual {
    first: u32,
    second: u32,
}

impl Manual {
    fn new(first: u32, second: u32) -> Manual {
        Manual { first, second }
    }
}

fn parse_day5(input: &str) -> (Vec<Manual>, Vec<Vec<u32>>) {
    let (rules, pages) = input
        .split_once("\n\n")
        .expect("deberia haber reglas y paginas");
    let rules: Vec<Manual> = rules
        .lines()
        .map(|l| {
            let (first, second) = l
                .split_once("|")
                .expect("deberia haber numeros separados por |");
            Manual::new(
                first.parse().expect("deberia ser un numero"),
                second.parse().expect("deberia se run numero"),
            )
        })
        .collect();

    let updates: Vec<Vec<u32>> = pages
        .lines()
        .map(|l| {
            let order: Vec<u32> = l
                .split(",")
                .map(|c| c.parse::<u32>().expect("la pagina deberia ser un numero"))
                .collect();
            order
        })
        .collect();
    (rules, updates)
}

fn check_update(rules: &[Manual], update: &[u32]) -> bool {
    let mut map = HashMap::new();
    update.iter().enumerate().for_each(|(i, page)| {
        map.insert(page, i);
    });
    rules.iter().all(|rule| {
        if let Some(index_first) = map.get(&rule.first) {
            if let Some(index_second) = map.get(&rule.second) {
                index_first < index_second
            } else {
                true
            }
        } else {
            true
        }
    })
}

fn correct_update(rules: &[Manual], update: &[u32]) -> Vec<u32> {
    let mut map = HashMap::new();
    update.iter().enumerate().for_each(|(i, page)| {
        map.insert(page, i);
    });
    let mut correct = false;
    while !correct {
        correct = rules.iter().all(|rule| {
            if let Some(&index_first) = map.get(&rule.first) {
                if let Some(&index_second) = map.get(&rule.second) {
                    if index_first >= index_second {
                        map.entry(&rule.first).and_modify(|e| *e = index_second);
                        map.entry(&rule.second).and_modify(|e| *e = index_first);
                    }
                    false
                } else {
                    true
                }
            } else {
                true
            }
        })
    }
    let mut res: Vec<u32> = vec![0; update.len()];
    for (key, value) in map {
        res[value] = *key;
    }
    res
}

#[aoc(day5, part1)]
fn part1(input: &str) -> u32 {
    let (rules, updates) = parse_day5(input);
    updates.iter().fold(0, |mut acc, update| {
        let correct = check_update(&rules, update);
        if correct {
            acc += update
                .get(update.len() / 2)
                .expect("deberia haber un numero en el medio");
        }
        acc
    })
}

#[aoc(day5, part2)]
fn part2(input: &str) -> u32 {
    let (rules, updates) = parse_day5(input);
    updates.iter().fold(0, |mut acc, update| {
        if !check_update(&rules, update) {
            let new_update = correct_update(&rules, update);
            acc += *new_update
                .get(new_update.len() / 2)
                .expect("debe existir un numero en el medio");
        } 
        acc
    })
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parser_gay5() {
        let input = "47|53
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
        let (rules, pages) = parse_day5(input);
        for rule in rules {
            println!("firsts: {} second : {}", rule.first, rule.second);
        }

        for vec in pages {
            for page in vec {
                print!(" {}", page);
            }
            println!();
        }
    }

    #[test]
    fn test_day5_part1() {
        let input = "47|53
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
        assert_eq!(part1(input), 143);
    }


    #[test]
    fn test_day5_part2() {
        let input = "47|53
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
        assert_eq!(part2(input), 123);
    }


}
