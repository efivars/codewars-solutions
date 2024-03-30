#![allow(dead_code)]
/**
 * (6 kyu)
 * https://www.codewars.com/kata/52c31f8e6605bcc646000082/train/rust
 *
 * Write a function that takes an array of numbers (integers for the tests) and a target number.
 * It should find two different items in the array that, when added together, give the target value.
 * The indices of these items should then be returned in a tuple / list (depending on your language)
 */
use core::panic;
use std::collections::HashMap;

fn two_sum(numbers: &[i32], target: i32) -> (usize, usize) {
  let mut map: HashMap<i32, usize> = HashMap::with_capacity(numbers.len() / 2);

  for (index, number) in numbers.into_iter().enumerate() {
    let remainder = target - number;

    match map.get(&number) {
      Some(remainder_index) => return (index, remainder_index.to_owned()),
      None => {}
    }

    map.insert(remainder, index.to_owned());
  }

  panic!("Should never occur!")
}

#[cfg(test)]
mod tests {
  use super::two_sum;

  #[test]
  fn sample() {
    do_test(&[1, 2, 3], 4);
    do_test(&[1234, 5678, 9012], 14690);
    do_test(&[2, 2, 3], 4);
  }

  fn do_test(nums: &[i32], sum: i32) {
    let len = nums.len();
    let user_tuple = two_sum(nums, sum);
    assert!(
      user_tuple.0 < len && user_tuple.1 < len,
      "\nnumbers: {:?}\ntarget: {}\nresult: {:?}\nresult tuple has an index out of bounds",
      nums,
      sum,
      user_tuple
    );
    assert!(
      user_tuple.0 != user_tuple.1,
      "\nnumbers: {:?}\ntarget: {}\nresult: {:?}\nresult tuple must have two different indices",
      nums,
      sum,
      user_tuple
    );
    let num1 = nums[user_tuple.0];
    let num2 = nums[user_tuple.1];
    let user_sum = num1 + num2;
    assert!(
            user_sum == sum,
            "\nnumbers: {:?}\ntarget: {}\nresult: {:?}\nnumber as index {}: {}\nnumber as index {}: {}\nsum of the two numbers: {}\nsum of the two numbers did not equal target",
            nums, sum, user_tuple, user_tuple.0, num1, user_tuple.1, num2, user_sum
        )
  }
}
