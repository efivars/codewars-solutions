#![allow(dead_code)]
/**
 * (4 kyu)
 * https://www.codewars.com/kata/52f677797c461daaf7000740/train/rust
 *
 * Given an array X of positive integers, its elements are to be transformed by running the following operation on them as many times as required:
 *
 * if X[i] >(actually <?) X[j] then X[i(actually j?)] = X[i] - X[j]
 *
 * When no more transformations are possible, return its sum ("smallest possible sum").
 */
use std::cmp::Ordering;

fn solution(arr: &[u64]) -> u128 {
  let mut nums = arr.to_vec();
  let mut is_dirty = true;

  while is_dirty {
    is_dirty = false;

    for i in 0..(nums.len() - 1) {
      match nums[i].cmp(&nums[i + 1]) {
        Ordering::Equal => {}
        Ordering::Less => {
          nums[i + 1] -= nums[i];
          is_dirty = true;
        }
        Ordering::Greater => {
          nums[i] -= nums[i + 1];
          is_dirty = true;
        }
      };
    }
  }

  nums.into_iter().fold(0, |acc, num| acc + (num as u128))
}

#[cfg(test)]
mod tests {
  use super::solution;

  #[test]
  fn fixed_tests() {
    assert_eq!(solution(&[1, 21, 55]), 3);
    assert_eq!(solution(&[3, 13, 23, 7, 83]), 5);
    assert_eq!(solution(&[4, 16, 24]), 12);
    assert_eq!(solution(&[30, 12]), 12);
    assert_eq!(solution(&[60, 12, 96, 48, 60, 24, 72, 36, 72, 72, 48]), 132);
    assert_eq!(
      solution(&[71, 71, 71, 71, 71, 71, 71, 71, 71, 71, 71, 71, 71]),
      923
    );
    assert_eq!(solution(&[11, 22]), 22);
    assert_eq!(solution(&[9]), 9);
    assert_eq!(solution(&[1]), 1);
    assert_eq!(solution(&[9, 9]), 18);
  }
}
