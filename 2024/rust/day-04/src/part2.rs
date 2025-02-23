use std::collections::HashMap;

use glam::IVec2;

const DIRECTIONS: [[IVec2; 2]; 4] = [
    [IVec2::new(-1, -1), IVec2::new(1, 1)],
    [IVec2::new(-1, 1), IVec2::new(1, -1)],
    [IVec2::new(1, 1), IVec2::new(-1, -1)],
    [IVec2::new(1, -1), IVec2::new(-1, 1)],
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

    let search_chars = ['M', 'S'];

    Ok(positions
        .iter()
        .filter(|(_positions, value)| **value == 'A')
        .filter(|(position, _value)| {
            DIRECTIONS
                .iter()
                .map(|search_char_positions| {
                    search_char_positions
                        .iter()
                        .map(|pos| positions.get(&(*position + pos)))
                        .enumerate()
                        .all(|(index, value)| search_chars.get(index) == value)
                })
                .filter(|b| *b)
                .count()
                == 2
        })
        .count()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = indoc::indoc!(
            r#"
        .M.S......
        ..A..MSMS.
        .M.S.MAA..
        ..A.ASMSM.
        .M.S.M....
        ..........
        S.S.S.S.S.
        .A.A.A.A..
        M.M.M.M.M.
        ..........
        "#
        );
        assert_eq!("9", process(input)?);
        Ok(())
    }
}
