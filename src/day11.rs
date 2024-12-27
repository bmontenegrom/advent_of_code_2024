use std::collections::{HashMap, VecDeque};

use cached::proc_macro::cached;

fn parse_day11_input(input: &str) -> VecDeque<u64> {
    input
        .split(' ')
        .map(|x| x.parse().expect("deberia ser un numero"))
        .collect()
}

#[aoc(day11, part1)]
fn day11_part1(input: &str) -> usize {
    let mut stones = parse_day11_input(input);
    for _ in 0..25 {
        let mut new_stones = VecDeque::new();
        while let Some(stone) = stones.pop_front() {
            if stone == 0 {
                new_stones.push_back(1);
            } else if stone.to_string().len() % 2 == 0 {
                let first = stone
                    .to_string()
                    .get(0..stone.to_string().len() / 2)
                    .unwrap()
                    .parse()
                    .unwrap();
                let second = stone
                    .to_string()
                    .get(stone.to_string().len() / 2..)
                    .unwrap()
                    .parse()
                    .unwrap();
                new_stones.push_back(first);
                new_stones.push_back(second);
            } else {
                new_stones.push_back(stone * 2024);
            }
        }
        stones = new_stones;
    }
    stones.len()
}

#[aoc(day11, part2)]
fn day11_part2(input: &str) -> u64 {
    let stones = parse_day11_input(input);
    let mut cache: HashMap<u64, u64> = HashMap::default();
    for num in stones.iter() {
        cache.entry(*num).and_modify(|e| *e += 1).or_insert(1);
    }
    for _ in 0..75 {
        let mut new_cache: HashMap<u64, u64> = HashMap::default();
        for (num, count) in cache.iter() {
            match num {
                0 => {
                    new_cache
                        .entry(1)
                        .and_modify(|e| *e += count)
                        .or_insert(*count);
                }
                n if n.to_string().len() % 2 == 0 => {
                    let first = n
                        .to_string()
                        .get(0..n.to_string().len() / 2)
                        .unwrap()
                        .parse()
                        .unwrap();
                    let second = n
                        .to_string()
                        .get(n.to_string().len() / 2..)
                        .unwrap()
                        .parse()
                        .unwrap();
                    new_cache
                        .entry(first)
                        .and_modify(|e| *e += count)
                        .or_insert(*count);
                    new_cache
                        .entry(second)
                        .and_modify(|e| *e += count)
                        .or_insert(*count);
                }
                n => {
                    new_cache
                        .entry(n * 2024)
                        .and_modify(|e| *e += count)
                        .or_insert(*count);
                }
            }
        }
        cache = new_cache;
    }
    cache.values().sum::<u64>()
}

#[aoc(day11, part2, recursive)]
fn day11_part2_rec(input: &str) -> u64 {
    let stones = parse_day11_input(input);
    stones.iter().map(|x| blinks(*x, 75)).sum()
}

#[cached]
fn blinks(num: u64, remaining: u64 ) -> u64 {
    if remaining == 0{
        return 1;
    }

    match num {
        0 => blinks(1, remaining - 1),
        n if n.to_string().len() % 2 == 0 => {
            let first = n
                .to_string()
                .get(0..n.to_string().len() / 2)
                .unwrap()
                .parse()
                .unwrap();
            let second = n
                .to_string()
                .get(n.to_string().len() / 2..)
                .unwrap()
                .parse()
                .unwrap();
            blinks(first, remaining - 1) + blinks(second, remaining - 1)
        }
        n => blinks(n * 2024, remaining - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day11_part1() {
        let input = "125 17";
        assert_eq!(day11_part1(input), 55312);
    }
}
