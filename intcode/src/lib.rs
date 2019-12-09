use std::convert::TryFrom;

pub type IP = usize;
pub type IPOffset = isize;
pub type Word = i64;

#[derive(Debug)]
pub struct Program {
  memory: Vec<Word>,
  ip: IP,
  rel_base: IPOffset,
}

impl Program {
  pub fn new(capacity: usize) -> Self {
    let memory = Vec::with_capacity(capacity);
    let ip = 0;
    let rel_base = 0;

    Program {
      memory,
      ip,
      rel_base,
    }
  }

  pub fn from_str<S>(input: S) -> Result<Self, String>
  where
    S: AsRef<str>,
  {
    let input = input.as_ref();

    let memory = input
      .split(',')
      .map(|i| i.parse().map_err(|e| format!("cannot parse: {}", e)))
      .collect::<Result<_, _>>()?;
    let ip = 0;
    let rel_base = 0;

    Ok(Program {
      memory,
      ip,
      rel_base,
    })
  }

  pub fn mem_size(&self) -> usize {
    self.memory.len()
  }

  pub fn mimick(&mut self, other: &Self) {
    self.memory.clear();
    self.memory.extend_from_slice(&other.memory);
    self.ip = 0;
    self.rel_base = 0;
  }

  pub fn is_halted(&self) -> bool {
    self.ip >= self.memory.len()
  }

  pub fn read(&self, i: usize) -> Result<Word, String> {
    if i >= self.memory.len() {
      return Err(format!(
        "read: index out of bounds: {} ({})",
        i,
        self.memory.len()
      ));
    }

    Ok(self.memory[i])
  }

  pub fn write(&mut self, i: usize, w: Word) -> Result<(), String> {
    if i >= self.memory.len() {
      return Err(format!(
        "write: index out of bounds: {} ({})",
        i,
        self.memory.len()
      ));
    }

    self.memory[i] = w;

    Ok(())
  }

  /// Ensure the IP will not overflow memory.
  fn guard_memory_ip(&self, offset: IPOffset) -> Result<(), String> {
    let len = self.memory.len();
    let ip = self.ip;

    if ip + offset as usize >= len {
      Err(format!(
        "cannot execute instruction: IP={}, offset={}, memory={}",
        ip, offset, len
      ))
    } else {
      Ok(())
    }
  }

  fn perform_op<F>(
    &mut self,
    mode_1: ParamMode,
    mode_2: ParamMode,
    f: F,
  ) -> Result<IPControl, String>
  where
    F: FnOnce(Word, Word) -> Word,
  {
    let ip = self.ip;

    self.guard_memory_ip(3)?;

    let memory = &mut self.memory;

    let op1 = if let ParamMode::Position = mode_1 {
      *memory
        .get(memory[ip + 1] as usize)
        .ok_or(format!("read 1 out of bounds: {}", ip + 1,))?
    } else {
      memory[ip + 1]
    };

    let op2 = if let ParamMode::Position = mode_2 {
      *memory
        .get(memory[ip + 2] as usize)
        .ok_or(format!("read 2 out of bounds: {}", ip + 2))?
    } else {
      memory[ip + 2]
    };

    let output_idx = memory[ip + 3] as usize;

    let output = f(op1, op2);

    *memory
      .get_mut(output_idx)
      .ok_or(format!("write out of bounds: {}", output_idx))? = output;

    Ok(IPControl::Increase(4))
  }

  fn perform_get_input(&mut self, inputs: &[Word]) -> Result<IPControl, String> {
    let ip = self.ip;

    self.guard_memory_ip(1)?;

    let memory = &mut self.memory;

    if inputs.is_empty() {
      return Err("no input".to_owned());
    }

    let addr = memory[ip + 1] as usize;

    if addr >= memory.len() {
      return Err(format!("cannot store input at {}: out of bounds", addr));
    }

    memory[addr] = inputs[0];

    Ok(IPControl::Increase(2))
  }

  fn perform_output(&mut self, output: &mut Word) -> Result<IPControl, String> {
    let ip = self.ip;

    self.guard_memory_ip(1)?;

    let memory = &mut self.memory;

    let addr = memory[ip + 1] as usize;

    if addr >= memory.len() {
      return Err(format!("cannot store input at {}: out of bounds", addr));
    }

    *output = memory[addr];

    Ok(IPControl::Increase(2))
  }

