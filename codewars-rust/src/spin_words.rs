#![allow(dead_code)]
/**
 * (6 kyu)
 * https://www.codewars.com/kata/5264d2b162488dc400000001/train/rust
 * 
 * Write a function that takes in a string of one or more words,
 * and returns the same string, but with all words that have
 * five or more letters reversed (Just like the name of this Kata).
 * Strings passed in will consist of only letters and spaces.
 * Spaces will be included only when more than one word is present.
 */

fn spin_words(sentence: &str) -> String {
    let words: Vec<String> = sentence
        .split(" ")
        .map(|word| {
            if word.len() >= 5 {
                word.chars().rev().collect::<String>()
            } else {
                word.into()
            }
        })
        .collect();

    return words.join(" ");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spins_mixed_sentence() {
        let samples = [
            ("Hey fellow warriors", "Hey wollef sroirraw"),
            ("This is a test", "This is a test"),
            ("This is another test", "This is rehtona test"),
        ];

        for (input, expected) in samples {
            assert_eq!(spin_words(input), expected)
        }
    }
}
