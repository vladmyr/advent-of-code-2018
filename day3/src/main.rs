use day3::read_input;
use day3::calc_part1;

fn main() {
  let filepath = "./input.txt";
  let claims = read_input(&filepath).unwrap();

  println!("Day #3, part #1 {}", calc_part1(&claims));
}