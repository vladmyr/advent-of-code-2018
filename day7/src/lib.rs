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
    ;

  Ok(vec![('A', 'B')])
}

fn parse(input: &String) -> Result<(char, char), String> {
  lazy_static! {
    static ref CHARS_RE: Regex = Regex::new(r"Step ([A-Z]{1}) must be finished before step ([A-Z]{1}) can begin.").unwrap();
  }

  CHARS_RE
    .captures(input)
    .ok_or("could not parse line")?
    .iter()
    .skip(1)
    .map(|o| o.unwrap())
    .map(|m| m.as_str().as_bytes()[0] as char)
    .enumerate()
    .scan('_' as char, |left, (idx, chr)| -> Option<(char, char)> {
      match idx {
        0 => {
          *left = chr;
          None
        },
        1 => Some((*left, chr)),
        _ => None,
      }
    })
    .last()
    .ok_or("could not produce a tuple".to_string())
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }

  fn parse_test_001() {
    
  }
}
