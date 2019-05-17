use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::collections::HashSet;

pub fn read_input(filepath: &str) -> Result<Vec<isize>, Error>{
  let file = File::open(filepath)?;
  let lines: Vec<isize> = BufReader::new(file)
    .lines()
    .map(|line| line.unwrap().parse::<isize>().unwrap())
    .collect();

  Ok(lines)
}

pub fn calc_part1(input: &Vec<isize>) -> isize {
  input.iter().fold(0, |a, b| a + b)
}

pub fn calc_part2(input: &Vec<isize>) -> isize {
  let mut cycle = input.iter().cycle();
  let mut result = 0;
  let mut set: HashSet<isize> = HashSet::new();

  while !set.contains(&result) {
    set.insert(result);
    result += cycle.next().unwrap();
  }

  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part2() {
    assert_eq!(calc_part2(&vec![1, -1]), 0);
    assert_eq!(calc_part2(&vec![3, 3, 4, -2, -4]), 10);
    assert_eq!(calc_part2(&vec![-6, 3, 8, 5, -6]), 5);
    assert_eq!(calc_part2(&vec![7, 7, -2, -7, -4]), 14);
  }
}