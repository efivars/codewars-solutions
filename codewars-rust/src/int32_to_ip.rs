#![allow(dead_code)]
/**
 * (5 kyu)
 * https://www.codewars.com/kata/52e88b39ffb6ac53a400022e/train/rust
 *
 * Complete the function that takes an unsigned 32 bit number
 * and returns a string representation of its IPv4 address.
 */

fn int32_to_ip(num: u32) -> String {
  let a = (num & 0xFF_00_00_00) >> 24;
  let b = (num & 0xFF_00_00) >> 16;
  let c = (num & 0xFF_00) >> 8;
  let d = num & 0xFF;

  format!("{}.{}.{}.{}", a, b, c, d)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn basic() {
    assert_eq!(int32_to_ip(2154959208), "128.114.17.104");
    assert_eq!(int32_to_ip(2149583361), "128.32.10.1");
    assert_eq!(int32_to_ip(0), "0.0.0.0");
  }
}
