#[derive(Debug, PartialEq, Eq)]
enum FType{
    File,
    Free,
}

#[derive(Debug, PartialEq, Eq)]
struct Block{
    ftype: FType,
    id: usize,
    used: usize,
    free: usize,
    data: Vec<usize>,
}

impl Block {
    fn new(index: usize, size: usize) -> Block {
        let ftype = if index % 2 == 0 {
            FType::File
        } else {
            FType::Free
        };
        let id = index/2;
        if ftype == FType::File {
            Block {
                ftype: ftype,
                id: id,
                used: size,
                free: 0,
                data: vec![id; size],
            }
        } else {
            Block {
                ftype: ftype,
                id: id,
                used: 0,
                free: size,
                data: Vec::new(),
            }
        }
    }
}


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

fn day9_part2(input: &str)->usize{
    let mut disco = parse_day9_part2(input);
    let mut right = disco.len() - 1;
    while right > 0 {
        while disco[right].ftype == FType::Free {
            right -= 1;            
        }
        let mut left = 0_usize;
        while left < right && (disco[left].ftype == FType::File || (disco[left].ftype == FType::Free  && disco[left].free < disco[right].used)) {
            left += 1;
        }
        if left < right {
            let right_data = &mut disco[right].data.clone();
            disco[left].data.append(right_data);
            disco[left].used += disco[right].used;
            disco[left].free -= disco[right].used;
            if disco[left].free == 0 {
                disco[left].ftype = FType::File;
            }
            disco[right].ftype = FType::Free;
            disco[right].used = 0;
            disco[right].data.clear();
        }
    }
    
    todo!()
}



fn parse_day9_part2(input: &str)-> Vec<Block>{
    input.chars().map(|c| c.to_digit(10).expect("deberia ser digito")).enumerate().map(|(i, d)| {
        Block::new(i, d as usize)
    }).collect()
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

    #[test]
    fn test_parse_part2(){
        let input =  "2333133121414131402";
        println!("{:?}", parse_day9_part2(input));
    }
}