#![allow(dead_code)]
/**
 * (5 kyu)
 * https://www.codewars.com/kata/5541f58a944b85ce6d00006a/train/rust
 *
 * Given a number, say prod (for product), we search two Fibonacci numbers F(n) and F(n+1) verifying
 * F(n) * F(n+1) = prod.
 */
use std::cmp::Ordering;

enum FibSeqMulResult {
  Exact(u64, u64),
  NonExact(u64, u64),
}

fn find_fib_seq_pair_multiplying_into(target: u64) -> FibSeqMulResult {
  let mut a = 0;
  let mut b = 1;
  let mut prod = 0;

  while prod < target {
    let next = a + b;

    a = b;
    b = next;

    prod = a * b;
  }

  match prod.cmp(&target) {
    Ordering::Equal => FibSeqMulResult::Exact(a, b),
    Ordering::Greater => FibSeqMulResult::NonExact(a, b),
    Ordering::Less => panic!("Impossible"),
  }
}

fn product_fib(target: u64) -> (u64, u64, bool) {
  match find_fib_seq_pair_multiplying_into(target) {
    FibSeqMulResult::Exact(a, b) => (a, b, true),
    FibSeqMulResult::NonExact(a, b) => (a, b, false),
  }
}

fn dotest(prod: u64, exp: (u64, u64, bool)) -> () {
  assert_eq!(product_fib(prod), exp)
}

#[test]
fn basics_product_fib() {
  dotest(4895, (55, 89, true));
  dotest(5895, (89, 144, false));
}
