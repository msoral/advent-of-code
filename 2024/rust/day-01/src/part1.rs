#[tracing::instrument]
pub fn process(_input: &str) -> anyhow::Result<String> {
    todo!("day 01 - part 1");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        todo!("haven't built test yet");
        let input = indoc::indoc!("");
        assert_eq!("", process(input)?);
        Ok(())
    }
}