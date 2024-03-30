#![allow(dead_code)]
/**
 * (5 kyu)
 * https://www.codewars.com/kata/51fc12de24a9d8cb0e000001/train/rust
 *
 * ISBN-10 identifiers are ten digits long.
 * The first nine characters are digits 0-9.
 * The last digit can be 0-9 or X, to indicate a value of 10.
 *
 * An ISBN-10 number is valid if the sum of the digits multiplied by their position modulo 11 equals zero.
 */

fn valid_isbn10(isbn: &str) -> bool {
  if isbn.len() != 10 {
    return false;
  }
  if !&isbn[0..9].chars().all(|c| c.is_numeric()) {
    return false;
  }
  {
    let last_char = &isbn[9..=9].chars().next().unwrap();

    if !(last_char.is_numeric() || last_char == &'X') {
      return false;
    }
  }

  isbn
    .chars()
    .map(|c| c.to_digit(10).unwrap_or(10))
    .enumerate()
    .fold(0, |sum, (index, digit)| {
      let position = index + 1;
      dbg!(&sum);
      sum + (digit * position as u32)
    })
    % 11
    == 0
}

#[cfg(test)]
mod tests {
  use super::valid_isbn10;

  fn do_test(isbn: &str, expected: bool) {
    let actual = valid_isbn10(isbn);
    assert!(
      actual == expected,
      "Test failed with isbn = {isbn}\nExpected {expected} but got {actual}"
    )
  }

  #[test]
  fn sample_tests() {
    do_test("1112223339", true);
    do_test("048665088X", true);
    do_test("1293000000", true);
    do_test("1234554321", true);
    do_test("1234512345", false);
    do_test("1293", false);
    do_test("X123456788", false);
    do_test("ABCDEFGHIJ", false);
    do_test("XXXXXXXXXX", false);
    do_test("123456789T", false);
  }
}
