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
        let x_max = self.x.max(other.x);
        let y_max = self.y.max(other.y);
        let x_min = self.x.min(other.x);
        let y_min = self.y.min(other.y);
        let point_1 = Point::new(
            x_max + self.x.abs_diff(other.x) as isize,
            y_max + self.y.abs_diff(other.y) as isize,
        );
        let point_2 = Point::new(
            x_min - self.x.abs_diff(other.x) as isize,
            y_min - self.y.abs_diff(other.y) as isize,
        );
        (point_1, point_2)
    }
}

fn parse_day8(input: &str) -> (Vec<Vec<char>>, HashMap<char, Vec<Point>>) {
    let map = input
        .lines()
        .zip(0_isize..)
        .fold(HashMap::new(), |mut acc, (l, i)| {
            l.chars().zip(0_isize..).for_each(|(c, j)| {
                if c != '.' {
                    acc.entry(c).or_insert_with(Vec::new).push(Point::new(j, i));
                }
            });
            acc
        });
    let grid = input.lines().map(|l| l.chars().collect()).collect();
    (grid, map)
}

fn day8_part1(input: &str) -> u32 {
    let (grid, map) = parse_day8(input);
    let x_max = grid[0].len() as isize;
    let y_max = grid.len() as isize;
    let mut vistos = vec![];
    map.keys().fold(0, |mut acc, c| {
        let mut points = map.get(c).expect("deberia existir").clone();
        while !points.is_empty() {
            let actual = points.pop().expect("deberia existir");
            points.iter().for_each(|p| {
                println!("actual: {:?} p: {:?}", actual, p);
                let (point_1, point_2) = actual.get_antinode(p);
                if point_1.x >= 0
                    && point_1.x < x_max
                    && point_1.y >= 0
                    && point_1.y < y_max
                    && !vistos.contains(&point_1)
                {
                    println!("{:?}", point_1);
                    vistos.push(point_1);
                    acc += 1;
                }
                if point_2.x >= 0
                    && point_2.x < x_max
                    && point_2.y >= 0
                    && point_2.y < y_max
                    && !vistos.contains(&point_2)
                {
                    println!("{:?}", point_2);
                    vistos.push(point_2);
                    acc += 1;
                }
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
}
