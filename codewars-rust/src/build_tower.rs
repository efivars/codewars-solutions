#![allow(dead_code)]
/**
 * (6 kyu)
 * https://www.codewars.com/kata/576757b1df89ecf5bd00073b/train/rust
 *
 * Build a pyramid-shaped tower, as an array/list of strings, given a positive
 * integer number of floors. A tower block is represented with "*" character.
 */

fn tower_builder(n_floors: usize) -> Vec<String> {
    let floor_width = 1 + ((n_floors - 1) * 2);

    (1..=n_floors)
        .into_iter()
        .fold(Vec::with_capacity(n_floors), |mut acc, floor| {
            let wall_width = 1 + ((floor - 1) * 2);
            let space_width = (floor_width - wall_width) / 2;

            let wall = "*".repeat(wall_width);
            let space = " ".repeat(space_width);

            acc.push(format!(
                "{}{}{}",
                space.as_str(),
                wall.as_str(),
                space.as_str()
            ));

            acc
        })
}

#[cfg(test)]
mod tests {
    use super::tower_builder;

    #[test]
    fn fixed_tests() {
        assert_eq!(tower_builder(1), vec!["*"]);
        assert_eq!(tower_builder(2), vec![" * ", "***"]);
        assert_eq!(tower_builder(3), vec!["  *  ", " *** ", "*****"]);
    }
}
