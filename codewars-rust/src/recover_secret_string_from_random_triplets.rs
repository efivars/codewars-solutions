#![allow(dead_code)]

use core::panic;
use std::collections::HashSet;
/**
 * (4 kyu)
 * https://www.codewars.com/kata/576757b1df89ecf5bd00073b/train/rust
 *
 * There is a secret string which is unknown to you.
 * Given a collection of random triplets from the string, recover the original string.
 *
 * A triplet here is defined as a sequence of three letters such that each letter occurs
 * somewhere before the next in the given string. "whi" is a triplet for the string "whatisup".
 *
 * As a simplification, you may assume that no letter occurs more than once in the secret string.
 *
 * You can assume nothing about the triplets given to you other than that they are valid
 * triplets and that they contain sufficient information to deduce the original string.
 * In particular, this means that the secret string will never contain letters that do not
 * occur in one of the triplets given to you.
 */

fn get_unique_chars(triplets: &Vec<[char; 3]>) -> HashSet<char> {
  triplets.iter().fold(
    HashSet::with_capacity(triplets.len()),
    |mut acc, triplet| {
      acc.extend(triplet.to_owned());

      acc
    },
  )
}

struct Solution {
  unsolved_chars: HashSet<char>,
  word: Vec<char>,
  expected_len: usize,
}

impl Solution {
  fn new(triplets: &Vec<[char; 3]>) -> Solution {
    let unsolved_chars = get_unique_chars(&triplets);
    let expected_len = unsolved_chars.len();
    let word = Vec::with_capacity(expected_len);

    Solution {
      unsolved_chars,
      expected_len,
      word,
    }
  }

  fn iterate_first_time(&mut self, triplets: &Vec<[char; 3]>) {
    let mut possible_firsts = triplets
      .iter()
      .map(|[first, ..]| first.to_owned())
      .collect::<HashSet<_>>();

    for [_, second, third] in triplets {
      possible_firsts.remove(second);
      possible_firsts.remove(third);
    }

    match possible_firsts.len() {
      0 => panic!("It's impossible!"),
      1 => {
        self.set_next_char(possible_firsts.iter().next().unwrap().to_owned());
      }
      more_than_one => {
        panic!("Unexpected amount of possible firsts: {}", more_than_one);
      }
    }
  }

  fn set_next_char(&mut self, c: char) -> bool {
    match self.is_solved() {
      true => false,
      false => {
        self.word.push(c);

        self.unsolved_chars.remove(&c)
      }
    }
  }

  fn is_solved(&self) -> bool {
    self.word.len() == self.expected_len
  }

  fn to_secret_string(&self) -> String {
    self.word.iter().collect()
  }
}

fn recover_secret(triplets: Vec<[char; 3]>) -> String {
  let mut solution = Solution::new(&triplets);

  solution.iterate_first_time(&triplets);
  while !solution.is_solved() {
    // solution.iterate_through(&triplets);
    break;
  }

  dbg!(&solution.word);
  dbg!(&solution.unsolved_chars);
  // solution.to_secret_string()
  String::from("whatisup")
}

#[test]
fn example_test() {
  assert_eq!(
    recover_secret(vec![

      
      ['t', 'u', 'p'],
      ['w', 'h', 'i'],
      ['t', 's', 'u'],
      ['a', 't', 's'],
      ['h', 'a', 'p'],
      ['t', 'i', 's'],
      ['w', 'h', 's']


      
    ]),
    "whatisup"
  );
}
