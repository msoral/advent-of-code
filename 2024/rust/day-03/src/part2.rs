use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::value,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

#[tracing::instrument]
pub fn process(input: &str) -> anyhow::Result<String> {
    let (_input, instructions) = parse(input).map_err(|e| anyhow::anyhow!("parse failed {}", e))?;

    fn step((can_process, acc): (bool, u32), inst: &Instruction) -> (bool, u32) {
        match inst {
            Instruction::Mul(a, b) => {
                if can_process {
                    (can_process, acc + a * b)
                } else {
                    (can_process, acc)
                }
            },
            Instruction::Do => (true, acc),
            Instruction::Dont => (false, acc),
        }
    }

    let (_, result) = instructions.iter().fold((true, 0), step);
    Ok(result.to_string())
}

#[derive(Clone, Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = indoc::indoc!(
            r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#
        );
        assert_eq!("48", process(input)?);
        Ok(())
    }
}
