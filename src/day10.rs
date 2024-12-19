use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn get_neighbours(&self, grid: &[Vec<isize>]) -> Vec<Point> {
        let mut res = Vec::new();
        let directions: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (dx, dy) in directions.iter() {
            let nx = self.x + dx;
            let ny = self.y + dy;
            if nx >= 0
                && nx < grid[0].len() as isize
                && ny >= 0
                && ny < grid.len() as isize
                && grid[ny as usize][nx as usize] == grid[self.y as usize][self.x as usize] + 1
            {
                res.push(Point::new(nx, ny));
            }
        }
        res
    }
}

fn parse_day10(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).expect("deberia ser un digito") as isize)
                .collect()
        })
        .collect()
}

fn get_inicio(grid: &[Vec<isize>]) -> Vec<Point> {
    let mut res = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == 0 {
                res.push(Point::new(x as isize, y as isize));
            }
        }
    }
    res
}

fn get_score_part1(grid: &[Vec<isize>], point: Point) -> isize {
    let mut stack = vec![point];
    let mut count = 0;
    let mut vistos = HashSet::new();
    while let Some(actual) = stack.pop() {
        if grid[actual.y as usize][actual.x as usize] == 9 {
            count += 1;
        } else {
            let neighbours = actual.get_neighbours(grid);
            for n in neighbours.iter() {
                if !vistos.contains(n) {
                    stack.push(*n);
                    vistos.insert(*n);
                }
            }
        }
    }
    count
}

fn get_score_part2(grid: &[Vec<isize>], point: Point) -> isize {
    let mut stack = vec![point];
    let mut count = 0;
    while let Some(actual) = stack.pop() {
        if grid[actual.y as usize][actual.x as usize] == 9 {
            count += 1;
        } else {
            let mut neighbours = actual.get_neighbours(grid);
            stack.append(&mut neighbours);
        }
    }
    count
}

#[aoc(day10, part1)]
fn day10_part1(input: &str) -> isize {
    let grid = parse_day10(input);
    let inicio = get_inicio(&grid);
    inicio.iter().map(|&p| get_score_part1(&grid, p)).sum()
}

#[aoc(day10, part2)]
fn day10_part2(input: &str) -> isize {
    let grid = parse_day10(input);
    let inicio = get_inicio(&grid);
    inicio.iter().map(|&p| get_score_part2(&grid, p)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day10_part1() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(day10_part1(input), 36);
    }

    #[test]
    fn test_day10_part2() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(day10_part2(input), 81);
    }
}
