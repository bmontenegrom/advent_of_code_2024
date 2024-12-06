use std::collections::HashMap;


fn parse_day5(input: &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let (rules, pages) = input
        .split_once("\n\n")
        .expect("deberia haber reglas y paginas");
    let map =rules
        .lines()
        .fold(HashMap::new(), |mut acc, l| {
            let (first, second) = l
                .split_once("|")
                .expect("deberia haber numeros separados por |");
            acc.entry(first.parse::<u32>().expect("deberia ser un numero"))
                .or_insert_with(Vec::new)
                .push(second.parse::<u32>().expect("deberia ser un numero"));
            acc
        });

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
    (map, updates)
}

fn check_update(rules: &HashMap<u32, Vec<u32>>, update: &[u32]) -> bool {
    update.iter().is_sorted_by(|&a, &b|{
        rules.get(a).map_or(std::cmp::Ordering::Greater, |v| {
            if v.contains(b) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        }) == std::cmp::Ordering::Less
    })
}

fn correct_update(rules: &HashMap<u32, Vec<u32>>, update: &[u32]) -> Vec<u32>{
    let mut new_update = update.to_vec();
    new_update.sort_by(|a, b|{
        rules.get(a).map_or(std::cmp::Ordering::Greater, |v| {
            if v.contains(b) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        })
    });
    new_update
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
