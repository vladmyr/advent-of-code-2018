use day5::read_input;
use day5::calc_part1;

fn main() {
  let filepath = "./input.txt";
  let input = read_input(&filepath).unwrap();

  println!("Day #5, part #1 {:?}", calc_part1(&input));
}