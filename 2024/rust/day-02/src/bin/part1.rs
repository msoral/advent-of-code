use day_02::part1::process;

#[tracing::instrument]
fn main() -> anyhow::Result<()> {
    use anyhow::Context;
    tracing_subscriber::fmt::init();

    let file = include_str!("../../input1.txt");
    let result = process(file).context("process part 1")?;
    println!("{}", result);
    Ok(())
}