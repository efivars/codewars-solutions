#![allow(dead_code)]
/**
 * (6 kyu)
 * https://www.codewars.com/kata/545cedaa9943f7fe7b000048/train/rust
 *
 * A pangram is a sentence that contains every single letter of the alphabet
 * at least once. For example, the sentence "The quick brown fox jumps
 * over the lazy dog" is a pangram, because it uses
 * the letters A-Z at least once (case is irrelevant).
 *
 * Given a string, detect whether or not it is a pangram.
 * Return True if it is, False if not. Ignore numbers and punctuation.
 */
use std::collections::HashSet;

fn make_charset() -> HashSet<char> {
    const A_CHARCODE: u8 = 'a' as u8;
    const Z_CHARCODE: u8 = 'z' as u8;

    let mut chars = HashSet::new();
    let mut char_code = A_CHARCODE as u8;

    while char_code <= Z_CHARCODE {
        chars.insert((char_code as char).to_owned());
        char_code += 1;
    }

    chars
}

fn is_pangram(s: &str) -> bool {
    let mut charset = make_charset();

    s.to_ascii_lowercase().chars().for_each(|ch| {
        charset.remove(&ch);
    });

    charset.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_pangrams() {
        let samples = [
            (("The quick, brown fox jumps over the lazy dog!"), (true)),
            (("Cwm fjord bank glyphs vext quiz"), (true)),
            (("Pack my box with five dozen liquor jugs."), (true)),
            (("How quickly daft jumping zebras vex."), (true)),
            (("ABCD45EFGH,IJK,LMNOPQR56STUVW3XYZ"), (true)),
            (("This isn't a pangram!"), (false)),
            (("abcdefghijklmopqrstuvwxyz"), (false)),
            (("Aacdefghijklmnopqrstuvwxyz"), (false)),
        ];

        for (input, expected) in samples {
            assert_eq!(is_pangram(input), expected)
        }
    }
}
