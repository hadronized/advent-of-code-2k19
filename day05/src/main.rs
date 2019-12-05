use intcode::{Program, Word};

const INPUT: &str = include_str!("../input.txt");

#[cfg(feature = "part-1")]
const USER_INPUT: Word = 1;

#[cfg(feature = "part-2")]
const USER_INPUT: Word = 5;

fn main() {
  let mut program = Program::from_str(&INPUT[0..INPUT.len() - 1], USER_INPUT).unwrap();

  println!("-----");
  program.run().unwrap();
  println!("-----");
}
