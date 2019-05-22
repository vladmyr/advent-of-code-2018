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
  let mut v = input
    .into_iter()
    .filter(|&(_, value)| whitelist.contains(&value))
    .fold(Vec::new(), |mut vec, (_, value)| {
      if !vec.contains(&value) {
        vec.push(value);
      }
      vec
    });

  v.sort();
  v
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

fn hamming_distance(a: &String, b: &String) -> usize {
  a
    .chars()
    .zip(b.chars())
    .map(|(x, y)| if x == y { 0 } else { 1 })
    .fold(0, |a, b| a + b)
}

fn strip_unique_chars(a: &String, b: &String) -> String {
  a
    .chars()
    .zip(b.chars())
    .filter_map(|(x, y)| if x == y { Some(x) } else { None })
    .fold(String::new(), |mut acc, chr| {
      acc.push(chr);
      acc
    })
}

pub fn calc_part2(input: &Vec<String>) -> String {
  let (a, b) = input
    .iter()
    .take(input.len() - 1)
    .enumerate()
    .flat_map(|(i, a)| {
      input
        .iter()
        .skip(i + 1)
        .filter_map(move |b| if hamming_distance(a, b) == 1 {
          Some((a, b))
        } else {
          None
        })
    })
    .last()
    .unwrap();

  strip_unique_chars(a, b)
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

  #[test]
  fn hamming_distance_test() {
    assert_eq!(
      hamming_distance(&String::from("fghij"), &String::from("klmno")),
      5
    );

    assert_eq!(
      hamming_distance(&String::from("karolin"), &String::from("kathrin")),
      3
    );

    assert_eq!(
      hamming_distance(&String::from("karolin"), &String::from("kerstin")),
      3
    );

    assert_eq!(
      hamming_distance(&String::from("1011101"), &String::from("1001001")),
      2
    );

    assert_eq!(
      hamming_distance(&String::from("2173896"), &String::from("2233796")),
      3
    );
  }

  #[test]
  fn strip_unique_chars_test() {
    assert_eq!(
      strip_unique_chars(&String::from("fghij"), &String::from("fguij")), 
      String::from("fgij")
    );

    assert_eq!(
      strip_unique_chars(&String::from("0011001"), &String::from("1011001")), 
      String::from("011001")
    );

    assert_eq!(
      strip_unique_chars(&String::from(""), &String::from("")), 
      String::from("")
    );

    assert_eq!(
      strip_unique_chars(&String::from("a"), &String::from("b")), 
      String::from("")
    );
  }

  #[test]
  fn calc_part2_test() {
    assert_eq!(
      calc_part2(&vec![
        String::from("abcde"),
        String::from("fghij"),
        String::from("klmno"),
        String::from("pqrst"),
        String::from("fguij"),
        String::from("axcye"),
        String::from("wvxyz"),
      ]),
      String::from("fgij")
    );

    assert_eq!(
      calc_part2(&vec![
        String::from("fghij"),
        String::from("fguij"),
        String::from("abcde"),
        String::from("klmno"),
        String::from("pqrst"),
        String::from("axcye"),
        String::from("wvxyz"),
      ]),
      String::from("fgij")
    );

    assert_eq!(
      calc_part2(&vec![
        String::from("abcde"),
        String::from("klmno"),
        String::from("pqrst"),
        String::from("axcye"),
        String::from("wvxyz"),
        String::from("fghij"),
        String::from("fguij"),
      ]),
      String::from("fgij")
    );

    assert_eq!(
      calc_part2(&vec![
        String::from("fghij"),
        String::from("abcde"),
        String::from("klmno"),
        String::from("pqrst"),
        String::from("axcye"),
        String::from("wvxyz"),
        String::from("fguij"),
      ]),
      String::from("fgij")
    );
  }
}
