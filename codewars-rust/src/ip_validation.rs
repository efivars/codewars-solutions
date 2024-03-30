#![allow(dead_code)]
/**
 * (6 kyu)
 * https://www.codewars.com/kata/515decfd9dcfc23bb6000006/train/rust
 *
 * Write an algorithm that will identify valid IPv4 addresses in dot-decimal format.
 * IPs should be considered valid if they consist of four octets, with values between 0 and 255, inclusive.
 */
fn is_decimal_num_string(s: String) -> bool {
  let chars: Vec<char> = s.chars().collect();
  let is_numeric = chars.clone().into_iter().all(|c| c.is_digit(10));

  match s.len() {
    0 => false,
    1 => is_numeric,
    _ => chars[0] != '0' && is_numeric,
  }
}

fn is_valid_ip(ip: &str) -> bool {
  let chunks: Vec<&str> = ip.split(".").collect();

  match chunks.len() {
    4 => chunks.into_iter().all(|chunk| {
      let num_str = chunk.to_string();
      let num = num_str.parse::<i64>().unwrap_or(-1);

      is_decimal_num_string(num_str) && num >= 0 && num <= 255
    }),
    _ => false,
  }
}

#[cfg(test)]
mod tests {
  use super::is_valid_ip;

  #[test]
  fn sample_test() {
    assert!(is_valid_ip("0.0.0.0"));
    assert!(is_valid_ip("12.255.56.1"));
    assert!(is_valid_ip("137.255.156.100"));

    assert!(!is_valid_ip(""));
    assert!(!is_valid_ip("abc.def.ghi.jkl"));
    assert!(!is_valid_ip("123.456.789.0"));
    assert!(!is_valid_ip("12.34.56"));
    assert!(!is_valid_ip("01.02.03.04"));
    assert!(!is_valid_ip("256.1.2.3"));
    assert!(!is_valid_ip("1.2.3.4.5"));
    assert!(!is_valid_ip("123,45,67,89"));
    assert!(!is_valid_ip("1e0.1e1.1e2.2e2"));
    assert!(!is_valid_ip(" 1.2.3.4"));
    assert!(!is_valid_ip("1.2.3.4 "));
    assert!(!is_valid_ip("12.34.56.-7"));
    assert!(!is_valid_ip("1.2.3.4\n"));
    assert!(!is_valid_ip("\n1.2.3.4"));
  }
}
