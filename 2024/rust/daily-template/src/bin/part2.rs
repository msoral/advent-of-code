use {{crate_name}}::part2::process;

#[tracing::instrument]
fn main() -> anyhow::Result<()> {
    use anyhow::Context;
    tracing_subscriber::fmt::init();

    let file = include_str!("../../input2.txt");
    let result = process(file).context("process part 2")?;
    println!("{}", result);
    Ok(())
}