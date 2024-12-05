use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::value,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

#[derive(Clone, PartialEq, Eq)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

fn mul(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;
    Ok((input, Instruction::Mul(pair.0, pair.1)))
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Dont, tag("don't()")),
        value(Instruction::Do, tag("do()")),
        mul,
    ))(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(many_till(anychar, instruction).map(|(_discard, ins)| ins))(input)
}

#[aoc(day3, part1)]
fn part1(input: &str) -> u32 {
    let (_input, instructions) = parse(input).expect("error en parser");
    instructions.iter().fold(0, |acc, ins| match ins {
        Instruction::Mul(a, b) => acc + a * b,
        _ => acc,
    })
}
#[aoc(day3, part2)]
fn part2(input: &str) -> u32 {
    let (_input, instructions) = parse(input).expect("deberia parsear");
    let mut state = Instruction::Do;
    instructions.iter().fold(0, |acc, ins| match ins {
        Instruction::Mul(a, b) => {
            if state == Instruction::Do {
                acc + a * b
            } else {
                acc
            }
        }
        Instruction::Do => {
            state = Instruction::Do;
            acc
        }
        Instruction::Dont => {
            state = Instruction::Dont;
            acc
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day3_part1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(input), 161)
    }

    #[test]
    fn test_day3_part2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(input), 48)
    }
}
