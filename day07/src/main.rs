use intcode::{Program, Suspended};

const INPUT: &str = include_str!("../input.txt");

fn main() {
  // generate all possible combinations of phases
  let mut phases_combinations = Vec::new();

  for phase1 in 0..=4 {
    for phase2 in 0..=4 {
      if phase2 == phase1 {
        continue;
      }

      for phase3 in 0..=4 {
        if phase3 == phase1 || phase3 == phase2 {
          continue;
        }

        for phase4 in 0..=4 {
          if phase4 == phase1 || phase4 == phase2 || phase4 == phase3 {
            continue;
          }

          for phase5 in 0..=4 {
            if phase5 == phase1 || phase5 == phase2 || phase5 == phase3 || phase5 == phase4 {
              continue;
            }

            let phase = [phase1, phase2, phase3, phase4, phase5];

            phases_combinations.push(phase);
          }
        }
      }
    }
  }

  let original_program = Program::from_str(INPUT.trim()).unwrap();
  let mut program = Program::new(original_program.mem_size());
  let mut thrusters_signal = 0;

  for phases in &phases_combinations {
    let mut signal = 0;

    for amp in 0..=4 {
      program.mimick(&original_program);
      signal = program.run(&[phases[amp], signal]).unwrap().unwrap();
    }

    thrusters_signal = thrusters_signal.max(signal);
  }

  println!("1st answer: {} thrusters signal", thrusters_signal);

  thrusters_signal = 0;

  let mut acses: Vec<_> = (0..=4)
    .map(|_| Program::new(original_program.mem_size()))
    .collect();

  for mut phases in phases_combinations {
    for phase in &mut phases {
      *phase += 5;
    }

    let mut signal = 0;

    // restart the all the ACSes for the next phase setting
    for acs in &mut acses {
      acs.mimick(&original_program);
    }

    let mut suspended: Vec<_> = (0..=4)
      .map(|amp| {
        let suspended = acses[amp].run_suspended(&[phases[amp], signal]).unwrap();
        signal = suspended.output().unwrap();
        suspended
      })
      .collect();

    // sets input / outputs
    for amp in 0..=4 {
      let output = suspended[amp].output().unwrap();

      // provide the ACS output as input for the next ACS
      if let Suspended::Running { ref mut inputs, .. } = suspended[(amp + 1) % 5] {
        inputs.clear();
        inputs.push(output);
      }
    }

    // feedback loop
    loop {
      for amp in 0..=4 {
        // run the ACS
        suspended[amp] = acses[amp].rerun(suspended[amp].clone()).unwrap();
        let output = suspended[amp].output().unwrap();

        // provide the ACS output as input for the next ACS
        if let Suspended::Running { ref mut inputs, .. } = suspended[(amp + 1) % 5] {
          inputs.clear();
          inputs.push(output);
        }
      }

      if let Suspended::Halted {
        output: Some(signal),
      } = suspended[4]
      {
        thrusters_signal = thrusters_signal.max(signal);
        break;
      }
    }
  }

  println!("2nd answer: {} thrusters signal", thrusters_signal);
}
