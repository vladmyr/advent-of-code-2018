use std::fs::File;
use std::io::{BufReader, BufRead};

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

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
