

const DIRECTIONS: [(i32, i32); 8] = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];


const MAS: [char; 3] = ['M', 'A', 'S'];

fn process_input(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<char>>) {
    let y_max = input.lines().count();
    let x_max = input.lines().last().unwrap().len();
    input.lines().enumerate().fold(
        (vec![], vec![vec![' '; x_max]; y_max]),
        |mut acc, (y, s)| {
            s.chars().enumerate().for_each(|(x, c)| match c {
                'X' => {
                    acc.0.push((y, x));
                    acc.1[y][x] = c;
                }
                _ => acc.1[y][x] = c,
            });
            acc
        },
    )
}

fn check_diagonal1(coord: ( usize, usize), grid: &[Vec<char>]) -> bool{
    let (y , x) = coord;
    if let Some(fila) = grid.get(y+1) {
        if let Some(c) = fila.get(x+1) {
            match c {
                'S' => {
                    if x > 0 && y > 0{
                        return *grid.get(y-1).expect("deberia existir la fila").get(x-1).expect("deberia existir el caracter") == 'M' ;
                    } else {
                        return false
                    }
                }
                'M' => {
                    if x > 0 && y > 0{
                        return *grid.get(y-1).expect("deberia existir la fila").get(x-1).expect("deberia existir el caracter") == 'S' ;
                    } else {
                        return false;
                    }
                }
                _=> return false,
            }
        }
    }
 false
}

fn check_diagonal2(coord: ( usize, usize), grid: &[Vec<char>]) -> bool{
    let (y , x) = coord;
    if y > 0{
        if let Some(c) = grid.get(y-1).expect("deberia existir la fila").get(x+1){
            match c {
                'S' =>{
                    if let Some(fila) = grid.get(y+1) {
                        if x > 0{
                            return *fila.get(x-1).expect("deberia existir el char") == 'M';
                        } else {
                            return false;
                        }
                    }
                }
                'M' => {
                    if let Some(fila) = grid.get(y+1) {
                        if x > 0{
                            return *fila.get(x-1).expect("deberia existir el char") == 'S';
                        } else {
                            return false;
                        }
                    }
                }
                _=> return false,
            }
        }
    }
    false
}


fn process_input_part2(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<char>>) {
    let y_max = input.lines().count();
    let x_max = input.lines().last().unwrap().len();
    input.lines().enumerate().fold(
        (vec![], vec![vec![' '; x_max]; y_max]),
        |mut acc, (y, s)| {
            s.chars().enumerate().for_each(|(x, c)| match c {
                'A' => {
                    acc.0.push((y, x));
                    acc.1[y][x] = c;
                }
                _ => acc.1[y][x] = c,
            });
            acc
        },
    )
}

#[aoc(day4, part1)]
fn part1(input: &str) -> u32 {
    let (x_pos, grid) = process_input(input);
    x_pos.iter().fold(0, |mut acc, (y, x)| {
        DIRECTIONS.iter().for_each(|d| {
            let mut y_d = *y as i32;
            let mut x_d = *x as i32;
            let mut count = 0;
            for item in MAS {
                y_d += d.0;
                x_d += d.1;
                if x_d >= 0 && y_d >= 0 {
                    if let Some(fila) = grid.get(y_d as usize) {
                        if let Some(c) = fila.get(x_d as usize) {
                            if *c == item {
                                count += 1;
                            }
                        }
                    }
                }
            }
            if count == 3 {
                acc += 1;
            }
        });
        acc
    })
}



#[aoc(day4, part2)]
fn part2(input: &str)->u32{
    let (pos_a, grid) = process_input_part2(input);
    pos_a.iter().fold(0, |mut acc, (y, x)|{
        if check_diagonal1((*y, *x), &grid) && check_diagonal2((*y, *x), &grid){
            acc +=1;
        }
        acc
    })
 
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_day4_part1() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(part1(input), 18);
    }

    #[test]
    fn test_day4_part2(){
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(part2(input), 9);
    }
}
