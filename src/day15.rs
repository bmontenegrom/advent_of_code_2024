#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn nex_pos(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        }
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<char>>,
    robot: (usize, usize),
}

impl Map {
    fn new(map: &str) -> Self {
        let map: Vec<Vec<char>> = map.lines().map(|l| l.chars().collect()).collect();
        let robot = map
            .iter()
            .enumerate()
            .find_map(|(i, l)| l.iter().position(|&c| c == '@').map(|j| (i, j)))
            .unwrap();
        Self { map, robot }
    }


    fn new_wide(input: &str) -> Self {
        let mut map = Vec::new();
        for line in input.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                match c {
                    '#' => {
                        row.push('#');
                        row.push('#');
                    }
                    '.' => {
                        row.push('.');
                        row.push('.');
                    }
                    'O' => {
                        row.push('[');
                        row.push(']');
                    }
                    '@' => {
                        row.push('@');
                        row.push('.');
                    }
                    _ => panic!("Invalid character"),
                }
            }
            map.push(row);
        }
        let robot = map
            .iter()
            .enumerate()
            .find_map(|(i, l)| l.iter().position(|&c| c == '@').map(|j| (i, j)))
            .unwrap();
        Self { map, robot }
    }


    fn shift(&mut self, x: usize, y: usize, instruction: &Direction) {
        let (nx, ny) = instruction.nex_pos(x, y);
        if self.map[ny][nx] == '.' {
            let actual = self.map[y][x];
            self.map[y][x] = '.';
            self.map[ny][nx] = actual;
            if actual == '@' {
                self.robot = (nx, ny);
            }
        } else if self.map[ny][nx] != '#' {
            self.shift(nx, ny, instruction);
            if self.map[ny][nx] == '.' {
                let actual = self.map[y][x];
                self.map[y][x] = '.';
                self.map[ny][nx] = actual;
                if actual == '@' {
                    self.robot = (nx, ny);
                }
            }
        }
    }

    fn run(&mut self, instructions: &[Direction]) {
        for i in instructions {
            self.shift(self.robot.0, self.robot.1, i);
        }
    }

    fn gps(&self) -> usize {
        self.map
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .map(move |(x, c)| if *c == 'O' { 100 * y + x } else { 0 })
            })
            .sum()
    }
}

#[aoc(day15, part1)]
fn day15_part1(input: &str) -> usize {
    let (map, instruction) = input.split_once("\n\n").unwrap();
    let mut map = Map::new(map);
    let instructions: Vec<Direction> = instruction
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(Direction::from)
        .collect();
    map.run(&instructions);
    map.gps()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day15_shift() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        assert_eq!(day15_part1(input), 10092);
    }
}
