#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
    let mut safe = 0;
    for line in input.lines() {
        let report: Vec<u32> = line
            .split(' ')
            .map(|s| s.parse::<u32>().expect("deberia ser un numero"))
            .collect();
        if check_part1(&report) {
            safe += 1;
        }
    }
    safe
}

fn check_part1(report: &[u32]) -> bool {
    report
        .iter()
        .zip(report.iter().skip(1))
        .all(|(&l, &r)| l < r && r - l >= 1 && r - l <= 3)
        || report
            .iter()
            .zip(report.iter().skip(1))
            .all(|(&l, &r)| l > r && l - r >= 1 && l - r <= 3)
}

fn check_part2(report: &[u32]) -> bool {
    if check_part1(report) {
        return true;
    } else {
        for i in 0..report.len() {
            let mut report_clone = report.to_owned();
            report_clone.remove(i);
            if check_part1(&report_clone) {
                return true;
            }
        }
    }
    false
}
#[aoc(day2, part2)]
pub fn part2(input: &str) -> u32 {
    let mut safe = 0;
    for line in input.lines() {
        let report: Vec<u32> = line
            .split(' ')
            .map(|s| s.parse::<u32>().expect("deberia ser un numero"))
            .collect();
        if check_part2(&report) {
            safe += 1;
        }
    }
    safe
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_day2_part1() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(part1(input), 2)
    }
    #[test]
    fn test_day2_part2() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(part2(input), 4);
    }
}
