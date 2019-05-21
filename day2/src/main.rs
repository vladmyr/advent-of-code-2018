use day2::read_input;
use day2::calc_part1;

fn main() {
  let filepath = "./input.txt";
  let input = read_input(&filepath).unwrap();

  println!("Day #2, part #1 {:?}", calc_part1(&input));
}