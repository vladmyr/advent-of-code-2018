use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

use regex::Regex;

use chrono::prelude::*;
use chrono::Duration;

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

#[derive(Debug, PartialEq)]
struct AsleepRecord {
  id: usize,
  datetime: DateTime<Utc>,
  duration: Duration
}

fn parse(input: &String) -> Result<Record, String> {
  let datetime_re = Regex::new(r"\[(\d+)-(\d+)-(\d+)\s(\d+):(\d+)\]")
    .map_err(|e| e.to_string())?;
  let shift_re = Regex::new(r".+#(\d+).+")
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
    
  claim_results.sort_by(|a, b| if a.datetime.lt(&b.datetime) { 
      Ordering::Less 
    } else { 
      Ordering::Greater 
  });

  Ok(claim_results)
}

fn filter_map_asleep(records: &Vec<Record>) -> Vec<AsleepRecord> {
  records
    .iter()
    .scan((0_usize, Utc::now()), |(id, start), r| {
      match r.action {
        Action::Shift(new_id) => {
          *id = new_id;
          Some(None)
        },
        Action::FallAsleep => {
          *start = r.datetime;
          Some(None)
        },
        Action::WakeUp => {
          // ToDo: is this enough to fully describe the problem?
          let duration = Duration::minutes(
            (r.datetime.time().minute() - start.time().minute()) as i64
          );
          
          Some(Some(AsleepRecord {
            id: *id, 
            datetime: *start, 
            duration 
          }))
        }
      }
    })
    .filter_map(|o| o)
    .filter(|r| r.datetime.time().hour() == 0)
    .collect()
}

fn find_sleepiest(records: &Vec<AsleepRecord>) -> Option<usize> {
  records
    .iter()
    .fold(HashMap::new() as HashMap<usize, u64>, |mut map, r| {
      let entry = map.entry(r.id).or_insert(0);
      *entry += r.duration.num_minutes() as u64;

      map
    })
    .iter()
    .fold(None as Option<(usize, u64)>, |acc, (&r_id, &r_count)| {
      match acc {
        None => Some((r_id, r_count)),
        Some((_, count)) if r_count > count => Some((r_id, r_count)),
        Some(t) => Some(t),
      }
    })
    .map(|(id, _)| id)
}

fn get_is_intersecting(a: &AsleepRecord, t: &NaiveTime) -> Option<bool> {
  let a_end = a.datetime.checked_add_signed(a.duration)?;
  Some(a.datetime.time().lt(&t) && t.lt(&a_end.time()))
}

fn find_intersection_beginning(a: &AsleepRecord, b: &AsleepRecord) -> Option<DateTime<Utc>> {
  match (
    get_is_intersecting(a, &b.datetime.time())?,
    get_is_intersecting(b, &a.datetime.time())?,
  ) {
    (false, true) => Some(a.datetime),
    (true, false) => Some(b.datetime),
    _ => None,
  }
}

fn find_sleepiest_minute(records: &Vec<AsleepRecord>) -> Option<u8> {
  records
    .iter()
    .take(records.len() - 1)
    .enumerate()
    .flat_map(|(i, a)| {
      records
        .iter()
        .skip(i + 1)
        .filter_map(move |b| find_intersection_beginning(a, b))
    })
    .fold(HashMap::new() as HashMap<NaiveTime, usize>, |mut map, datetime| {
      let entry = map.entry(datetime.time()).or_insert(0);
      *entry += 1;

      map
    })
    .iter()
    .max_by(|(_, a_count), (_, b_count)| if a_count < b_count {
      Ordering::Less
    } else {
      Ordering::Greater
    })
    .map(|(t, _)| t.minute() as u8)
}

fn find_sleepiest_id(records: &Vec<AsleepRecord>, t: &NaiveTime) -> Option<usize> {
  records
    .iter()
    .filter(|r| match get_is_intersecting(&r, &t) {
      Some(b) => b,
      _ => false,
    })
    .map(|r| r.id)
    .fold(HashMap::new() as HashMap<usize, usize>, |mut map, id| {
      let entry = map.entry(id).or_insert(0);
      *entry += 1;

      map
    })
    .iter()
    .max_by(|(_, a_count), (_, b_count)| if a_count < b_count {
      Ordering::Less
    } else {
      Ordering::Greater
    })
    .map(|(id, _)| *id)
}

