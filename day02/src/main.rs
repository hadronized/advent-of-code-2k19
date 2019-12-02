const INPUT: &str = include_str!("../input.txt");

fn get_program() -> Result<Vec<u32>, String> {
  // remove the \n
  INPUT[0..INPUT.len() - 1]
    .split(',')
    .map(|i| i.parse().map_err(|e| format!("cannot parse: {}", e)))
    .collect()
}

fn perform_op<F>(program: &mut [u32], ip: usize, f: F) -> Result<(), String>
where
  F: FnOnce(u32, u32) -> u32,
{
  if ip + 3 >= program.len() {
    return Err("operator is missing data".to_owned());
  }

  let op1 = *program
    .get(program[ip + 1] as usize)
    .ok_or("read 1 out of bound".to_owned())?;
  let op2 = *program
    .get(program[ip + 2] as usize)
    .ok_or("read 2 out of bounds".to_owned())?;
  let output_idx = program[ip + 3];

  let output = f(op1, op2);

  *program
    .get_mut(output_idx as usize)
    .ok_or(format!("write out of bounds: {}", output_idx))? = output;

  Ok(())
}

fn run_program(program: &mut [u32]) -> Result<(), String> {
  if program.is_empty() {
    return Err("no more data".to_owned());
  }

  let mut ip = 0; // instruction pointer
  loop {
    let opcode = program[ip];

    match opcode {
      // addition
      1 => perform_op(program, ip, |a, b| a + b)?,
      2 => perform_op(program, ip, |a, b| a * b)?,
      99 => break,
      _ => return Err("unknown operator".to_owned()),
    }

    ip += 4;
  }

  Ok(())
}

fn main() {
  let mut program = get_program().unwrap();
  program[1] = 12;
  program[2] = 2;
  run_program(&mut program).unwrap();

  println!("1st answer: {}", program[0]);

  'outer: for noun in 0..99 {
    for verb in 0..99 {
      let mut program = get_program().unwrap();
      program[1] = noun;
      program[2] = verb;
      run_program(&mut program).unwrap();

      if program[0] == 19690720 {
        println!("2nd answer: {}{}", program[1], program[2]);
        break 'outer;
      }
    }
  }
}
