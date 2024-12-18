#[derive(Debug, PartialEq, Eq)]
enum FType {
    File,
    Free,
}

#[derive(Debug, PartialEq, Eq)]
struct Block {
    ftype: FType,
    free: usize,
    data: Vec<usize>,
}

impl Block {
    fn new(id: usize, size: usize) -> Self {
        if id % 2 == 0 {
            Block {
                ftype: FType::File,
                free: 0,
                data: vec![id / 2; size],
            }
        } else {
            Block {
                ftype: FType::Free,
                free: size,
                data: Vec::new(),
            }
        }
    }
}

fn parse_day9_part2(input: &str) -> Vec<Block> {
    input
        .chars()
        .map(|c| c.to_digit(10).expect("deberia ser digiyo") as usize)
        .enumerate()
        .map(|(i, d)| Block::new(i, d))
        .collect()
}

fn parse_day9_input(input: &str) -> Vec<Option<usize>> {
    input
        .chars()
        .map(|c| c.to_digit(10).expect("deberia ser digito"))
        .enumerate()
        .fold(Vec::new(), |mut acc, (i, d)| {
            if i % 2 == 0 {
                for _ in 0..d {
                    acc.push(Some(i / 2));
                }
            } else {
                for _ in 0..d {
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
    disco
        .iter()
        .enumerate()
        .filter_map(|(i, d)| d.as_ref().map(|x| *x * i))
        .sum()
}

#[aoc(day9, part2)]
fn day9_part2(input: &str) -> usize {
    let mut disco = parse_day9_part2(input);
    let mut right = disco.len() - 1;
    while right > 0 {
        while disco[right].ftype == FType::Free {
            right -= 1;
        }

        let mut left = 0;

        while left < right {
            if disco[left].ftype == FType::File
                || (disco[left].ftype == FType::Free && disco[left].free < disco[right].data.len())
            {
                left += 1;
                continue;
            }

            let mut aux = disco[right].data.clone();
            disco[left].data.append(&mut aux);
            disco[left].free -= disco[right].data.len();
            if disco[left].free == 0 {
                disco[left].ftype = FType::File;
            }
            disco[right].ftype = FType::Free;
            disco[right].free = disco[right].data.len();
            disco[right].data.clear();
            break;
        }
        right -= 1;
    }
    disco
        .iter()
        .flat_map(|b| {
            b.data
                .iter()
                .cloned()
                .chain(std::iter::repeat(0).take(b.free))
        })
        .enumerate()
        .map(|(i, d)| i * d)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day9_part1() {
        let input = "2333133121414131402";
        assert_eq!(day9_part1(input), 1928);
    }

    #[test]
    fn test_day9_part2() {
        let input = "2333133121414131402";
        assert_eq!(day9_part2(input), 2858);
    }
}
