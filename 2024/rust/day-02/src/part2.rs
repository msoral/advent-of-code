use nom::{
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    IResult,
};

enum Direction {
    Increasing,
    Decreasing,
}

#[tracing::instrument(ret)]
fn check_safety(report: &Report) -> Result<(), String> {
    use itertools::Itertools;
    let mut direction: Option<Direction> = None;

    for (a, b) in report.iter().tuple_windows() {
        let diff = a - b;
        if !(1..=3).contains(&diff.abs()) {
            return Err(format!("{}, {} diff value is {}", a, b, diff.abs()))
        }
        match diff.signum() {
            -1 => match direction {
                Some(Direction::Increasing) => return Err(format!("{}, {} switched to increasing", a, b)),
                Some(Direction::Decreasing) => (),
                None => {
                    direction = Some(Direction::Decreasing);
                }
            },
            1 => match direction {
                Some(Direction::Increasing) => (),
                Some(Direction::Decreasing) => return Err(format!("{}, {} switched to decreasing", a, b)),
                None => {
                    direction = Some(Direction::Increasing);
                }
            },
            0 => return Err(format!("{}, {} diff was 0", a, b)),
            _ => panic!("should never have a non -1, 1, 0 number"),
        }
    }
    Ok(())
}

type Report = Vec<i32>;

fn parse(input: &str) -> IResult<&str, Vec<Report>> {
    separated_list1(line_ending, separated_list1(space1, complete::i32))(input)
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> anyhow::Result<String> {
    let (_, reports) = parse(input).map_err(|err| anyhow::anyhow!("Parse failed {}", err))?;

    let safe_report_count = reports
        .iter()
        .filter(|report| {
            if check_safety(report).is_ok() {
                return true;
            }
            
            // Check if there is a safe version of the report
            for index in 0..report.len() {
                let mut amended_report = (*report).clone();
                amended_report.remove(index);
                if check_safety(&amended_report).is_ok() {
                    return true;
                } else {
                    continue;
                }
            }
            return false;
        })
        .count();
    Ok(safe_report_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = indoc::indoc!(
            r#"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
        "#
        );
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
