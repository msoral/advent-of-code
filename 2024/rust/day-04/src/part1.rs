use std::collections::HashMap;

use glam::IVec2;

const DIRECTIONS: [[IVec2; 3]; 8] = [
    [IVec2::new(0, 1), IVec2::new(0, 2), IVec2::new(0, 3)],
    [IVec2::new(0, -1), IVec2::new(0, -2), IVec2::new(0, -3)],
    [IVec2::new(1, 1), IVec2::new(2, 2), IVec2::new(3, 3)],
    [IVec2::new(1, -1), IVec2::new(2, -2), IVec2::new(3, -3)],
    [IVec2::new(-1, 1), IVec2::new(-2, 2), IVec2::new(-3, 3)],
    [IVec2::new(-1, -1), IVec2::new(-2, -2), IVec2::new(-3, -3)],
    [IVec2::new(1, 0), IVec2::new(2, 0), IVec2::new(3, 0)],
    [IVec2::new(-1, 0), IVec2::new(-2, 0), IVec2::new(-3, 0)],
];

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> anyhow::Result<String> {
    let positions: HashMap<IVec2, char> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, value)| (IVec2::new(x as i32, y as i32), value))
        })
        .collect();

    let search_chars = ['M', 'A', 'S'];

    Ok(positions
        .iter()
        .filter(|(_position, value)| **value == 'X')
        .map(|(position, _value)| {
            DIRECTIONS
                .iter()
                .map(|search_char_positions| {
                    search_char_positions
                        .iter()
                        .map(|offset| positions.get(&(position + offset)))
                        .enumerate()
                        .all(|(index, value)| search_chars.get(index) == value)
                })
                .filter(|b| *b)
                .count()
        })
        .sum::<usize>()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = indoc::indoc!(
            r#"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
        "#
        );
        assert_eq!("18", process(input)?);
        Ok(())
    }
}
