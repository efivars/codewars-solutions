#![allow(dead_code)]
/**
 * (6 kyu)
 * https://www.codewars.com/kata/517abf86da9663f1d2000003/train/rust
 *
 * Complete the method/function so that it converts dash/underscore delimited words
 * into camel casing. The first word within the output should be capitalized only if
 * the original word was capitalized (known as Upper Camel Case, also often referred
 * to as Pascal case). The next words should be always capitalized.
 */

fn capitalize(s: &str) -> String {
  let mut c = s.chars();

  match c.next() {
    None => String::new(),
    Some(f) => f.to_uppercase().chain(c).collect(),
  }
}

fn to_camel_case(text: &str) -> String {
  let separators = ['_', '-'];

  text
    .split(&separators)
    .enumerate()
    .map(|(index, str)| match index {
      0 => str.to_owned(),
      _ => capitalize(str),
    })
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn converts_to_camelcase() {
    let samples = [
      ("the-stealth-warrior", "theStealthWarrior"),
      ("The_Stealth_Warrior", "TheStealthWarrior"),
      ("The_Stealth-Warrior", "TheStealthWarrior"),
    ];

    for (input, expected) in samples {
      assert_eq!(to_camel_case(input), expected)
    }
  }
}
