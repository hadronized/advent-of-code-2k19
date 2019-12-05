use intcode::Program;

const INPUT: &str = include_str!("../input.txt");

fn main() {
  let mut program = Program::from_str(INPUT.trim(), 0).unwrap();
  program.write(1, 12).unwrap();
  program.write(2, 2).unwrap();
  program.run().unwrap();

  println!("1st answer: {:?}", program.read(0));

  let initial_program = Program::from_str(INPUT.trim(), 0).unwrap();
  let mut program = Program::new(initial_program.mem_size(), 0);

  'outer: for noun in 0..=99 {
    for verb in 0..=99 {
      program.mimick(&initial_program);
      program.write(1, noun).unwrap();
      program.write(2, verb).unwrap();

      program.run().unwrap();

      if program.read(0).unwrap() == 19690720 {
        println!(
          "2nd answer: {}{}",
          program.read(1).unwrap(),
          program.read(2).unwrap()
        );
        break 'outer;
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn simple_program() {
    const INPUT_TEST: &str = "1,9,10,3,2,3,11,0,99,30,40,50";

    let mut program = Program::from_str(&INPUT_TEST[0..INPUT_TEST.len()], 0).unwrap();
    program.run().unwrap();

    assert_eq!(program.read(0).unwrap(), 3500);

    const INPUT_TEST_2: &str = "1,1,1,4,99,5,6,0,99";
    program = Program::from_str(&INPUT_TEST_2[0..INPUT_TEST_2.len()], 0).unwrap();
    program.run().unwrap();

    assert_eq!(program.read(0).unwrap(), 30);
  }
}