pub fn calc_part1(records: &Vec<Record>) -> usize {
  let sleep_records = filter_map_asleep(&records);
  let sleepiest_id = find_sleepiest(&sleep_records)
    .unwrap();

  let sleepiest_records: Vec<AsleepRecord> = sleep_records
    .into_iter()
    .filter(|r| r.id == sleepiest_id)
    .collect();

  let sleepiest_minute: usize = find_sleepiest_minute(&sleepiest_records)
    .expect("cound not calculate the sleepies minute") as usize;

  sleepiest_id * sleepiest_minute
}

pub fn calc_part2(records: &Vec<Record>) -> usize {
  let sleep_records = filter_map_asleep(&records);
  let sleepiest_minute: usize = find_sleepiest_minute(&sleep_records)
    .expect("cound not calculate the sleepies minute") as usize;
  let t = NaiveTime::from_hms(0, sleepiest_minute as u32, 0);
  let sleepiest_id = find_sleepiest_id(&sleep_records, &t)
    .expect("count not calculate the sleepiest id");

  sleepiest_id * sleepiest_minute
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

  #[test]
  fn filter_map_asleep_test() {
    let v = vec![
      Record { 
        datetime: Utc.ymd(1518, 11, 1).and_hms(0, 0, 0),
        action: Action::Shift(10)
      },
      Record { 
        datetime: Utc.ymd(1518, 11, 1).and_hms(0, 5, 0),
        action: Action::FallAsleep
      },
      Record { 
        datetime: Utc.ymd(1518, 11, 1).and_hms(0, 25, 0),
        action: Action::WakeUp
      },
      Record { 
        datetime: Utc.ymd(1518, 11, 1).and_hms(0, 30, 0),
        action: Action::FallAsleep
      },
      Record { 
        datetime: Utc.ymd(1518, 11, 1).and_hms(0, 55, 0),
        action: Action::WakeUp
      },
      Record { 
        datetime: Utc.ymd(1518, 11, 1).and_hms(23, 58, 0),
        action: Action::Shift(99)
      },
      Record { 
        datetime: Utc.ymd(1518, 11, 2).and_hms(0, 40, 0),
        action: Action::FallAsleep
      },
      Record { 
        datetime: Utc.ymd(1518, 11, 2).and_hms(0, 50, 0),
        action: Action::WakeUp
      },
      Record { 
        datetime: Utc.ymd(1518, 11, 3).and_hms(0, 5, 0),
        action: Action::Shift(10)
      },
      Record { 
        datetime: Utc.ymd(1518, 11, 3).and_hms(0, 24, 0),
        action: Action::FallAsleep
      },
      Record { 
        datetime: Utc.ymd(1518, 11, 3).and_hms(0, 29, 0),
        action: Action::WakeUp
      },
      Record { 
        datetime: Utc.ymd(1518, 11, 4).and_hms(0, 2, 0),
        action: Action::Shift(99)
      },
      Record { 
        datetime: Utc.ymd(1518, 11, 4).and_hms(0, 36, 0),
        action: Action::FallAsleep
      },
      Record { 
        datetime: Utc.ymd(1518, 11, 4).and_hms(0, 46, 0),
        action: Action::WakeUp
      },
      Record { 
        datetime: Utc.ymd(1518, 11, 5).and_hms(0, 3, 0),
        action: Action::Shift(99)
      },
      Record { 
        datetime: Utc.ymd(1518, 11, 5).and_hms(0, 45, 0),
        action: Action::FallAsleep
      },
      Record { 
        datetime: Utc.ymd(1518, 11, 5).and_hms(0, 55, 0),
        action: Action::WakeUp
      },
    ];

    assert_eq!(filter_map_asleep(&v), vec![
      AsleepRecord { 
        id: 10, 
        datetime: Utc.ymd(1518, 11, 1).and_hms(0, 5, 0),
        duration: Duration::minutes(20)
      },
      AsleepRecord { 
        id: 10, 
        datetime: Utc.ymd(1518, 11, 1).and_hms(0, 30, 0),
        duration: Duration::minutes(25)
      },
      AsleepRecord { 
        id: 99, 
        datetime: Utc.ymd(1518, 11, 2).and_hms(0, 40, 0),
        duration: Duration::minutes(10)
      },
      AsleepRecord { 
        id: 10, 
        datetime: Utc.ymd(1518, 11, 3).and_hms(0, 24, 0),
        duration: Duration::minutes(5)
      },
      AsleepRecord { 
        id: 99, 
        datetime: Utc.ymd(1518, 11, 4).and_hms(0, 36, 0),
        duration: Duration::minutes(10)
      },
      AsleepRecord {
        id: 99,
        datetime: Utc.ymd(1518, 11, 5).and_hms(0, 45, 0),
        duration: Duration::minutes(10)
      },
    ]);
  }

  #[test]
  fn find_sleepiest_test() {
    assert_eq!(Some(10), find_sleepiest(&vec![
      AsleepRecord { 
        id: 10, 
        datetime: Utc.ymd(1518, 11, 1).and_hms(0, 5, 0),
        duration: Duration::minutes(20)
      },
      AsleepRecord { 
        id: 10, 
        datetime: Utc.ymd(1518, 11, 1).and_hms(0, 30, 0),
        duration: Duration::minutes(25)
      },
      AsleepRecord { 
        id: 99, 
        datetime: Utc.ymd(1518, 11, 2).and_hms(0, 40, 0),
        duration: Duration::minutes(10)
      },
      AsleepRecord { 
        id: 10, 
        datetime: Utc.ymd(1518, 11, 3).and_hms(0, 24, 0),
        duration: Duration::minutes(5)
      },
      AsleepRecord { 
        id: 99, 
        datetime: Utc.ymd(1518, 11, 4).and_hms(0, 36, 0),
        duration: Duration::minutes(10)
      },
      AsleepRecord {
        id: 99,
        datetime: Utc.ymd(1518, 11, 5).and_hms(0, 45, 0),
        duration: Duration::minutes(10)
      },
    ]));
  }

  #[test]
  fn find_intersection_beginning_test() {
    let a = AsleepRecord { 
      id: 10, 
      datetime: Utc.ymd(1518, 11, 1).and_hms(0, 5, 0),
      duration: Duration::minutes(20)
    };

    let b = AsleepRecord { 
      id: 10, 
      datetime: Utc.ymd(1518, 11, 3).and_hms(0, 24, 0),
      duration: Duration::minutes(5)
    };

    let result = find_intersection_beginning(&a, &b);

    assert!(result.is_some());
    assert_eq!(result.unwrap().time(), NaiveTime::from_hms(0, 24, 0));
  }

  #[test]
  fn calc_part1_test() {
    assert_eq!(240, calc_part1(&vec![
      Record { 
        datetime: Utc.ymd(1518, 11, 1).and_hms(0, 0, 0),
        action: Action::Shift(10)
      },
      Record { 
        datetime: Utc.ymd(1518, 11, 1).and_hms(0, 5, 0),
        action: Action::FallAsleep
      },
      Record { 
        datetime: Utc.ymd(1518, 11, 1).and_hms(0, 25, 0),
        action: Action::WakeUp
      },
      Record { 
        datetime: Utc.ymd(1518, 11, 1).and_hms(0, 30, 0),
        action: Action::FallAsleep
      },
      Record { 
        datetime: Utc.ymd(1518, 11, 1).and_hms(0, 55, 0),
        action: Action::WakeUp
      },
      Record { 
        datetime: Utc.ymd(1518, 11, 1).and_hms(23, 58, 0),
        action: Action::Shift(99)
      },
      Record { 
        datetime: Utc.ymd(1518, 11, 2).and_hms(0, 40, 0),
        action: Action::FallAsleep
      },
      Record { 
        datetime: Utc.ymd(1518, 11, 2).and_hms(0, 50, 0),
        action: Action::WakeUp
      },
      Record { 
        datetime: Utc.ymd(1518, 11, 3).and_hms(0, 5, 0),
        action: Action::Shift(10)
      },
      Record { 
        datetime: Utc.ymd(1518, 11, 3).and_hms(0, 24, 0),
        action: Action::FallAsleep
      },
      Record { 
        datetime: Utc.ymd(1518, 11, 3).and_hms(0, 29, 0),
        action: Action::WakeUp
      },
      Record { 
        datetime: Utc.ymd(1518, 11, 4).and_hms(0, 2, 0),
        action: Action::Shift(99)
      },
      Record { 
        datetime: Utc.ymd(1518, 11, 4).and_hms(0, 36, 0),
        action: Action::FallAsleep
      },
      Record { 
        datetime: Utc.ymd(1518, 11, 4).and_hms(0, 46, 0),
        action: Action::WakeUp
      },
      Record { 
        datetime: Utc.ymd(1518, 11, 5).and_hms(0, 3, 0),
        action: Action::Shift(99)
      },
      Record { 
        datetime: Utc.ymd(1518, 11, 5).and_hms(0, 45, 0),
        action: Action::FallAsleep
      },
      Record { 
        datetime: Utc.ymd(1518, 11, 5).and_hms(0, 55, 0),
        action: Action::WakeUp
      },
    ]));
  }
}
