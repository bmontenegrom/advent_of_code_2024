use std::ops::{Add, Mul};

use nom::{
    bytes::complete::tag, character::complete::{self, line_ending, space1}, multi::separated_list1, sequence::{preceded, separated_pair}, IResult, Parser
};


const X_MAX: i32 = if cfg!(test) { 11 } else { 101 };
const Y_MAX: i32 = if cfg!(test) { 7 } else { 103 };


#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<i32> for Point {
    type Output = Point;
    fn mul(self, other: i32) -> Point {
        Point {
            x: self.x * other,
            y: self.y * other,
        }
    }
}


#[derive(Debug)]
struct Robot {
    position: Point,
    velocity: Point,
}

impl Robot {
    fn new(position: Point, velocity: Point) -> Robot {
        Robot { position, velocity }
    }

    fn step(&mut self, step: i32, x_max: i32, y_max: i32) {
        self.position = self.position.clone() + self.velocity.clone() * step;
        self.position.x = self.position.x.rem_euclid(x_max);
        self.position.y = self.position.y.rem_euclid(y_max);
    }
}



fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input, (x, y)) = separated_pair(complete::i32, tag(","), complete::i32)(input)?;
    Ok((input, Point::new(x, y)))
}


fn parse_day13(input: &str)-> IResult<&str, Vec<Robot>> {
    separated_list1(
        line_ending,
        separated_pair(
            preceded(tag("p="), parse_point),
            space1,
            preceded(tag("v="), parse_point),
        ).map(|(position, velocity)| Robot::new(position, velocity)),
    )(input)
}

#[aoc(day14, part1)]
fn day14_part1(input: &str) -> i32 {
    let (_, mut robots) = parse_day13(input).unwrap();
    robots.iter_mut().for_each(|robot| robot.step(100, X_MAX, Y_MAX));
    let cuadrants = robots.iter().fold([0;4], |mut acc, robot|{
        if robot.position.x < X_MAX / 2 && robot.position.y < Y_MAX / 2 {
            acc[0] += 1;
        } else if robot.position.x > X_MAX / 2 && robot.position.y < Y_MAX / 2 {
            acc[1] += 1;
        } else if robot.position.x < X_MAX / 2 && robot.position.y > Y_MAX / 2 {
            acc[2] += 1;
        } else if robot.position.x > X_MAX / 2 && robot.position.y > Y_MAX / 2 {
            acc[3] += 1;
        }
        acc
    });
    cuadrants.iter().product()
    
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day14_part1() {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!(day14_part1(input), 12);
    }

 
}