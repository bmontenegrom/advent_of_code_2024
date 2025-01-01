



#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    fn get_neighbor(&self, direction: Direction) -> Point {
        match direction {
            Direction::Up => Point::new(self.x, self.y - 1),
            Direction::Down => Point::new(self.x, self.y + 1),
            Direction::Left => Point::new(self.x - 1, self.y),
            Direction::Right => Point::new(self.x + 1, self.y),
        }
    }
}

#[derive(Debug, Clone)]
struct Values {
    point: Point,
    distance: usize,
    direction: Direction,
    path: Vec<Point>,
}

impl Ord for Values {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for Values {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Values {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for Values {}

impl Values {
    fn new(point: Point, distance: usize, direction: Direction, path: Vec<Point>) -> Self {
        Self {
            point,
            distance,
            direction,
            path
        }
    }
}

struct Grid {
    grid: Vec<Vec<char>>,
    start: Point,
    end: Point,
    distance: usize,
    paths: Vec<Vec<Point>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let mut start = Point::new(0, 0);
        let mut end = Point::new(0, 0);
        grid.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, &cell)| {
                if cell == 'S' {
                    start = Point::new(x as isize, y as isize);
                } else if cell == 'E' {
                    end = Point::new(x as isize, y as isize);
                }
            });
        });
        Self { grid, start, end, distance: std::usize::MAX, paths: vec![] }
    }

    fn neighbors(&self, value: &Values, direction: Direction) -> Vec<Values> {
        let direciones = vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];
        direciones
            .iter()
            .filter_map(|dir| {
                let neighbor = value.point.get_neighbor(dir.clone());
                let mut path = value.path.clone();
                path.push(value.point.clone());
                if self.grid[neighbor.y as usize][neighbor.x as usize] == '#' || direction == dir.opposite() {
                    return None;
                }
                let distance = if *dir == direction {
                    value.distance + 1
                } else {
                    value.distance + 1001
                };
                Some(Values::new(neighbor, distance, dir.clone(), path))
            })
            .collect()
    }

    fn find_path(&mut self) {
        let mut visited = std::collections::HashMap::new();
        let mut queue = std::collections::BinaryHeap::new();
        let mut paths = vec![];
        queue.push(Values::new(self.start.clone(), 0, Direction::Right, vec![self.start.clone()]));
        while let Some(value) = queue.pop() {
            if value.point == self.end && value.distance < self.distance {
                
                self.distance = value.distance;
                paths = vec![value.path.clone()];
                
            } else if value.point == self.end && value.distance == self.distance {
                paths.push(value.path.clone());
                
            }
            if let Some(visited_value) = visited.get(&value.point) {
                if value.distance > *visited_value {
                    continue;
                } 
            }
            visited.insert(value.point.clone(), value.distance);
            self.neighbors(&value.clone(), value.direction)
                .iter()
                .for_each(|neighbor| {
                    queue.push(neighbor.clone());
                });
        }
        self.paths = paths;
        println!("paths: {:?}", self.paths);
    }
}

#[aoc(day16, part1)]
fn day16_part1(input: &str) -> usize {
    let mut grid = Grid::new(input);
    grid.find_path();
    grid.distance    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value() {
        let point = Point::new(1, 2);
        let value1 = Values::new(point.clone(), 10, Direction::Up, vec![point.clone()]);
        let value2 = Values::new(point.clone(), 12, Direction::Up, vec![point]);
        assert_eq!(true, value1 > value2);
    }

    #[test]
    fn test_queue() {
        let mut queue = std::collections::BinaryHeap::new();
        queue.push(Values::new(Point::new(1, 2), 10, Direction::Up, vec![Point::new(1, 2)]));
        queue.push(Values::new(Point::new(1, 2), 12, Direction::Up, vec![Point::new(1, 2)]));
        assert_eq!(Values::new(Point::new(1, 2), 10, Direction::Up, vec![Point::new(1, 2)]), queue.pop().unwrap());
    }

    #[test]
    fn test_day16_part1() {
        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        assert_eq!(7036, day16_part1(input));
    }

    #[test]
    fn test_day16_part1_2() {
        let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        assert_eq!(11048, day16_part1(input));
    }

    #[test]
    fn test_day16_paths() {
                let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        let mut grid = Grid::new(input);
        grid.find_path();
        let cant = grid.paths.iter().flatten().collect::<std::collections::HashSet<_>>().len();
        println!("cantidad de puntos: {}",cant );
    }
}
