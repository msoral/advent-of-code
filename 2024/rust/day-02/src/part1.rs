use nom::{
    character::complete::{self, newline, space1},
    multi::separated_list1,
    IResult,
};

enum Direction {
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq)]
enum Safety {
    Safe,
    Unsafe,
}

#[tracing::instrument(ret)]
fn check_safety(report: &Report) -> Safety {
    use itertools::Itertools;

    let mut direction: Option<Direction> = None;
    for (a, b) in report.iter().tuple_windows() {
        let diff = a - b;
        if !(1..=3).contains(&diff.abs()) {
            return Safety::Unsafe;
        }
        match diff.signum() {
            -1 => match direction {
                Some(Direction::Up) => return Safety::Unsafe,
                Some(Direction::Down) => (),
                None => {
                    direction = Some(Direction::Down);
                }
            },
            1 => match direction {
                Some(Direction::Up) => (),
                Some(Direction::Down) => return Safety::Unsafe,
                None => {
                    direction = Some(Direction::Up);
                }
            },
            0 => return Safety::Unsafe,
            _ => panic!("should never have a non -1, 1, 0 number"),
        }
    }
    Safety::Safe
}

type Report = Vec<i32>;

fn parse(input: &str) -> IResult<&str, Vec<Report>> {
    separated_list1(newline, separated_list1(space1, complete::i32))(input)
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> anyhow::Result<String> {
    let (_, reports) = parse(input).map_err(|err| anyhow::anyhow!("Parse failed {}", err))?;

    let safe_report_count = reports
        .iter()
        .map(check_safety)
        .filter(|safety| safety == &Safety::Safe)
        .count();
    Ok(safe_report_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = indoc::indoc!(
            r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#
        );
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
