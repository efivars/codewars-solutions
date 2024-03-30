#![allow(dead_code)]
/**
 * (5 kyu)
 * https://www.codewars.com/kata/5550d638a99ddb113e0000a2/train/rust
 *
 * You are now to create a function that returns a Josephus permutation,
 * taking as parameters the initial array/list of items to be permuted as if
 * they were in a circle and counted out every k places until none remained.
 */

fn josephus<T: Clone + Copy>(xs: Vec<T>, k: usize) -> Vec<T> {
  if xs.len() < 2 || k < 2 {
    return xs;
  }

  let mut victims_that_left = xs.clone();
  let mut victims_in_order: Vec<T> = Vec::with_capacity(xs.len());

  let mut next_victim_index: Option<usize> = None;
  while victims_that_left.len() > 1 {
    next_victim_index = match next_victim_index {
      Some(value) => Some((value + k - 1) % victims_that_left.len()),
      None => Some((k - 1) % victims_that_left.len()),
    };

    let victim = victims_that_left.remove(next_victim_index.unwrap());
    victims_in_order.push(victim);
  }

  victims_in_order.append(&mut victims_that_left);
  victims_in_order
}

#[cfg(test)]
mod josephus {
  use super::josephus;

  #[test]
  fn test_works_with_integers() {
    assert_eq!(
      josephus(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 1),
      vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    );
    assert_eq!(
      josephus(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 2),
      vec![2, 4, 6, 8, 10, 3, 7, 1, 9, 5]
    );
  }
  #[test]
  fn test_works_with_strings() {
    assert_eq!(
      josephus("CodeWars".chars().collect::<Vec<char>>(), 4),
      "esWoCdra".chars().collect::<Vec<char>>()
    );
  }
}
