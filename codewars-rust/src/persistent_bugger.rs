#![allow(dead_code)]
/**
 * (6 kyu)
 * https://www.codewars.com/kata/55bf01e5a717a0d57e0000ec/train/rust
 *
 * Write a function, persistence, that takes in a positive parameter num
 * and returns its multiplicative persistence, which is the number of times you
 * must multiply the digits in num until you reach a single digit.
 */

fn persistence_recursive(num: u64, times: u64) -> u64 {
  if num < 10 {
    return times;
  }

  let digits: Vec<u64> = num
    .to_string()
    .chars()
    .map(|d| d.to_string().parse::<u64>().unwrap())
    .collect();

  let result = digits.into_iter().fold(1_u64, |acc, value| acc * value);

  persistence_recursive(result, times + 1)
}

fn persistence(num: u64) -> u64 {
  persistence_recursive(num, 0)
}

#[cfg(test)]
mod tests {
  use super::persistence;

  #[test]
  fn sample_tests() {
    assert_eq!(persistence(39), 3);
    assert_eq!(persistence(4), 0);
    assert_eq!(persistence(25), 2);
    assert_eq!(persistence(999), 4);
  }
}
