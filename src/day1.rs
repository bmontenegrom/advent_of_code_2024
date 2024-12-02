use std::collections::HashMap;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    for line in input.lines() {
        let mut iter = line.split_whitespace();
        left.push(
            iter.next()
                .expect("deberia haber un numero")
                .parse()
                .expect("deberia ser un numero"),
        );
        right.push(
            iter.next()
                .expect("deberia haber un numero")
                .parse()
                .expect("deberia ser un numero"),
        );
    }
    left.sort();
    right.sort();
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum::<u32>()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let mut left = vec![];
    let mut right: HashMap<u32, u32> = HashMap::new();

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        left.push(
            iter.next()
                .expect("debe haber un numero")
                .parse()
                .expect("deberia ser un numero"),
        );
        right
            .entry(
                iter.next()
                    .expect("deberia haber un numero")
                    .parse()
                    .expect("deberia ser un numero"),
            )
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }
    left.iter()
        .map(|l| l * right.get(l).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(part1(input), 11)
    }
}

#[test]
fn test2() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3";
    assert_eq!(part2(input), 31)
}
