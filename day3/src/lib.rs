use std::fs::File;
use std::io::{BufReader, BufRead};
use std::cmp::max;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Claim {
  start_x: usize,
  start_y: usize,
  len_x: usize,
  len_y: usize,
}

fn parse(input: &String) -> Result<Claim, String> {
  let re = Regex::new(r".*@\s(\d+),(\d+):\s(\d+)x(\d+)")
    .map_err(|e| e.to_string())?;

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
    start_x: matches[0], 
    start_y: matches[1], 
    len_x: matches[2],
    len_y: matches[3],
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

pub fn calc_part1(claims: &Vec<Claim>) -> usize {
  let (size_x, size_y) = claims
    .iter()
    .fold((0_usize, 0_usize), |(size_x, size_y), claim| {
      (max(size_x, claim.start_x + claim.len_x), max(size_y, claim.start_y + claim.len_y))
    });

  let fabric = claims
    .iter()
    .fold(vec![0; size_x * size_y], |mut fabric, claim| {
      for i in claim.start_x..(claim.start_x + claim.len_x) {
        for j in claim.start_y..(claim.start_y + claim.len_y) {
          fabric[j * size_x + i] += 1;
        }
      }

      fabric
    });

  fabric
    .iter()
    .filter(|&n| *n > 1)
    .count()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_test() {
    assert_eq!(
      parse(&String::from("#1 @ 1,1: 1x1")).unwrap(), 
      Claim { start_x: 1, start_y: 1, len_x: 1, len_y: 1 }
    );

    assert_eq!(
      parse(&String::from("#1 @ 1,1: 1x2")).unwrap(), 
      Claim { start_x: 1, start_y: 1, len_x: 1, len_y: 2 }
    );

    assert_eq!(
      parse(&String::from("#1 @ 1,1: 2x1")).unwrap(), 
      Claim { start_x: 1, start_y: 1, len_x: 2, len_y: 1 }
    );

    assert_eq!(
      parse(&String::from("#1 @ 3,2: 5x4")).unwrap(), 
      Claim { start_x: 3, start_y: 2, len_x: 5, len_y: 4 }
    );

    assert_eq!(
      parse(&String::from("#1 @ 5,5: 2x2")).unwrap(), 
      Claim { start_x: 5, start_y: 5, len_x: 2, len_y: 2 }
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

  #[test]
  fn calc_part1_test() {
    assert_eq!(calc_part1(&vec![
      Claim { start_x: 1, start_y: 3, len_x: 4, len_y: 4 },
      Claim { start_x: 3, start_y: 1, len_x: 4, len_y: 4 },
      Claim { start_x: 5, start_y: 5, len_x: 2, len_y: 2 },
    ]), 4);

    assert_eq!(calc_part1(&vec![
      Claim { start_x: 1, start_y: 1, len_x: 1, len_y: 2 },
      Claim { start_x: 1, start_y: 2, len_x: 2, len_y: 1 },
    ]), 1);
  }
}
