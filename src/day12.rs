#[derive(Debug, Clone)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Region {
    area: usize,
    perimeter: usize,
    sides: usize,
}

impl Region {
    fn new(points: &[Point], grid: &[Vec<char>]) -> Self {
        let area = points.len();
        let perimeter = points.iter().map(|p| 4 - p.vecinos(grid).len()).sum();
        let sides = points.iter().map(|p| p.corners(grid)).sum();
        Region {
            area,
            perimeter,
            sides,
        }
    }
}

impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }

    fn is_valid(&self, grid: &[Vec<char>]) -> bool {
        self.x >= 0
            && self.y >= 0
            && self.x < grid[0].len() as isize
            && self.y < grid.len() as isize
    }

    fn vecinos(&self, grid: &[Vec<char>]) -> Vec<Point> {
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        directions.iter().fold(vec![], |mut acc, (dx, dy)| {
            let new_point = Point::new(self.x + dx, self.y + dy);
            if new_point.is_valid(grid)
                && grid[new_point.y as usize][new_point.x as usize]
                    == grid[self.y as usize][self.x as usize]
            {
                acc.push(new_point);
            }
            acc
        })
    }

    fn corners(&self, grid: &[Vec<char>]) -> usize {
        let corners = [
            [
                Point::new(self.x - 1, self.y - 1),
                Point::new(self.x, self.y - 1),
                Point::new(self.x - 1, self.y),
            ],
            [
                Point::new(self.x + 1, self.y - 1),
                Point::new(self.x, self.y - 1),
                Point::new(self.x + 1, self.y),
            ],
            [
                Point::new(self.x + 1, self.y + 1),
                Point::new(self.x, self.y + 1),
                Point::new(self.x + 1, self.y),
            ],
            [
                Point::new(self.x - 1, self.y + 1),
                Point::new(self.x, self.y + 1),
                Point::new(self.x - 1, self.y),
            ],
        ];

        corners
            .iter()
            .filter(|corner| {
                let esquina = grid
                    .get(corner[0].y as usize)
                    .and_then(|l| l.get(corner[0].x as usize));
                let a = grid
                    .get(corner[1].y as usize)
                    .and_then(|l| l.get(corner[1].x as usize));
                let b = grid
                    .get(corner[2].y as usize)
                    .and_then(|l| l.get(corner[2].x as usize));
                let char = grid
                    .get(self.y as usize)
                    .and_then(|l| l.get(self.x as usize));
                (char != a && char != b) || (char == b && char == a && char != esquina)
            })
            .count()
    }
}

fn parse_day12(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn regiones(grid: &[Vec<char>]) -> Vec<Region> {
    let mut res = vec![];
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if visited[y][x] {
                continue;
            }
            let mut queue = vec![Point::new(x as isize, y as isize)];
            let mut region = vec![];
            while let Some(p) = queue.pop() {
                if visited[p.y as usize][p.x as usize] {
                    continue;
                }
                visited[p.y as usize][p.x as usize] = true;
                region.push(p.clone());
                queue.extend(p.vecinos(grid).iter().cloned());
            }
            res.push(Region::new(&region, grid));
        }
    }
    res
}

#[aoc(day12, part1)]
fn day12_part1(input: &str) -> usize {
    let grid = parse_day12(input);
    let regiones = regiones(&grid);
    regiones.iter().map(|r| r.area * r.perimeter).sum()
}

#[aoc(day12, part2)]
fn day12_part2(input: &str) -> usize {
    let grid = parse_day12(input);
    let regiones = regiones(&grid);
    regiones.iter().map(|r| r.sides * r.area).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!(day12_part1(input), 1930);
    }

    #[test]
    fn test_part2() {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!(day12_part2(input), 1206);
    }
}
