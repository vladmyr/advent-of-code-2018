use std::fs::File;
use std::io::prelude::*;
use std::collections::BinaryHeap;

pub fn read_input(filepath: &str) -> Result<Vec<char>, String> {
  let mut file = File::open(filepath).map_err(|e| e.to_string())?;
  let mut contents = String::new();
  file.read_to_string(&mut contents);

  Ok(contents.chars().collect())
}

fn get_is_matching(a: &char, b: &char) -> bool {
  (a.is_uppercase() && a.to_ascii_lowercase() == *b)
    || (a.is_lowercase() && a.to_ascii_uppercase() == *b)
}

pub fn calc_part1(input: &Vec<char>) -> usize {
  let mut heap: BinaryHeap<usize> = BinaryHeap::new();
  for (i, c) in input.iter().enumerate() {
    
  }

  0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn get_is_matching_test() {
    assert!(get_is_matching(&'a', &'A'));
    assert!(get_is_matching(&'A', &'a'));
    assert!(!get_is_matching(&'a', &'a'));
    assert!(!get_is_matching(&'a', &'b'));
    assert!(!get_is_matching(&'A', &'b'));
    assert!(!get_is_matching(&'a', &'B'));
  }
}
