#![allow(dead_code)]
/**
 * (6 kyu)
 * https://www.codewars.com/kata/56a5d994ac971f1ac500003e/train/rust
 *
 * You are given an array(list) strarr of strings and an integer k.
 * Your task is to return the first longest string consisting of k
 * consecutive strings taken in the array.
 */
use std::cmp;

// FIXME
fn longest_consec(strings: Vec<&str>, k: usize) -> String {
    dbg!(&strings.len());
    let last_iter_index = cmp::max((strings.len() as i32) - k as i32, 0);
    dbg!(&last_iter_index);

    let mut consecutive_strings: Vec<String> = strings
        .clone()
        .into_iter()
        .enumerate()
        .map(|(index, _)| {
            dbg!(&index);
            if index > last_iter_index as usize || strings.len() == 0 {
                String::from("")
            } else {
                ((index)..(cmp::min(strings.len() - 1, index + k)))
                    .map(|i| strings[i])
                    .collect::<String>()
            }
        })
        .collect();

    dbg!(&consecutive_strings);
    consecutive_strings.sort_by(|a, b| b.len().cmp(&a.len()));

    String::from(
        &consecutive_strings
            .get(0)
            .unwrap_or(&"".to_owned())
            .to_owned(),
    )
}

fn testing(strarr: Vec<&str>, k: usize, exp: &str) -> () {
    assert_eq!(&longest_consec(strarr, k), exp)
}

#[test]
fn basics_longest_consec() {
    testing(
        vec!["zone", "abigail", "theta", "form", "libe", "zas"],
        2,
        "abigailtheta",
    );
    testing(
        vec![
            "ejjjjmmtthh",
            "zxxuueeg",
            "aanlljrrrxx",
            "dqqqaaabbb",
            "oocccffuucccjjjkkkjyyyeehh",
        ],
        1,
        "oocccffuucccjjjkkkjyyyeehh",
    );
    testing(vec![], 3, "");
    testing(
        vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"],
        3,
        "ixoyx3452zzzzzzzzzzzz",
    );
    testing(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 15, "");
    testing(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 0, "");
}
