#![allow(dead_code)]
/**
 * (7 kyu)
 * https://www.codewars.com/kata/51f2d1cafc9c0f745c00037d/train/rust
 *
 * Complete the solution so that it returns true if the first argument(string)
 * passed in ends with the 2nd argument (also a string).
 */

fn solution(word: &str, ending: &str) -> bool {
  word.ends_with(ending)
}
