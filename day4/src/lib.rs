use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufReader, BufRead};

use regex::Regex;
use chrono::prelude::*;

#[derive(Debug, PartialEq)]
enum Action {
  Shift(usize),
  FallAsleep,
  WakeUp,
}

#[derive(Debug, PartialEq)]
pub struct Record {
  datetime: DateTime<Utc>,
  action: Action,
}

fn parse(input: &String) -> Result<Record, String> {
  let datetime_re = Regex::new(r"\[(\d+)-(\d+)-(\d+)\s(\d+):(\d+)\]")
    .map_err(|e| e.to_string())?;
  let shift_re = Regex::new(r".+#(\d)+.+")
    .map_err(|e| e.to_string())?;

  if input.len() < 18 {
    return Err("length of input does not meet the criteria".to_string());
  }

  let (datetime_input, action_input) = input.split_at(18);
  let datetime = datetime_re
    .captures(datetime_input)
    .ok_or("could not parse datetime")?
    .iter()
    .skip(1)
    .map(|o| o.unwrap())
    .map(|m| m.as_str())
    .map(|s| s.parse::<usize>().map_err(|e| e.to_string()))
    .collect::<Result<Vec<usize>, _>>()?;

  let action = if shift_re.is_match(action_input) {
    let shift_cap = shift_re
      .captures(action_input)
      .ok_or("could not parse shift action")?;

    let id = shift_cap[1]
      .parse::<usize>()
      .map_err(|e| e.to_string())?;

    Action::Shift(id)
  } else {
    match action_input.trim() {
      "wakes up" => Action::WakeUp,
      "falls asleep" => Action::FallAsleep,
      _ => return Err("could not parse action".to_string()),
    }
  };

  let datetime = Utc
    .ymd(datetime[0] as i32, datetime[1] as u32, datetime[2] as u32)
    .and_hms(datetime[3] as u32, datetime[4] as u32, 0);

  Ok(Record { datetime, action })
}

pub fn read_input(filepath: &str) -> Result<Vec<Record>, String> {
  let file = File::open(filepath).map_err(|e| e.to_string())?;
  let mut claim_results = BufReader::new(file)
    .lines()
    .map(|line_r| parse(&line_r.map_err(|e| e.to_string())?))
    .collect::<Result<Vec<Record>, _>>()?;
    
  claim_results.sort_by(|a, b| if a.datetime < b.datetime { 
      Ordering::Less 
    } else { 
      Ordering::Greater 
  });

  Ok(claim_results)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_test() {
    let result = parse(
        &String::from("[1970-01-01 23:59] Guard #1 begins shift")
      )
      .unwrap();

    let expected = Record {
      datetime: Utc.ymd(1970, 1, 1).and_hms(23, 59, 0),
      action: Action::Shift(1)
    };

    assert_eq!(result.action, expected.action);
    assert_eq!(result.datetime, expected.datetime);

    assert!(
      parse(&String::from("[1970-01-01 23:59] Guard begins shift")).is_err()
    );

    assert!(
      parse(&String::from("[1970-01-01 23:]Guard #1 begins shift")).is_err()
    );

    assert!(parse(&String::from("[1970-01-01 23:59]")).is_err());
    assert!(parse(&String::from("Guard #1 begins shift")).is_err());
    assert!(parse(&String::from("")).is_err());

    assert!(
      parse(&String::from("[1970-01-01 23:59] Guard #1 begins shift")).is_ok()
    );
  }
}
