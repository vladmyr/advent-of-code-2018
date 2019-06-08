use std::fs::File;
use std::io::prelude::*;
use std::collections::BinaryHeap;

pub fn read_input(filepath: &str) -> Result<Vec<char>, String> {
  let mut file = File::open(filepath).map_err(|e| e.to_string())?;
  let mut contents = String::new();
  file.read_to_string(&mut contents).map_err(|e| e.to_string())?;

  Ok(contents.chars().collect())
}

fn get_is_matching(a: &char, b: &char) -> bool {
  (a.is_uppercase() && a.to_ascii_lowercase() == *b)
    || (a.is_lowercase() && a.to_ascii_uppercase() == *b)
}

pub fn calc_part1(input: &Vec<char>) -> usize {
  let mut heap: BinaryHeap<usize> = [0]
    .into_iter()
    .cloned()
    .collect();

  for (i, c) in input.iter().enumerate().skip(1) {
    match heap.peek() {
      None => heap.push(i),
      Some(j) => match get_is_matching(c, &input[*j]) {
        false => heap.push(i),
        true => { heap.pop(); },
      }
    };
  }

  heap.len()
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

  #[test]
  fn calc_part1_test() {
    assert_eq!(calc_part1(&String::from("dabAcCaCBAcCcaDA").chars().collect()), 10);
  }
}
