use intcode::Program;

const INPUT: &str = include_str!("../input.txt");

fn main() {
  let mut program = Program::from_str(&INPUT[0..INPUT.len() - 1]).unwrap();

  println!("-----");
  program.run(&[1]).unwrap();
  println!("-----");
  program = Program::from_str(&INPUT[0..INPUT.len() - 1]).unwrap();
  program.run(&[5]).unwrap();
  println!("-----");
}
