use std::num::ParseIntError;

struct Claim {
  startX: usize,
  startY: usize,
  endX: usize,
  endY: usize,
}

// fn parse(input: &String) -> Result<Claim, String> {
fn parse(input: &String) -> Result<Vec<usize>, ParseIntError> {
  // let [startX, startY, width, height] = input
  input
    .split("@ ")
    .skip(1)
    .flat_map(|x| x.split(": "))
    .flat_map(|x| x.split(|c| c == ',' || c == 'x'))
    .map(|s| s.parse::<usize>())
    .try_fold(Vec::new(), |mut acc, r| {
      match r.clone().err() {
        Some(err) => Err(err),
        None => {
          acc.push(r.unwrap());
          Ok(acc)
        }
      }
    })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_test() {
    // assert_eq!(parse(&String::from("#1 @ 1,1: 1x1")).is_ok(), true);

    // assert_eq!(parse(&String::from("#1 @ 1,1: 1x")).is_ok(), false);
    // assert_eq!(parse(&String::from("#1 @ 1,1: x1")).is_ok(), false);
    // assert_eq!(parse(&String::from("#1 @ 1,: 1x1")).is_ok(), false);
    // assert_eq!(parse(&String::from("#1 @ ,1: 1x1")).is_ok(), false);
    // assert_eq!(parse(&String::from("#1 @ ,: 1x1")).is_ok(), false);
    // assert_eq!(parse(&String::from("#1 @ ,: x")).is_ok(), false);
    // assert_eq!(parse(&String::from("#1 @ a,b: -1x-2")).is_ok(), false);

    assert_eq!(parse(&String::from("")).is_ok(), false);
    // assert_eq!(parse(&String::from("#1 1,1: 1x1")).is_ok(), false);
    // assert_eq!(parse(&String::from("#1 @ 1,1 1x1")).is_ok(), false);
    // assert_eq!(parse(&String::from("#1 @ 1,1 @ 1x1")).is_ok(), false);
    // assert_eq!(parse(&String::from("#1 : 1,1 : 1x1")).is_ok(), false);
    // assert_eq!(parse(&String::from("#1 , 1,1 , 1x1")).is_ok(), false);
    // assert_eq!(parse(&String::from("#1 , 1, , x1")).is_ok(), false);
  }
}