  fn perform_jump(
    &mut self,
    mode_1: ParamMode,
    mode_2: ParamMode,
    truth: bool,
  ) -> Result<IPControl, String> {
    let ip = self.ip;

    self.guard_memory_ip(2)?;

    let memory = &mut self.memory;

    let c = if let ParamMode::Position = mode_1 {
      *memory
        .get(memory[ip + 1] as usize)
        .ok_or(format!("read 1 out of bound: {}", ip + 1))?
        != 0
    } else {
      memory[ip + 1] != 0
    };

    if c == truth {
      let new_ip = if let ParamMode::Position = mode_2 {
        *memory
          .get(memory[ip + 2] as usize)
          .ok_or(format!("read 2 out of bounds: {}", ip + 2))? as IP
      } else {
        memory[ip + 2] as IP
      };

      Ok(IPControl::Manual(new_ip))
    } else {
      Ok(IPControl::Increase(3))
    }
  }

  fn perform_conditional<F>(
    &mut self,
    mode_1: ParamMode,
    mode_2: ParamMode,
    pred: F,
  ) -> Result<IPControl, String>
  where
    F: FnOnce(Word, Word) -> bool,
  {
    let ip = self.ip;

    self.guard_memory_ip(3)?;

    let memory = &mut self.memory;

    let op1 = if let ParamMode::Position = mode_1 {
      *memory
        .get(memory[ip + 1] as usize)
        .ok_or(format!("read 1 out of bounds: {}", ip + 1,))?
    } else {
      memory[ip + 1]
    };

    let op2 = if let ParamMode::Position = mode_2 {
      *memory
        .get(memory[ip + 2] as usize)
        .ok_or(format!("read 2 out of bounds: {}", ip + 2))?
    } else {
      memory[ip + 2]
    };

    let output_idx = memory[ip + 3] as usize;

    *memory
      .get_mut(output_idx)
      .ok_or(format!("write out of bounds: {}", ip + 3))? = pred(op1, op2) as Word;

    Ok(IPControl::Increase(4))
  }


  }

  /// Run until the program halts.
  pub fn run(&mut self, inputs: &[Word]) -> Result<Option<Word>, String> {
    let inputs = inputs.to_owned();

    let mut suspended = self.rerun_suspended(inputs, None)?;

    loop {
      match suspended {
        Suspended::Running { inputs, output } => {
          suspended = self.rerun_suspended(inputs, output)?;
        }

        Suspended::Halted { output } => {
          break Ok(output);
        }
      }
    }
  }

  /// Run and suspend.
  pub fn run_suspended(&mut self, inputs: &[Word]) -> Result<Suspended, String> {
    let inputs = inputs.to_owned();
    self.rerun_suspended(inputs, None)
  }

  /// Run until the program emits an output, suspending its state. Use the output variable to either
  /// kill the program, or continue.
  fn rerun_suspended(
    &mut self,
    mut inputs: Vec<Word>,
    mut output: Option<Word>,
  ) -> Result<Suspended, String> {
    if self.memory.is_empty() {
      return Err("no more data".to_owned());
    }

    loop {
      let opcode = extract_op_code(self.memory[self.ip])?;

      let ip_ctrl = match opcode {
        OpCode::Add(mode_1, mode_2) => self.perform_op(mode_1, mode_2, |a, b| a + b)?,

        OpCode::Mult(mode_1, mode_2) => self.perform_op(mode_1, mode_2, |a, b| a * b)?,

        OpCode::GetInput => {
          let ip_ctrl = self.perform_get_input(&inputs)?;
          inputs.swap_remove(0);
          ip_ctrl
        }

        OpCode::Output => {
          let mut out = 0;
          let ip_ctrl = self.perform_output(&mut out)?;
          output = Some(out);

          self.update_ip(ip_ctrl);

          return Ok(Suspended::Running { inputs, output });
        }

        OpCode::JumpIfTrue(mode_1, mode_2) => self.perform_jump(mode_1, mode_2, true)?,

        OpCode::JumpIfFalse(mode_1, mode_2) => self.perform_jump(mode_1, mode_2, false)?,

        OpCode::IfLT(mode_1, mode_2) => self.perform_conditional(mode_1, mode_2, |a, b| a < b)?,

        OpCode::IfEQ(mode_1, mode_2) => self.perform_conditional(mode_1, mode_2, |a, b| a == b)?,

        OpCode::Halt => break,
      };

      self.update_ip(ip_ctrl);
    }

    Ok(Suspended::Halted { output })
  }

  pub fn rerun(&mut self, suspended: Suspended) -> Result<Suspended, String> {
    match suspended {
      Suspended::Running { inputs, output } => self.rerun_suspended(inputs, output),

      _ => Ok(suspended),
    }
  }

  fn update_ip(&mut self, ip_ctrl: IPControl) {
    self.ip = match ip_ctrl {
      IPControl::Increase(off) => (self.ip as isize + off) as usize,
      IPControl::Manual(new_ip) => new_ip,
    };
  }
}

