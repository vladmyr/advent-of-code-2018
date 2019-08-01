use day6::read_input;
use day6::calc_part1;
use day6::calc_part2;

static WATERLINE: u64 = 10000;

fn main() {
  let filepath = "./input.txt";
  let input = read_input(filepath).unwrap();

  println!("Day #6, part #1 {:?}", calc_part1(&input));
  println!("Day #6, part #2 {:?}", calc_part2(&input, &WATERLINE));
}