use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}
impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }

    fn get_antinode(&self, other: &Point) -> (Point, Point) {
        let x_dif = self.x - other.x;
        let y_dif = self.y - other.y;
        (
            Point {
                x: other.x - x_dif,
                y: other.y - y_dif,
            },
            Point {
                x: self.x + x_dif,
                y: self.y + y_dif,
            },
        )
    }

    fn get_all_antinodes(&self, other: &Point, x_max: isize, y_max: isize) -> Vec<Point> {
        let x_dif = self.x - other.x;
        let y_dif = self.y - other.y;
        let mut res = vec![];
        let mut x = self.x;
        let mut y = self.y;
        while x < x_max && y < y_max && x >= 0 && y >= 0 {
            res.push(Point { x, y });
            x += x_dif;
            y += y_dif;
        }
        x = other.x;
        y = other.y;
        while x >= 0 && y >= 0 && x < x_max && y < y_max {
            res.push(Point { x, y });
            x -= x_dif;
            y -= y_dif;
        }
        res
    }
}

fn parse_day8(input: &str) -> (Vec<Vec<char>>, HashMap<char, Vec<Point>>) {
    let map = input
        .lines()
        .zip(0_isize..)
        .fold(HashMap::new(), |mut acc: HashMap<char, Vec<Point>>  , (l, i)| {
            l.chars().zip(0_isize..).for_each(|(c, j)| {
                if c != '.' {
                    acc.entry(c).or_default().push(Point::new(j, i));
                }
            });
            acc
        });
    let grid = input.lines().map(|l| l.chars().collect()).collect();
    (grid, map)
}

#[aoc(day8, part1)]
fn day8_part1(input: &str) -> u32 {
    let (grid, map) = parse_day8(input);
    let x_max = grid[0].len() as isize;
    let y_max = grid.len() as isize;
    let mut vistos = vec![];
    map.keys().fold(0, |mut acc, c| {
        let mut points = map.get(c).expect("deberia existir").clone();
        while let Some(actual)= points.pop() {
            points.iter().for_each(|p| {
                let (point_1, point_2) = actual.get_antinode(p);
                if point_1.x >= 0
                    && point_1.x < x_max
                    && point_1.y >= 0
                    && point_1.y < y_max
                    && !vistos.contains(&point_1)
                {
                    vistos.push(point_1);
                    acc += 1;
                }
                if point_2.x >= 0
                    && point_2.x < x_max
                    && point_2.y >= 0
                    && point_2.y < y_max
                    && !vistos.contains(&point_2)
                {
                    vistos.push(point_2);
                    acc += 1;
                }
            });
        }
        acc
    })
}


#[aoc(day8, part2)]
fn day8_part2(input: &str) -> u32 {
    let (grid, map) = parse_day8(input);
    let x_max = grid[0].len() as isize;
    let y_max = grid.len() as isize;
    let mut vistos = vec![];
    map.keys().fold(0, |mut acc, c| {
        let mut points = map.get(c).expect("deberia existir").clone();
        while let Some(actual) = points.pop() {
            points.iter().for_each(|p| {
                let antinodes = actual.get_all_antinodes(p, x_max, y_max);
                antinodes.iter().for_each(|p| {
                    if !vistos.contains(p) {
                        vistos.push(*p);
                        acc += 1;
                    }
                });
            });
        }
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day8_part1() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        // let (grid, map) = parse_day8(input);
        // println!("{:?}", grid);
        // println!("{:?}", map);
        assert_eq!(day8_part1(input), 14);
    }

    #[test]
    fn test_day8_part2() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(day8_part2(input), 34);
    }
}
