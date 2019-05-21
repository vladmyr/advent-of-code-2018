use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::collections::HashMap;

pub fn read_input(filepath: &str) -> Result<Vec<String>, Error>{
  let file = File::open(filepath)?;
  let lines = BufReader::new(file)
    .lines()
    .map(|line| line.unwrap())
    .collect::<Vec<String>>();

  Ok(lines)
}

fn count_char_ocurrences(input: &String) -> HashMap<char, usize> {
  input
    .chars()
    .fold(HashMap::new(), |mut map, chr| {
      let occurrences = map.entry(chr).or_insert(0);
      *occurrences += 1;
      map
    })
}

fn filter_unique_by_ocurrence_count(
  input: HashMap<char, usize>, 
  whitelist: &Vec<usize>
) -> Vec<usize> {
  input
    .into_iter()
    .filter(|&(_, value)| whitelist.contains(&value))
    .fold(Vec::new(), |mut vec, (_, value)| {
      if !vec.contains(&value) {
        vec.push(value);
      }
      vec
    })
}

pub fn calc_part1(input: &Vec<String>) -> usize {
  input
    .iter()
    .map(|box_id| count_char_ocurrences(box_id))
    .map(|map| filter_unique_by_ocurrence_count(map, &vec![2, 3]))
    .fold(HashMap::new(), |mut map, v| {
      for value in v {
        let count = map.entry(value).or_insert(0);
        *count += 1
      }
      map
    })
    .iter()
    .fold(1, |a, (_, b)| a * b)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn count_char_ocurrences_test() {
    assert_eq!(
      count_char_ocurrences(&String::from("abcdef")),
      [('a', 1), ('b', 1), ('c', 1), ('d', 1), ('e', 1), ('f', 1)]
        .iter().cloned().collect()
    );

    assert_eq!(
      count_char_ocurrences(&String::from("bababc")),
      [('b', 3), ('a', 2), ('c', 1)].iter().cloned().collect()
    );

    assert_eq!(
      count_char_ocurrences(&String::from("abbcde")),
      [('a', 1), ('b', 2), ('c', 1), ('d', 1), ('e', 1)]
        .iter().cloned().collect()
    );

    assert_eq!(
      count_char_ocurrences(&String::from("abcccd")),
      [('a', 1), ('b', 1), ('c', 3), ('d', 1)].iter().cloned().collect()
    );

    assert_eq!(
      count_char_ocurrences(&String::from("aabcdd")),
      [('a', 2), ('b', 1), ('c', 1), ('d', 2)].iter().cloned().collect()
    );

    assert_eq!(
      count_char_ocurrences(&String::from("abcdee")),
      [('a', 1), ('b', 1), ('c', 1), ('d', 1), ('e', 2)]
        .iter().cloned().collect()
    );

    assert_eq!(
      count_char_ocurrences(&String::from("ababab")),
      [('a', 3), ('b', 3)].iter().cloned().collect()
    );
  }

  #[test]
  fn filter_unique_by_ocurrence_count_test() {
    assert_eq!(
      filter_unique_by_ocurrence_count(
        [('a', 1), ('b', 1), ('c', 1), ('d', 1), ('e', 1), ('f', 1)]
          .iter().cloned().collect(), 
        &vec![2, 3]
      ),
      vec![]
    );

    assert_eq!(
      filter_unique_by_ocurrence_count(
        [('b', 3), ('a', 2), ('c', 1)].iter().cloned().collect(), 
        &vec![2, 3]
      ),
      vec![2, 3]
    );

    assert_eq!(
      filter_unique_by_ocurrence_count(
        [('a', 1), ('b', 2), ('c', 1), ('d', 1), ('e', 1)]
          .iter().cloned().collect(),
        &vec![2, 3]
      ),
      vec![2]
    );

    assert_eq!(
      filter_unique_by_ocurrence_count(
        [('a', 1), ('b', 1), ('c', 3), ('d', 1)].iter().cloned().collect(), 
        &vec![2, 3]
      ),
      vec![3]
    );

    assert_eq!(
      filter_unique_by_ocurrence_count(
        [('a', 2), ('b', 1), ('c', 1), ('d', 2)].iter().cloned().collect(),
        &vec![2, 3]
      ),
      vec![2]
    );

    assert_eq!(
      filter_unique_by_ocurrence_count(
        [('a', 1), ('b', 1), ('c', 1), ('d', 1), ('e', 2)]
          .iter().cloned().collect(),
        &vec![2, 3]
      ),
      vec![2]
    );

    assert_eq!(
      filter_unique_by_ocurrence_count(
        [('a', 3), ('b', 3)].iter().cloned().collect(),
        &vec![2, 3]
      ),
      vec![3]
    );
  }

  #[test]
  fn calc_part1_test() {
    assert_eq!(
      calc_part1(&vec![
        String::from("abcdef"), 
        String::from("bababc"), 
        String::from("abbcde"), 
        String::from("abcccd"), 
        String::from("aabcdd"), 
        String::from("abcdee"), 
        String::from("ababab"),
      ]),
      12
    );
  }
}
