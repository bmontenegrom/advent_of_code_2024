

fn parse_day9_input(input: &str) ->Vec<Option<usize>>{
    input.chars().map(|c| c.to_digit(10).expect("deberia ser digito")).enumerate().fold(Vec::new(), |mut acc, (i, d)| {
        if i % 2 == 0{
            for _ in 0..d{
                acc.push(Some(i /2));
            }
        }
        else{
            for _ in 0..d{
                acc.push(None);
            }
        }
        acc
    })
}



#[aoc(day9, part1)]
fn day9_part1(input: &str) -> usize {
    let mut disco = parse_day9_input(input);
    let mut left = 0;
    let mut right = disco.len() - 1;
    while left < right {
        while disco[left].is_some() {
            left += 1;
        }
        while disco[right].is_none() {
            right -= 1;
        }
        if left < right {
            disco.swap(left, right);
        }
    }
    disco.iter().enumerate().filter_map(|(i, d)| {
        match d {
            Some(x) => Some(*x * i),
            None => None,
        }
    }).sum()

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day9_part1() {
        let input = "2333133121414131402";
        assert_eq!(day9_part1(input), 1928);
    }
}