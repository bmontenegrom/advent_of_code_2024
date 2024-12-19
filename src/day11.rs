use std::collections::VecDeque;

fn parse_day11_input(input: &str) -> VecDeque<u64> {
    input
        .split(' ')
        .map(|x| x.parse().expect("deberia ser un numero"))
        .collect()
}





#[aoc(day11, part1)]
fn day11_part1(input: &str) -> usize{
    let mut stones = parse_day11_input(input);
    for _ in 0..25 {
        let mut new_stones = VecDeque::new();
        while let Some(stone) = stones.pop_front() {
            if stone == 0{
                new_stones.push_back(1);
            } else if stone.to_string().len() % 2 == 0 {
                let first = stone.to_string().get(0..stone.to_string().len()/2).unwrap().parse().unwrap();
                let second = stone.to_string().get(stone.to_string().len()/2..).unwrap().parse().unwrap();
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
fn day11_part2(input: &str)-> usize{
    let mut stones = parse_day11_input(input);
    for _ in 0..75 {
        let mut new_stones = VecDeque::new();
        while let Some(stone) = stones.pop_front() {
            if stone == 0{
                new_stones.push_back(1);
            } else if stone.to_string().len() % 2 == 0 {
                let first = stone.to_string().get(0..stone.to_string().len()/2).unwrap().parse().unwrap();
                let second = stone.to_string().get(stone.to_string().len()/2..).unwrap().parse().unwrap();
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day11_part1() {
        let input = "125 17";
        assert_eq!(day11_part1(input), 55312);
    }
}