#![allow(dead_code)]
/**
 * (5 kyu)
 * https://www.codewars.com/kata/52597aa56021e91c93000cb0/train/rust
 *
 * Write an algorithm that takes an array and moves all
 * of the zeros to the end, preserving the order of the other elements.
 */

fn move_zeros(nums: &[u8]) -> Vec<u8> {
  let mut result: Vec<u8> = Vec::with_capacity(nums.len());

  result.extend(nums.iter().filter(|&&x| x != 0));
  result.resize(nums.len(), 0);

  result
}

#[cfg(test)]
mod tests {
  use super::move_zeros;

  const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

  fn dotest(a: &[u8], expected: &[u8]) {
    let actual = move_zeros(a);
    assert!(
      actual == expected,
      "With arr = {a:?}\nExpected {expected:?} but got {actual:?}"
    )
  }

  #[test]
  fn sample_tests() {
    dotest(
      &[1, 2, 0, 1, 0, 1, 0, 3, 0, 1],
      &[1, 2, 1, 1, 3, 1, 0, 0, 0, 0],
    );
    dotest(
      &[9, 0, 0, 9, 1, 2, 0, 1, 0, 1, 0, 3, 0, 1, 9, 0, 0, 0, 0, 9],
      &[9, 9, 1, 2, 1, 1, 3, 1, 9, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    );
    dotest(&[0, 0], &[0, 0]);
    dotest(&[0], &[0]);
    dotest(&[], &[]);
  }
}
