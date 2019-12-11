use std::convert::TryFrom;

const DEFAULT_MEMORY_SIZE: usize = 10000;

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

    let mut memory: Vec<_> = input
      .split(',')
      .map(|i| i.parse().map_err(|e| format!("cannot parse: {}", e)))
      .collect::<Result<_, _>>()?;
    let ip = 0;
    let rel_base = 0;

    // resize memory to make it bigger
    memory.resize(DEFAULT_MEMORY_SIZE, 0);

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

  /// Read an instruction operand based on the mode of the instruction.
  fn read_operand(&self, offset: IPOffset, mode: ParamMode) -> Result<Word, String> {
    let op = match mode {
      ParamMode::Position => *self
        .memory
        .get(self.memory[self.ip + offset as usize] as usize)
        .ok_or(format!(
          "cannot read operand in position mode: {} + {}",
          self.ip, offset
        ))?,

      ParamMode::Immediate => self.memory[self.ip + offset as usize],

      ParamMode::Relative => *self
        .memory
        .get((self.rel_base + self.memory[self.ip + offset as usize] as isize) as usize)
        .ok_or(format!(
          "cannot read operand in relative mode: {} + {}, {}",
          self.ip, offset, self.rel_base
        ))?,
    };

    Ok(op)
  }

  /// Read an address operand based on the mode of the instruction.
  fn read_addr_operand(&self, offset: IPOffset, mode: ParamMode) -> Result<Word, String> {
    match mode {
      ParamMode::Position => Ok(self.memory[self.ip + offset as usize]),

      ParamMode::Immediate => Err("not supported in immediate mode".to_owned()),

      ParamMode::Relative => Ok(self.rel_base as Word + self.memory[self.ip + offset as usize]),
    }
  }

  fn perform_op<F>(
    &mut self,
    mode_1: ParamMode,
    mode_2: ParamMode,
    mode_3: ParamMode,
    f: F,
  ) -> Result<IPControl, String>
  where
    F: FnOnce(Word, Word) -> Word,
  {
    self.guard_memory_ip(3)?;

    let op1 = self.read_operand(1, mode_1)?;
    let op2 = self.read_operand(2, mode_2)?;
    let output_idx = self.read_addr_operand(3, mode_3)? as usize;

    let output = f(op1, op2);

    self.write(output_idx, output)?;

    Ok(IPControl::Increase(4))
  }

  fn perform_get_input(&mut self, inputs: &[Word], mode: ParamMode) -> Result<IPControl, String> {
    self.guard_memory_ip(1)?;

    if inputs.is_empty() {
      return Err("no input".to_owned());
    }

    let addr = self.read_addr_operand(1, mode)? as usize;

    if addr >= self.memory.len() {
      return Err(format!("cannot store input at {}: out of bounds", addr));
    }

    self.write(addr, inputs[0])?;

    Ok(IPControl::Increase(2))
  }

  fn perform_output(&mut self, output: &mut Word, mode: ParamMode) -> Result<IPControl, String> {
    self.guard_memory_ip(1)?;

    let value = self.read_operand(1, mode)?;

    *output = value;

    Ok(IPControl::Increase(2))
  }

  fn perform_jump(
    &mut self,
    mode_1: ParamMode,
    mode_2: ParamMode,
    truth: bool,
  ) -> Result<IPControl, String> {
    self.guard_memory_ip(2)?;

    let c = self.read_operand(1, mode_1)? != 0;

    if c == truth {
      let new_ip = self.read_operand(2, mode_2)? as IP;
      Ok(IPControl::Manual(new_ip))
    } else {
      Ok(IPControl::Increase(3))
    }
  }

  fn perform_conditional<F>(
    &mut self,
    mode_1: ParamMode,
    mode_2: ParamMode,
    mode_3: ParamMode,
    pred: F,
  ) -> Result<IPControl, String>
  where
    F: FnOnce(Word, Word) -> bool,
  {
    self.guard_memory_ip(3)?;

    let op1 = self.read_operand(1, mode_1)?;
    let op2 = self.read_operand(2, mode_2)?;
    let output_idx = self.read_addr_operand(3, mode_3)? as usize;

    self.write(output_idx, pred(op1, op2) as Word)?;

    Ok(IPControl::Increase(4))
  }

  fn perform_adjust_rel_base(&mut self, mode: ParamMode) -> Result<IPControl, String> {
    self.guard_memory_ip(1)?;

    let new_base_offset = self.read_operand(1, mode)?;

    self.rel_base += new_base_offset as isize;

    Ok(IPControl::Increase(2))
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
        OpCode::Add(mode_1, mode_2, mode_3) => {
          self.perform_op(mode_1, mode_2, mode_3, |a, b| a + b)?
        }

        OpCode::Mult(mode_1, mode_2, mode_3) => {
          self.perform_op(mode_1, mode_2, mode_3, |a, b| a * b)?
        }

        OpCode::GetInput(mode) => {
          let ip_ctrl = self.perform_get_input(&inputs, mode)?;
          inputs.swap_remove(0);
          ip_ctrl
        }

        OpCode::Output(mode) => {
          let mut out = 0;
          let ip_ctrl = self.perform_output(&mut out, mode)?;
          output = Some(out);

          self.update_ip(ip_ctrl);

          return Ok(Suspended::Running { inputs, output });
        }

        OpCode::JumpIfTrue(mode_1, mode_2) => self.perform_jump(mode_1, mode_2, true)?,

        OpCode::JumpIfFalse(mode_1, mode_2) => self.perform_jump(mode_1, mode_2, false)?,

        OpCode::IfLT(mode_1, mode_2, mode_3) => {
          self.perform_conditional(mode_1, mode_2, mode_3, |a, b| a < b)?
        }

        OpCode::IfEQ(mode_1, mode_2, mode_3) => {
          self.perform_conditional(mode_1, mode_2, mode_3, |a, b| a == b)?
        }

        OpCode::AdjustRelBase(mode) => self.perform_adjust_rel_base(mode)?,

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
  Add(ParamMode, ParamMode, ParamMode),
  Mult(ParamMode, ParamMode, ParamMode),
  GetInput(ParamMode),
  Output(ParamMode),
  JumpIfTrue(ParamMode, ParamMode),
  JumpIfFalse(ParamMode, ParamMode),
  IfLT(ParamMode, ParamMode, ParamMode),
  IfEQ(ParamMode, ParamMode, ParamMode),
  AdjustRelBase(ParamMode),
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
      let mode_1 = ParamMode::try_from((w / 100) % 10)?;
      let mode_2 = ParamMode::try_from((w / 1000) % 10)?;
      let mode_3 = ParamMode::try_from((w / 10000) % 10)?;
      Ok(OpCode::Add(mode_1, mode_2, mode_3))
    }

    // multiplication
    2 => {
      let mode_1 = ParamMode::try_from((w / 100) % 10)?;
      let mode_2 = ParamMode::try_from((w / 1000) % 10)?;
      let mode_3 = ParamMode::try_from((w / 10000) % 10)?;
      Ok(OpCode::Mult(mode_1, mode_2, mode_3))
    }

    // get input
    3 => {
      let mode = ParamMode::try_from((w / 100) % 10)?;
      Ok(OpCode::GetInput(mode))
    }

    // output
    4 => {
      let mode = ParamMode::try_from((w / 100) % 10)?;
      Ok(OpCode::Output(mode))
    }

    // jump if true
    5 => {
      let mode_1 = ParamMode::try_from((w / 100) % 10)?;
      let mode_2 = ParamMode::try_from((w / 1000) % 10)?;

      Ok(OpCode::JumpIfTrue(mode_1, mode_2))
    }

    // jump if false
    6 => {
      let mode_1 = ParamMode::try_from((w / 100) % 10)?;
      let mode_2 = ParamMode::try_from((w / 1000) % 10)?;

      Ok(OpCode::JumpIfFalse(mode_1, mode_2))
    }

    // if less than
    7 => {
      let mode_1 = ParamMode::try_from((w / 100) % 10)?;
      let mode_2 = ParamMode::try_from((w / 1000) % 10)?;
      let mode_3 = ParamMode::try_from((w / 10000) % 10)?;

      Ok(OpCode::IfLT(mode_1, mode_2, mode_3))
    }

    // if equals
    8 => {
      let mode_1 = ParamMode::try_from((w / 100) % 10)?;
      let mode_2 = ParamMode::try_from((w / 1000) % 10)?;
      let mode_3 = ParamMode::try_from((w / 10000) % 10)?;

      Ok(OpCode::IfEQ(mode_1, mode_2, mode_3))
    }

    // adjust relative base
    9 => {
      let mode = ParamMode::try_from((w / 100) % 10)?;
      Ok(OpCode::AdjustRelBase(mode))
    }

    // halt
    99 => Ok(OpCode::Halt),

    x => Err(format!("unknown opcode: {} ({})", x, w)),
  }
}