/// A suspended program.
///
/// A suspended program can be re-run or killed.
#[derive(Clone, Debug)]
pub enum Suspended {
  Running {
    inputs: Vec<Word>,
    output: Option<Word>,
  },

  Halted {
    output: Option<Word>,
  },
}

impl Suspended {
  pub fn output(&self) -> Option<Word> {
    match *self {
      Suspended::Running { output, .. } => output,
      Suspended::Halted { output } => output,
    }
  }
}

/// Instruction pointer control.
///
/// `IPControl::Increase` is just the normal flow (the IP increases after each instruction).
///
/// `IPControl::Manual` sets the IP manually.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum IPControl {
  Increase(IPOffset),
  Manual(IP),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum OpCode {
  Add(ParamMode, ParamMode),  // last is immediate
  Mult(ParamMode, ParamMode), // last is immediate
  GetInput,                   // always immediate
  Output,                     // same
  JumpIfTrue(ParamMode, ParamMode),
  JumpIfFalse(ParamMode, ParamMode),
  IfLT(ParamMode, ParamMode),
  IfEQ(ParamMode, ParamMode),
  Halt, // the world makes no sense
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum ParamMode {
  Position,
  Immediate,
  Relative,
}

impl TryFrom<Word> for ParamMode {
  type Error = String;

  fn try_from(w: Word) -> Result<Self, Self::Error> {
    match w {
      0 => Ok(ParamMode::Position),
      1 => Ok(ParamMode::Immediate),
      2 => Ok(ParamMode::Relative),
      _ => Err(format!("unsupported parameter mode: {}", w)),
    }
  }
}

fn extract_op_code(w: Word) -> Result<OpCode, String> {
  match w % 100 {
    // addition
    1 => {
      let mode_1 = ParamMode::try_from((w / 100) & 0x1)?;
      let mode_2 = ParamMode::try_from((w / 1000) & 0x1)?;
      Ok(OpCode::Add(mode_1, mode_2))
    }

    // multiplication
    2 => {
      let mode_1 = ParamMode::try_from((w / 100) & 0x1)?;
      let mode_2 = ParamMode::try_from((w / 1000) & 0x1)?;
      Ok(OpCode::Mult(mode_1, mode_2))
    }

    // get input
    3 => Ok(OpCode::GetInput),

    // output
    4 => Ok(OpCode::Output),

    // jump if true
    5 => {
      let mode_1 = ParamMode::try_from((w / 100) & 0x1)?;
      let mode_2 = ParamMode::try_from((w / 1000) & 0x1)?;
      Ok(OpCode::JumpIfTrue(mode_1, mode_2))
    }

    // jump if false
    6 => {
      let mode_1 = ParamMode::try_from((w / 100) & 0x1)?;
      let mode_2 = ParamMode::try_from((w / 1000) & 0x1)?;
      Ok(OpCode::JumpIfFalse(mode_1, mode_2))
    }

    // if less than
    7 => {
      let mode_1 = ParamMode::try_from((w / 100) & 0x1)?;
      let mode_2 = ParamMode::try_from((w / 1000) & 0x1)?;
      Ok(OpCode::IfLT(mode_1, mode_2))
    }

    // if equals
    8 => {
      let mode_1 = ParamMode::try_from((w / 100) & 0x1)?;
      let mode_2 = ParamMode::try_from((w / 1000) & 0x1)?;
      Ok(OpCode::IfEQ(mode_1, mode_2))
    }

    // halt
    99 => Ok(OpCode::Halt),

    x => Err(format!("unknown opcode: {} ({})", x, w)),
  }
}
