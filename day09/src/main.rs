use intcode::Program;

const INPUT: &str = include_str!("../input.txt");

fn main() {
  let mut program = Program::from_str(INPUT.trim()).unwrap();
  program.run(&[1]).unwrap();

  let mut program = Program::from_str(INPUT.trim()).unwrap();
  program.run(&[2]).unwrap();
}
