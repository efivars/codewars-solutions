#![allow(dead_code)]
/**
 * (7 kyu)
 * https://www.codewars.com/kata/546e2562b03326a88e000020/train/rust
 *
 * Welcome. In this kata, you are asked to square every digit of a number and concatenate them.
 * For example, if we run 9119 through the function, 811181 will come out, because 92 is 81 and 12 is 1. (81-1-1-81)
 * Example #2: An input of 765 will/should return 493625 because 72 is 49, 62 is 36, and 52 is 25. (49-36-25)
 * Note: The function accepts an integer and returns an integer.
 * Happy Coding!
 */

fn digits(num: u64) -> impl Iterator<Item = u64> {
  num
    .to_string()
    .chars()
    .map(|d| d.to_digit(10).unwrap().into())
    .collect::<Vec<_>>()
    .into_iter()
}

fn square_digits(num: u64) -> u64 {
  digits(num)
    .map(|d| (d * d).to_string())
    .collect::<Vec<String>>()
    .join("")
    .parse()
    .unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn squares_every_digit() {
    let samples = [(9119, 811181), (1234, 14916), (2405, 416025)];

    for (input, expected) in samples {
      assert_eq!(square_digits(input), expected)
    }
  }
}
