#![allow(dead_code)]
/**
 * (6 kyu)
 * https://www.codewars.com/kata/54da5a58ea159efa38000836/train/rust
 *
 * Given an array of integers, find the one that appears an odd number of times.
 *
 * There will always be only one integer that appears an odd number of times.
 */

fn find_odd(arr: &[i32]) -> i32 {
  arr.into_iter().fold(0, |acc, n| acc ^ n)
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
