#![allow(dead_code)]
/**
 * (6 kyu)
 * https://www.codewars.com/kata/54da5a58ea159efa38000836/train/rust
 *
 * Given an array of integers, find the one that appears an odd number of times.
 *
 * There will always be only one integer that appears an odd number of times.
 */
use std::collections::HashMap;

fn find_odd(arr: &[i32]) -> i32 {
    let mut nums: HashMap<&i32, usize> = HashMap::new();

    for it in arr {
        let occurrences = nums.get(it).unwrap_or(&0);

        nums.insert(it, occurrences + 1);
    }

    nums.into_iter()
        .find(|(_, occurrences)| occurrences % 2 == 1)
        .map(|(num, _occurrences)| num)
        .unwrap()
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_odd() {
        let samples = [
            (
                (&vec![20, 1, -1, 2, -2, 3, 3, 5, 5, 1, 2, 4, 20, 4, -1, -2, 5]),
                (5),
            ),
            ((&vec![1, 1, 2, -2, 5, 2, 4, 4, -1, -2, 5]), (-1)),
            ((&vec![20, 1, 1, 2, 2, 3, 3, 5, 5, 4, 20, 4, 5]), (5)),
            ((&vec![10]), (10)),
            ((&vec![1, 1, 1, 1, 1, 1, 10, 1, 1, 1, 1]), (10)),
            ((&vec![5, 4, 3, 2, 1, 5, 4, 3, 2, 10, 10]), (1)),
        ];

        for (input, expected) in samples {
            assert_eq!(find_odd(input), expected)
        }
    }
}
