use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Claim {
  startX: usize,
  startY: usize,
  endX: usize,
  endY: usize,
}

fn parse(input: &String) -> Result<Claim, String> {
  let re = Regex::new(r".*@\s(\d+),(\d+):\s(\d+)x(\d+)")
    .unwrap();

  let caps = re
    .captures(input)
    .ok_or("could not match input")?;

  let matches = caps
    .iter()
    .skip(1)
    .map(|o| o.unwrap())
    .map(|m| m.as_str())
    .map(|s| s.parse::<usize>().map_err(|e| e.to_string()))
    .collect::<Result<Vec<usize>, _>>()?;

  Ok(Claim { 
    startX: matches[0], 
    startY: matches[1], 
    endX: matches[0] + matches[2],
    endY: matches[1] + matches[3],
  })
}

pub fn read_input(filepath: &str) -> Result<Vec<Claim>, String> {
  let file = File::open(filepath).map_err(|e| e.to_string())?;
  let claim_results = BufReader::new(file)
    .lines()
    .map(|line_r| parse(&line_r.map_err(|e| e.to_string())?))
    .collect::<Result<Vec<Claim>, _>>()?;

  Ok(claim_results)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_sucess_test() {
    assert_eq!(
      parse(&String::from("#1 @ 1,1: 1x1")).unwrap(), 
      Claim { startX: 1, startY: 1, endX: 2, endY: 2}
    );
    
    assert_eq!(parse(&String::from("#1 @ 1,1: 1x")).is_ok(), false);
    assert_eq!(parse(&String::from("#1 @ 1,1: x1")).is_ok(), false);
    assert_eq!(parse(&String::from("#1 @ 1,: 1x1")).is_ok(), false);
    assert_eq!(parse(&String::from("#1 @ ,1: 1x1")).is_ok(), false);
    assert_eq!(parse(&String::from("#1 @ ,: 1x1")).is_ok(), false);
    assert_eq!(parse(&String::from("#1 @ ,: x")).is_ok(), false);
    assert_eq!(parse(&String::from("#1 @ a,b: -1x-2")).is_ok(), false);

    assert_eq!(parse(&String::from("")).is_ok(), false);
    assert_eq!(parse(&String::from("#1 1,1: 1x1")).is_ok(), false);
    assert_eq!(parse(&String::from("#1 @ 1,1 1x1")).is_ok(), false);
    assert_eq!(parse(&String::from("#1 @ 1,1 @ 1x1")).is_ok(), false);
    assert_eq!(parse(&String::from("#1 : 1,1 : 1x1")).is_ok(), false);
    assert_eq!(parse(&String::from("#1 , 1,1 , 1x1")).is_ok(), false);
    assert_eq!(parse(&String::from("#1 , 1, , x1")).is_ok(), false);
  }
}
