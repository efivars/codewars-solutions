#![allow(dead_code)]
/**
 * (6 kyu)
 * https://www.codewars.com/kata/54b42f9314d9229fd6000d9c/train/rust
 *
 * The goal of this exercise is to convert a string to a new string where each character
 * in the new string is "(" if that character appears only once in the original string,
 * or ")" if that character appears more than once in the original string.
 * Ignore capitalization when determining if a character is a duplicate.
 */

const DUP_CHAR: char = ')';
const ONCE_CHAR: char = '(';

fn has_dups(string: &str, char: char) -> bool {
  string
    .to_ascii_lowercase()
    .match_indices(char.to_ascii_lowercase())
    .count()
    > 1
}

fn duplicate_encode(word: &str) -> String {
  word
    .chars()
    .map(|char| match has_dups(word, char) {
      true => DUP_CHAR,
      false => ONCE_CHAR,
    })
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn encodes() {
    let samples = [
      (("din"), ("(((")),
      (("recede"), ("()()()")),
      (("Success"), (")())())")),
      (("(( @"), ("))((")),
    ];

    for (input, expected) in samples {
      assert_eq!(duplicate_encode(input), expected)
    }
  }
}
