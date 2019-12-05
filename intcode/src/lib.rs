use std::convert::TryFrom;

pub type IP = usize;
pub type IPOffset = isize;
pub type Word = i64;

#[derive(Debug)]
pub struct Program {
  memory: Vec<Word>,
  input: Word,
}

impl Program {
  pub fn new(capacity: usize, user_input: Word) -> Self {
    let memory = Vec::with_capacity(capacity);
    let input = user_input;

    Program { memory, input }
  }

  pub fn from_str<S>(input: S, user_input: Word) -> Result<Self, String>
  where
    S: AsRef<str>,
  {
    let input = input.as_ref();

    let memory = input
      .split(',')
      .map(|i| i.parse().map_err(|e| format!("cannot parse: {}", e)))
      .collect::<Result<_, _>>()?;

    Ok(Program {
      memory,
      input: user_input,
    })
  }

  pub fn mem_size(&self) -> usize {
    self.memory.len()
  }

  pub fn mimick(&mut self, other: &Self) {
    self.memory.clear();
    self.memory.extend_from_slice(&other.memory);
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

  fn perform_op<F>(
    &mut self,
    ip: IP,
    mode_1: ParamMode,
    mode_2: ParamMode,
    f: F,
  ) -> Result<IPControl, String>
  where
    F: FnOnce(Word, Word) -> Word,
  {
    let memory = &mut self.memory;

    if ip + 3 >= memory.len() {
      return Err("operator is missing data".to_owned());
    }

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

  fn perform_get_input(&mut self, ip: IP) -> Result<IPControl, String> {
    let memory = &mut self.memory;

    if ip + 1 >= memory.len() {
      return Err("operator is missing data".to_owned());
    }

    let addr = memory[ip + 1] as usize;

    if addr >= memory.len() {
      return Err(format!("cannot store input at {}: out of bounds", addr));
    }

    memory[addr] = self.input;

    Ok(IPControl::Increase(2))
  }

  fn perform_output(&mut self, ip: IP) -> Result<IPControl, String> {
    let memory = &mut self.memory;

    if ip + 1 >= memory.len() {
      return Err("operator is missing data".to_owned());
    }

    let addr = memory[ip + 1] as usize;

    if addr >= memory.len() {
      return Err(format!("cannot store input at {}: out of bounds", addr));
    }

    println!("{}", memory[addr]);

    Ok(IPControl::Increase(2))
  }

  fn perform_jump(
    &mut self,
    ip: IP,
    mode_1: ParamMode,
    mode_2: ParamMode,
    truth: bool,
  ) -> Result<IPControl, String> {
    let memory = &mut self.memory;

    if ip + 2 >= memory.len() {
      return Err("operator is missing data".to_owned());
    }

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
    ip: IP,
    mode_1: ParamMode,
    mode_2: ParamMode,
    pred: F,
  ) -> Result<IPControl, String>
  where
    F: FnOnce(Word, Word) -> bool,
  {
    let memory = &mut self.memory;

    if ip + 3 >= memory.len() {
      return Err("operator is missing data".to_owned());
    }

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

  pub fn run(&mut self) -> Result<(), String> {
    if self.memory.is_empty() {
      return Err("no more data".to_owned());
    }

    let mut ip = 0; // instruction pointer

    loop {
      let opcode = extract_op_code(self.memory[ip])?;

      let ip_ctrl = match opcode {
        OpCode::Add(mode_1, mode_2) => self.perform_op(ip, mode_1, mode_2, |a, b| a + b)?,
        OpCode::Mult(mode_1, mode_2) => self.perform_op(ip, mode_1, mode_2, |a, b| a * b)?,
        OpCode::GetInput => self.perform_get_input(ip)?,
        OpCode::Output => self.perform_output(ip)?,
        OpCode::JumpIfTrue(mode_1, mode_2) => self.perform_jump(ip, mode_1, mode_2, true)?,
        OpCode::JumpIfFalse(mode_1, mode_2) => self.perform_jump(ip, mode_1, mode_2, false)?,
        OpCode::IfLT(mode_1, mode_2) => {
          self.perform_conditional(ip, mode_1, mode_2, |a, b| a < b)?
        }
        OpCode::IfEQ(mode_1, mode_2) => {
          self.perform_conditional(ip, mode_1, mode_2, |a, b| a == b)?
        }
        OpCode::Halt => break,
      };

      match ip_ctrl {
        IPControl::Increase(off) => ip = (ip as isize + off) as usize,
        IPControl::Manual(new_ip) => ip = new_ip,
      }
    }

    Ok(())
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
}

impl TryFrom<Word> for ParamMode {
  type Error = String;

  fn try_from(w: Word) -> Result<Self, Self::Error> {
    match w {
      0 => Ok(ParamMode::Position),
      1 => Ok(ParamMode::Immediate),
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
