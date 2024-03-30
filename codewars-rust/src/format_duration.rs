#![allow(dead_code)]
/**
 * (4 kyu)
 * https://www.codewars.com/kata/52742f58faf5485cae000b9a/train/rust
 *
 * Your task in order to complete this Kata is to write a function which formats a duration,
 * given as a number of seconds, in a human-friendly way.
 *
 * The function must accept a non-negative integer.
 * If it is zero, it just returns "now". Otherwise, the duration is expressed
 * as a combination of years, days, hours, minutes and seconds.
 */

fn format_plural(string: &str, count: u64) -> String {
  match count {
    0 => "".to_string(),
    1 => format!("{} {}", count, string),
    _ => format!("{} {}s", count, string),
  }
}

fn format_list(phrases: &Vec<String>) -> String {
  match phrases.len() {
    0 | 1 => phrases.join(""),
    2 => phrases.join(" and "),
    big_length => {
      let all_but_last = &phrases[0..(big_length - 1)];
      let last = &phrases[big_length - 1];

      format!("{} and {}", all_but_last.join(", "), *last)
    }
  }
}

const SECONDS_IN_MINUTE: u64 = 60;
const SECONDS_IN_HOUR: u64 = SECONDS_IN_MINUTE * 60;
const SECONDS_IN_DAY: u64 = SECONDS_IN_HOUR * 24;
const SECONDS_IN_YEAR: u64 = SECONDS_IN_DAY * 365;

fn format_duration(duration_seconds: u64) -> String {
  if duration_seconds == 0 {
    return "now".to_string();
  }

  let mut seconds_left = duration_seconds;

  let years = seconds_left / SECONDS_IN_YEAR;
  seconds_left %= SECONDS_IN_YEAR;

  let days = seconds_left / SECONDS_IN_DAY;
  seconds_left %= SECONDS_IN_DAY;

  let hours = seconds_left / SECONDS_IN_HOUR;
  seconds_left %= SECONDS_IN_HOUR;

  let minutes = seconds_left / SECONDS_IN_MINUTE;
  seconds_left %= SECONDS_IN_MINUTE;

  let seconds = seconds_left;

  let phrases = vec![
    format_plural("year", years),
    format_plural("day", days),
    format_plural("hour", hours),
    format_plural("minute", minutes),
    format_plural("second", seconds),
  ]
  .into_iter()
  .filter(|phrase| !phrase.is_empty())
  .collect();

  format_list(&phrases)
}

#[cfg(test)]
mod tests {
  use super::format_duration;

  #[test]
  fn test_basic() {
    assert_eq!(format_duration(1), "1 second");
    assert_eq!(format_duration(62), "1 minute and 2 seconds");
    assert_eq!(format_duration(120), "2 minutes");
    assert_eq!(format_duration(3600), "1 hour");
    assert_eq!(format_duration(3662), "1 hour, 1 minute and 2 seconds");
  }
}
