#![allow(dead_code)]
/**
 * (6 kyu)
 * https://www.codewars.com/kata/5842df8ccbd22792a4000245/train/rust
 *
 * You will be given a number and you will need to return it as a string in Expanded Form
 */

fn expanded_form(number: u64) -> String {
    let mut flattens: Vec<u64> = Vec::new();

    let mut rest = number;
    let mut multiplier = 1;
    loop {
        let remainder = rest % 10;
        rest = rest / 10;
        if remainder > 0 {
            flattens.push(remainder * multiplier);
        }
        multiplier *= 10;

        if rest == 0 {
            break;
        }
    }

    flattens
        .into_iter()
        .rev()
        .map(|number| number.to_string())
        .collect::<Vec<String>>()
        .join(" + ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(expanded_form(12), "10 + 2");
        assert_eq!(expanded_form(42), "40 + 2");
        assert_eq!(expanded_form(70304), "70000 + 300 + 4");
    }
}
