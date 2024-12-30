

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult, Parser,
};

#[derive(Debug, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }
}



#[derive(Debug)]
struct Machine {
    a_button: Point,
    b_button: Point,
    prize: Point,
}

impl Machine {
    fn new(a_button: Point, b_button: Point, prize: Point) -> Machine {
        Machine {
            a_button,
            b_button,
            prize,
        }
    }

    fn solve(&self) -> Option<isize> {
        let a = self.a_button.x;
        let b = self.b_button.x;
        let c = self.a_button.y;
        let d = self.b_button.y;
        let e = self.prize.x;
        let f = self.prize.y;
        let den = a * d - b * c;
        let num = e * d - b * f;
        if den == 0 || num % den != 0 {
            return None;
        }
        let x = num / den;

        let num = a * f - e * c;
        if num % den != 0 {
            return None;
        }
        let y = num / den;

        Some(3 * x + y)
    }
}

fn a_button(input: &str) -> IResult<&str, Point> {
    preceded(
        tag("Button A: X+"),
        separated_pair(complete::u32, tag(", Y+"), complete::u32)
            .map(|(x, y)| Point::new(x as isize, y as isize)),
    )(input)
}

fn b_button(input: &str) -> IResult<&str, Point> {
    preceded(
        tag("Button B: X+"),
        separated_pair(complete::u32, tag(", Y+"), complete::u32)
            .map(|(x, y)| Point::new(x as isize, y as isize)),
    )(input)
}

fn prize(input: &str) -> IResult<&str, Point> {
    preceded(
        tag("Prize: X="),
        separated_pair(complete::u32, tag(", Y="), complete::u32)
            .map(|(x, y)| Point::new(x as isize, y as isize)),
    )(input)
}

fn machine(input: &str) -> IResult<&str, Machine> {
    let (input, (a, b, p)) = tuple((
        terminated(a_button, line_ending),
        terminated(b_button, line_ending),
        prize,
    ))(input)?;
    Ok((input, Machine::new(a, b, p)))
}

fn parse_day12(input: &str) -> IResult<&str, Vec<Machine>> {
    separated_list1(tuple((line_ending, line_ending)), machine)(input)
}


#[aoc(day13, part1)]
fn day12_part1(input: &str) -> isize {
    let (_, machines) = parse_day12(input).unwrap();
    machines.iter().filter_map(|m| m.solve()).sum()
}

#[aoc(day13, part2)]
fn day12_part2(input: &str) -> isize {
    let (_, machines) = parse_day12(input).unwrap();
    machines.iter().filter_map(|m|{
        let new_prize = Point::new(m.prize.x + 10000000000000, m.prize.y + 10000000000000);
        Machine::new(m.a_button.clone(), m.b_button.clone(), new_prize).solve()
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day12_part1() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!(day12_part1(input), 480);
    }
}
