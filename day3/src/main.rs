use day3::read_input;
use day3::calc_part1;
use day3::calc_part2;

fn main() {
  let filepath = "./input.txt";
  let claims = read_input(&filepath).unwrap();

  println!("Day #3, part #1 {}", calc_part1(&claims));
  println!("Day #3, part #2 {}", calc_part2(&claims));
}