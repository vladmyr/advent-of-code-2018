use std::fs::File;
use std::io::{BufReader, BufRead};
use std::cmp;
use std::collections::{HashSet, HashMap};

pub fn read_input(filepath: &str) -> Result<Vec<(u64, u64)>, String> {
  let file = File::open(filepath).map_err(|e| e.to_string())?;
  
  BufReader::new(file)
    .lines()
    .map(|line_r| parse(&line_r.map_err(|e| e.to_string())?))
    .try_fold(vec![], |mut vec, parse_r| {
      match parse_r {
        Ok(t) => {
          vec.push(t);
          Ok(vec)
        },
        Err(e) => Err(e),
      }
    })
}

fn parse(input: &String) -> Result<(u64, u64), String> {
  input
    .split(", ")
    .enumerate()
    .try_fold((0, 0), |(t0, t1), (i, s)| {
      match (i, s.parse::<u64>()) {
        (0, Ok(v)) => Ok((v, t1)),
        (1, Ok(v)) => Ok((t0, v)),
        _ => Err("input does not meet parsing criteria".to_string()),
      }
    })
}

fn calc_grid_size(input: &Vec<(u64, u64)>) -> (u64, u64) {
  input
    .iter()
    .fold((0_u64, 0_u64), |(acc_l, acc_r), (l, r)| (cmp::max(acc_l, *l), cmp::max(acc_r, *r)))
}

fn calc_distance((a_l, a_r): &(u64, u64), (b_l, b_r): &(u64, u64)) -> u64 {
  ((*a_l as i64 - *b_l as i64).abs() + (*a_r as i64 - *b_r as i64).abs()) as u64
}

fn calc_closest(target: &(u64, u64), base: &Vec<(u64, u64)>) -> Option<(u64, u64)> {
  base
    .iter()
    .map(|b| (calc_distance(b, target), *b))
    .try_fold((true, (u64::max_value(), (0, 0))), |(is_unique, current), next| match current.0.cmp(&next.0) {
      cmp::Ordering::Equal => Some((false, current)),
      cmp::Ordering::Greater => Some((true, next)),
      cmp::Ordering::Less => Some((is_unique, current)),
    })
    .map(|(is_unique, (_, closest))| {
      match is_unique {
        false => None,
        true => Some(closest),
      }
    })
    .unwrap()
}

fn calc_area(input: &Vec<(u64, u64)>) -> HashMap<(u64, u64), u64> {
  let (x, y) = calc_grid_size(input);

  (0..=x)
    .flat_map(|c| (0..=y)
      .map(move |r| (c, r)))
    .fold(HashMap::new() as HashMap<(u64, u64), u64>, |mut acc, target| {
      if let Some(base) = calc_closest(&target, input) {
        let entry = acc.entry(base).or_insert(0);

        if target.0 == 0 || target.0 == x || target.1 == 0 || target.1 == y {
          *entry = u64::max_value();
        } else {
          *entry = match entry.checked_add(1) {
            Some(inc) => inc,
            None => *entry
          };
        }
      }

      acc
    })
}

pub fn calc_part1(input: &Vec<(u64, u64)>) -> u64 {
  calc_area(input)
    .into_iter()
    .filter(|(_, v)| *v < u64::max_value())
    .map(|(_, v)| v)
    .max_by(|v1, v2| v1.cmp(v2))
    .unwrap_or(0)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_test_001() {
    let input = "0, 0".to_string();
    let expected = (0, 0);
    let actual_r = parse(&input);

    assert!(actual_r.is_ok());
    assert_eq!(expected, actual_r.unwrap());
  }

  #[test]
  fn parse_test_002() {
    let input = "12, 67".to_string();
    let expected = (12, 67);
    let actual_r = parse(&input);

    assert!(actual_r.is_ok());
    assert_eq!(expected, actual_r.unwrap());
  }

  #[test]
  fn parse_test_003() {
    assert!(parse(&"0,0".to_string()).is_err());
  }

  #[test]
  fn parse_test_004() {
    assert!(parse(&"invalid string".to_string()).is_err());
  }

  #[test]
  fn calc_grid_size_test_001() {
    let input = vec![
      (0, 0),
      (7, 21),
      (1, 3),
      (5, 7),
      (39, 15),
      (28, 14),
    ];
    assert_eq!(calc_grid_size(&input), (39, 21));
  }

  #[test]
  fn calc_grid_size_test_002() {
    assert_eq!(calc_grid_size(&Vec::new()), (0, 0));
  }

  #[test]
  fn calc_distance_test_001() {
    assert_eq!(calc_distance(&(0, 0), &(0, 0)), 0);
  }

  #[test]
  fn calc_distance_test_002() {
    assert_eq!(calc_distance(&(0, 0), &(1, 0)), 1);
    assert_eq!(calc_distance(&(0, 0), &(0, 1)), 1);
    assert_eq!(calc_distance(&(0, 0), &(1, 1)), 2);
  }

  #[test]
  fn calc_closest_test_001() {
    let target = (3, 1);
    let base = vec![(1, 1), (4, 3)];
    assert_eq!(calc_closest(&target, &base), Some(base[0]));
  }

  #[test]
  fn calc_closest_test_002() {
    let target = (4, 1);
    let base = vec![(1, 1), (4, 3), (6, 1)];
    assert_eq!(calc_closest(&target, &base), None);
  }

  #[test]
  fn calc_closest_test_003() {
    let target = (6, 7);
    let base = vec![(1, 1), (6, 1), (3, 8), (4, 3), (5, 5), (9, 8)];
    assert_eq!(calc_closest(&target, &base), Some(base[4]));
  }

  #[test]
  fn calc_closest_test_004() {
    let target = (6, 8);
    let base = vec![(1, 1), (6, 1), (3, 8), (4, 3), (5, 5), (9, 8)];
    assert_eq!(calc_closest(&target, &base), None);
  }

  #[test]
  fn calc_closest_test_005() {
    let target = (5, 3);
    let base = vec![(1, 1), (6, 1), (3, 8), (4, 3), (5, 5), (9, 8)];
    assert_eq!(calc_closest(&target, &base), Some(base[3]));
  }

  #[test]
  fn calc_closest_test_006() {
    let target = (8, 3);
    let base = vec![(1, 1), (6, 1), (3, 8), (4, 3), (5, 5), (9, 8)];
    assert_eq!(calc_closest(&target, &base), None);
  }

  #[test]
  fn calc_area_test_001() {
    let input = vec![(1, 1), (6, 1), (3, 8), (4, 3), (5, 5), (9, 8)];
    let expected = vec![
      ((1, 1), u64::max_value()),
      ((6, 1), u64::max_value()),
      ((3, 8), u64::max_value()),
      ((4, 3), 9),
      ((5, 5), 17),
      ((9, 8), u64::max_value()),
    ]
      .into_iter()
      .collect::<HashMap<(u64, u64), u64>>();

    assert_eq!(calc_area(&input), expected);
  }
}
