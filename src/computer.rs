pub struct Program {
  code: Vec<i64>,
  ptr: usize,
  relative_base: i64,
}

impl Program {
  pub fn new(code: impl Into<Vec<i64>>) -> Self {
    const MIN_MEMORY: usize = 2048;

    let mut mem: Vec<_> = code.into();
    mem.extend(std::iter::repeat(0).take(MIN_MEMORY.saturating_sub(mem.len())));
    Self {
      code: mem,
      ptr: 0,
      relative_base: 0,
    }
  }

  fn read(&self, offset: usize) -> i64 {
    self.code[self.ptr + offset]
  }

  fn arg_value(&self, mode: &[ParamMode], idx: usize) -> i64 {
    match mode[idx] {
      ParamMode::Position => self.code[self.read(idx + 1) as usize],
      ParamMode::Immediate => self.read(idx + 1),
      ParamMode::Relative => self.code[(self.relative_base + self.read(idx + 1)) as usize],
    }
  }

  fn write(&mut self, mode: &[ParamMode], idx: usize, value: i64) {
    let pos = match mode[idx] {
      ParamMode::Position => self.read(idx + 1),
      ParamMode::Relative => self.relative_base + self.read(idx + 1),
      _ => panic!(),
    };
    self.code[pos as usize] = value;
  }

  pub fn run(&mut self, mut input: impl Iterator<Item = i64>) -> Option<i64> {
    loop {
      match Opcode::from(self.read(0)) {
        Opcode::Add(mode) => {
          let value = self.arg_value(&mode, 0) + self.arg_value(&mode, 1);
          self.write(&mode, 2, value);
          self.ptr += mode.len() + 1;
        }
        Opcode::Mul(mode) => {
          let value = self.arg_value(&mode, 0) * self.arg_value(&mode, 1);
          self.write(&mode, 2, value);
          self.ptr += mode.len() + 1;
        }
        Opcode::Input(mode) => {
          let value = input.next().expect("not enough input");
          self.write(&mode, 0, value);
          self.ptr += mode.len() + 1;
        }
        Opcode::Output(mode) => {
          let res = self.arg_value(&mode, 0);
          self.ptr += mode.len() + 1;
          return Some(res);
        }
        Opcode::JumpIfTrue(mode) => {
          self.ptr = match self.arg_value(&mode, 0) {
            0 => self.ptr + mode.len() + 1,
            _ => self.arg_value(&mode, 1) as usize,
          }
        }
        Opcode::JumpIfFalse(mode) => {
          self.ptr = match self.arg_value(&mode, 0) {
            0 => self.arg_value(&mode, 1) as usize,
            _ => self.ptr + mode.len() + 1,
          }
        }
        Opcode::LessThan(mode) => {
          let value = self.arg_value(&mode, 0) < self.arg_value(&mode, 1);
          self.write(&mode, 2, value as i64);
          self.ptr += mode.len() + 1;
        }
        Opcode::Equals(mode) => {
          let value = self.arg_value(&mode, 0) == self.arg_value(&mode, 1);
          self.write(&mode, 2, value as i64);
          self.ptr += mode.len() + 1;
        }
        Opcode::RelativeBase(mode) => {
          let value = self.arg_value(&mode, 0);
          self.relative_base += value;
          self.ptr += mode.len() + 1;
        }
        Opcode::Exit => return None,
      }
    }
  }

  pub fn code(&self) -> &[i64] {
    &self.code
  }

  pub fn into_code(self) -> Vec<i64> {
    self.code
  }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Opcode {
  Add([ParamMode; 3]),
  Mul([ParamMode; 3]),
  Input([ParamMode; 1]),
  Output([ParamMode; 1]),
  JumpIfTrue([ParamMode; 2]),
  JumpIfFalse([ParamMode; 2]),
  LessThan([ParamMode; 3]),
  Equals([ParamMode; 3]),
  RelativeBase([ParamMode; 1]),
  Exit,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum ParamMode {
  Position,
  Immediate,
  Relative,
}

fn arg_mode_immediate(instruction: i64, arg: u32) -> ParamMode {
  let decimal_pos = 10i64.pow(arg + 2);
  match (instruction / decimal_pos) % 10 {
    0 => ParamMode::Position,
    1 => ParamMode::Immediate,
    2 => ParamMode::Relative,
    i => panic!("unknown param mode {} ({})", i, instruction),
  }
}

fn arg_mode_3(instruction: i64) -> [ParamMode; 3] {
  [
    arg_mode_immediate(instruction, 0),
    arg_mode_immediate(instruction, 1),
    arg_mode_immediate(instruction, 2),
  ]
}

fn arg_mode_2(instruction: i64) -> [ParamMode; 2] {
  [
    arg_mode_immediate(instruction, 0),
    arg_mode_immediate(instruction, 1),
  ]
}

fn arg_mode_1(instruction: i64) -> [ParamMode; 1] {
  [arg_mode_immediate(instruction, 0)]
}

impl From<i64> for Opcode {
  fn from(instruction: i64) -> Self {
    match instruction % 100 {
      1 => Opcode::Add(arg_mode_3(instruction)),
      2 => Opcode::Mul(arg_mode_3(instruction)),
      3 => Opcode::Input(arg_mode_1(instruction)),
      4 => Opcode::Output(arg_mode_1(instruction)),
      5 => Opcode::JumpIfTrue(arg_mode_2(instruction)),
      6 => Opcode::JumpIfFalse(arg_mode_2(instruction)),
      7 => Opcode::LessThan(arg_mode_3(instruction)),
      8 => Opcode::Equals(arg_mode_3(instruction)),
      9 => Opcode::RelativeBase(arg_mode_1(instruction)),
      99 => Opcode::Exit,
      i => panic!("wrong opcode {}! ({})", i, instruction),
    }
  }
}
