#[macro_use] extern crate lazy_static;

use std::fs;
use std::io::{self, BufRead};

use regex::Regex;

pub fn read_input(filepath: &str) -> Result<Vec<(char, char)>, String> {
  let file = fs::File::open(filepath)
    .map_err(|e| e.to_string())?;

  io::BufReader::new(file)
    .lines()
    .map(|line_r| parse(&line_r.map_err(|e| e.to_string())?))
    .collect::<Result<Vec<(char, char)>, _>>()
}

fn parse(input: &String) -> Result<(char, char), String> {
  lazy_static! {
    static ref CHARS_RE: Regex = Regex::new(r"Step ([A-Z]{1}) must be finished before step ([A-Z]{1}) can begin.").unwrap();
  }

  let captures = CHARS_RE
    .captures(input)
    .ok_or("could not parse line")?;

  let mut chr_iter = captures
    .iter()
    .skip(1)
    .take(2)
    .map(|o| o.unwrap())
    .map(|m| m.as_str().as_bytes()[0] as char);

  let left = chr_iter.next().ok_or("error unpacking left character")?;
  let right = chr_iter.next().ok_or("error unpacking right character")?;

  Ok((left, right))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_test_001() {
    let input = "Step T must be finished before step X can begin.";
    let expected = Ok(('T', 'X'));
    let actual = parse(&input.to_string());

    assert_eq!(expected, actual);
  }

  #[test]
  fn parse_test_002() {
    let input = "Step T must be finished before step _ can begin.";
    let expected = Err("could not parse line".to_string());
    let actual = parse(&input.to_string());

    assert_eq!(expected, actual);
  }

  #[test]
  fn parse_test_003() {
    let input = "Step T must be finished before step 🔥 can begin.";
    let expected = Err("could not parse line".to_string());
    let actual = parse(&input.to_string());

    assert_eq!(expected, actual);
  }

  #[test]
  fn parse_test_004() {
    let input = "Random content.";
    let expected = Err("could not parse line".to_string());
    let actual = parse(&input.to_string());

    assert_eq!(expected, actual);
  }
}
