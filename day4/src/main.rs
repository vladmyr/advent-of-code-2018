use day4::read_input;
use day4::calc_part1;

fn main() {
  let filepath = "./input.txt";
  let records = read_input(&filepath).unwrap();

  println!("Day #4, part #1 {:?}", calc_part1(&records));
}