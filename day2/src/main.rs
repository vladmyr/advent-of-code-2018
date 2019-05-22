use day2::read_input;
use day2::calc_part1;
use day2::calc_part2;

fn main() {
  let filepath = "./input.txt";
  let input = read_input(&filepath).unwrap();

  println!("Day #2, part #1 {:?}", calc_part1(&input));
  println!("Day #2, part #2 {:?}", calc_part2(&input));
}