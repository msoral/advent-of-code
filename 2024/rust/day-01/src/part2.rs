use anyhow::Context;

#[tracing::instrument]
pub fn process(input: &str) -> anyhow::Result<String> {
    let mut left = vec![];
    let mut right = vec![];
    
    for line in input.lines() {
        let mut items = line.split_whitespace();

        left.push(items.next().context("Attempt to fetch the first element failed")?.parse::<usize>().context("The string failed to parse into an i32")?);
        right.push(items.next().context("Attempt to fetch the second element failed")?.parse::<usize>().context("The string failed to parse into an i32")?);
    }

    let result: usize = left.iter().map(|number| {
        number * right.iter().filter(|r| &number == r).count()
    })
    .sum();

 
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = indoc::indoc!(
            r#"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
        "#
        );
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
